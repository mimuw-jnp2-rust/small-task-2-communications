use std::collections::HashMap;

type CommsResult<T> = Result<T, CommsError>;

#[derive(Debug, PartialEq, Eq)]
enum CommsError {
    ServerLimitReached(String),
    UnexpectedHandshake(String),
    ConnectionExists(String),
    ConnectionClosed(String),
    ConnectionNotFound(String),
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum MessageType {
    Handshake,
    Post,
    GetCount,
}

impl MessageType {
    fn header(&self) -> &'static str {
        todo!()
    }
}

// There are three types of messages with the following specifications:
//
//      * Handshake - greets the new server and establishes the connection.
//                    The `load` field should be set to the ip of the client
//                    so that it can be saved by the server.
//
//      * Post      - sends whatever load to be consumed by the server.
//                    Contributes to the server's limit of received requests.
//
//      * GetCount  - Asks the server about the current number of received
//                    POST requests.
struct Message {
    msg_type: MessageType,
    load: String,
}

impl Message {
    fn content(&self) -> String {
        format!("{}\n{}", self.msg_type.header(), self.load)
    }
}

enum Connection {
    Closed,
    Open(Server),
}

struct Client {
    ip: String,
    connections: HashMap<String, Connection>,
}

impl Client {
    fn new(ip: String) -> Client {
        Client {
            ip,
            connections: HashMap::new(),
        }
    }

    // Attempts opening a new connection to the given address.
    // Method should return an error when a connection already exists.
    // The client should send a handshake to the server.
    fn open(&mut self, addr: &str, server: Server) -> CommsResult<()> {
        todo!()
    }

    // Sends the provided message to the server at the given `addr`.
    // Can only send messages through open connections. If the server
    // responds with a ServerLimitReached error, its corresponding connection
    // should be closed.
    fn send(&mut self, addr: &str, msg: Message) -> CommsResult<Response> {
        // server.receive(msg)
        todo!()
    }

    // Returns whether the connection to `addr` exists and has
    // the `Open` status.
    #[allow(dead_code)]
    fn is_open(&self, addr: &str) -> bool {
        todo!()
    }

    // Returns the number of closed connections
    #[allow(dead_code)]
    fn count_closed(&self) -> usize {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Response {
    HandshakeReceived,
    PostReceived,
    GetCount(u32),
}


#[derive(Clone)]
struct Server {
    name: String,
    post_count: u32,
    limit: u32,
    connected_client: Option<String>,
}

impl Server {
    fn new(name: String, limit: u32) -> Server {
        todo!()
    }

    // Consumes the message.
    // Server should report a ServerLimitReached error when it has received
    // a POST request above its limit..
    // Upon receiving a GET request, the server should respond
    // with the GetCount response containing the number of received POST requests.
    fn receive(&mut self, msg: Message) -> CommsResult<Response> {
        eprintln!("{} received:\n{}", self.name, msg.content());

        todo!()
    }
}

fn main() -> CommsResult<()> {
    let mut client = Client::new(String::from("10.0.0.1"));

    client.open("197.0.0.1", Server::new(String::from("TestServer"), 2))?;
    client.send(
        "197.0.0.1",
        Message {
            msg_type: MessageType::Post,
            load: String::from("Hello from the other side!"),
        },
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_headers() {
        assert_eq!(MessageType::Handshake.header(), "[HANDSHAKE]");
        assert_eq!(MessageType::Post.header(), "[POST]");
        assert_eq!(MessageType::GetCount.header(), "[GET COUNT]");
    }

    #[test]
    fn test_server_receive() -> CommsResult<()> {
        let mut server = Server::new(String::from("TestServer"), 1);
        assert_eq!(server.connected_client, None);

        // handshake
        let response = server.receive(Message {
            msg_type: MessageType::Handshake,
            load: String::from("localhost"),
        })?;
        assert_eq!(response, Response::HandshakeReceived);
        assert_eq!(server.post_count, 0);
        assert_eq!(server.connected_client, Some(String::from("localhost")));

        // another handshake should be rejected
        let result = server.receive(Message {
            msg_type: MessageType::Handshake,
            load: String::from("localhost"),
        });
        let error_msg = result.unwrap_err();
        assert_eq!(
            error_msg,
            CommsError::UnexpectedHandshake(String::from("TestServer"))
        );

        // GET
        let response = server.receive(Message {
            msg_type: MessageType::GetCount,
            load: String::new(),
        })?;
        assert_eq!(response, Response::GetCount(0));
        assert_eq!(server.post_count, 0);

        // POST
        let response = server.receive(Message {
            msg_type: MessageType::Post,
            load: String::from("The tale begins..."),
        })?;
        assert_eq!(response, Response::PostReceived);
        assert_eq!(server.post_count, 1);

        // another POST should cause a server error
        let result = server.receive(Message {
            msg_type: MessageType::Post,
            load: String::from("...and quickly ends."),
        });
        let error_msg = result.unwrap_err();
        assert_eq!(
            error_msg,
            CommsError::ServerLimitReached(String::from("TestServer"))
        );

        Ok(())
    }

    #[test]
    fn test_client_open() -> CommsResult<()> {
        let mut client = Client::new(String::from("localhost"));

        assert!(client
            .open("197.0.0.1", Server::new(String::from("TestServer"), 2))
            .is_ok());
        assert!(client.is_open("197.0.0.1"));

        let conn = client.connections.get("197.0.0.1").unwrap();
        match conn {
            &Connection::Open(ref server) => {
                assert_eq!(server.connected_client, Some("localhost".to_string()))
            }
            _ => panic!(),
        }

        // opening an already open connection should give an error
        let result = client.open("197.0.0.1", Server::new(String::from("TestServer2"), 100));
        let error_msg = result.unwrap_err();
        assert_eq!(
            error_msg,
            CommsError::ConnectionExists(String::from("197.0.0.1"))
        );

        Ok(())
    }

    #[test]
    fn test_client_send() -> CommsResult<()> {
        let mut client = Client::new(String::from("localhost"));

        client.open("197.0.0.1", Server::new(String::from("TestServer"), 1))?;

        let response = client.send(
            "197.0.0.1",
            Message {
                msg_type: MessageType::GetCount,
                load: String::new(),
            },
        )?;
        assert_eq!(response, Response::GetCount(0));

        let response = client.send(
            "197.0.0.1",
            Message {
                msg_type: MessageType::Post,
                load: String::from("Another tale"),
            },
        )?;
        assert_eq!(response, Response::PostReceived);

        // Server should have reached its limit. Another POST should halt the connection and give an error.
        let result = client.send(
            "197.0.0.1",
            Message {
                msg_type: MessageType::Post,
                load: String::from("Another abrupt end"),
            },
        );
        let error_msg = result.unwrap_err();
        assert_eq!(
            error_msg,
            CommsError::ServerLimitReached(String::from("TestServer"))
        );

        // The connection to the server should have been closed
        assert!(!client.is_open("197.0.0.1"));

        // No more messages can be sent through a halted connection.
        let result = client.send(
            "197.0.0.1",
            Message {
                msg_type: MessageType::Post,
                load: String::from("Maybe this time?"),
            },
        );
        let error_msg = result.unwrap_err();
        assert_eq!(
            error_msg,
            CommsError::ConnectionClosed(String::from("197.0.0.1"))
        );

        // Sending through a nonexistent connection should give an error
        let result = client.send(
            "10.0.0.1",
            Message {
                msg_type: MessageType::Post,
                load: String::new(),
            },
        );
        let error_msg = result.unwrap_err();
        assert_eq!(
            error_msg,
            CommsError::ConnectionNotFound(String::from("10.0.0.1"))
        );

        Ok(())
    }

    #[test]
    fn test_client_count_closed() -> CommsResult<()> {
        let to_open = [
            "197.0.0.1",
            "197.0.0.2",
            "197.0.0.3",
            "197.0.0.4",
            "197.0.0.5",
        ];
        let to_halt = ["197.0.0.1", "197.0.0.3"];

        let mut client = Client::new(String::from("localhost"));

        to_open
            .iter()
            .for_each(|&addr| client.open(addr, Server::new(addr.to_string(), 1)).unwrap());

        for addr in to_halt {
            client.send(
                addr,
                Message {
                    msg_type: MessageType::Post,
                    load: String::from("Push the limit"),
                },
            )?;
            client
                .send(
                    addr,
                    Message {
                        msg_type: MessageType::Post,
                        load: String::from("Too much"),
                    },
                )
                .expect_err("Connection should close now");
        }

        assert_eq!(client.count_closed(), 2);

        Ok(())
    }
}
