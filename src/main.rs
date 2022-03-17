use std::collections::HashMap;

type CommsResult<T> = Result<T, String>;

#[derive(Clone, Copy)]
enum MessageType {
    Handshake,
    Post,
    Get
}

impl MessageType {
    fn header(&self) -> &'static str {
        todo!()
    }
}

// There are three types of messages with the following specifications:
//
//      * Handshake - greets the new client and establishes the connection.
//                    The `load` field should be set to the ip of the server
//                    so that it can be saved by the client.
//
//      * Post      - sends whatever load to be consumed by the client.
//                    Contributes to the client's limit of received requests.
//
//      * Get       - Asks the client about the current number of received
//                    POST requests. The client responds to that message.
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
    Halted(Client),
    Open(Client),
}

struct Server {
    ip: String,
    connections: HashMap<String, Connection>,
}

impl Server {
    fn new(ip: String) -> Server {
        Server { ip, connections: HashMap::new() }
    }

    // Attempts opening a new connection to the given address.
    // Method should return an error when a connection already exists.
    // The server should send a handshake to the client.
    fn open(&mut self, addr: &str, client: Client) -> CommsResult<()> {
        todo!()
    }

    fn close_all(&mut self) {
        self.connections.values_mut().for_each(|conn| *conn = Connection::Closed)
    }

    // Sends the provided message to the client at the given `addr`.
    // Can only send messages through open connections. If the client
    // responds with any error, its corresponding connection should
    // be marked as halted.
    fn send(&mut self, addr: &str, msg: Message) -> CommsResult<Option<String>> {
        // client.receive(msg)
        todo!()
    }

    // Returns whether the connection to `addr` exists and has
    // the `Open` status.
    fn is_open(&self, addr: &str) -> bool {
        todo!()
    }

    // Returns the number of connections with the `Halted` status.
    fn count_halted(&self) -> usize {
        todo!()
    }
}

#[derive(Clone)]
struct Client {
    name: String,
    post_count: u32,
    limit: u32,
    connected_server: Option<String>,
}

impl Client {
    fn new(name: String, limit: u32) -> Client {
        todo!()
    }

    // Consumes the message.
    // Client should start reporting errors when it has received
    // a number of POST requests equal to its limit.
    // Upon receiving a GET request, the client should respond
    // with a string containing the number of received POST requests.
    fn receive(&mut self, msg: Message) -> CommsResult<Option<String>> {
        eprintln!("{} received:\n{}", self.name, msg.content());

        todo!()
    }
}

fn main() -> CommsResult<()> {
    let mut server = Server::new(String::from("10.0.0.1"));

    server.open("197.0.0.1", Client::new(String::from("TestClient"), 2))?;
    server.send("197.0.0.1", Message { msg_type: MessageType::Post, load: String::from("Hello from the other side!") })?;

    server.close_all();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_headers() {
        assert_eq!(MessageType::Handshake.header(), "[HANDSHAKE]");
        assert_eq!(MessageType::Post.header(), "[POST]");
        assert_eq!(MessageType::Get.header(), "[GET]");
    }

    #[test]
    fn test_client_receive() -> CommsResult<()> {
        let mut client = Client::new(String::from("TestClient"), 1);
        assert!(matches!(client.connected_server, None));

        // handshake
        let response = client.receive(Message { msg_type: MessageType::Handshake, load: String::from("localhost") })?;
        assert!(matches!(response, None));
        assert_eq!(client.post_count, 0);
        assert_eq!(*client.connected_server.as_ref().unwrap(), String::from("localhost"));

        // another handshake should be rejected
        let result = client.receive(Message { msg_type: MessageType::Handshake, load: String::from("localhost") });
        let error_msg = result.unwrap_err();
        assert_eq!(error_msg, String::from("Client 'TestClient' received unexpected handshake. Already connected to 'localhost'."));

        // GET
        let response = client.receive(Message { msg_type: MessageType::Get, load: String::new() })?;
        assert_eq!(response.unwrap(), String::from("0"));
        assert_eq!(client.post_count, 0);

        // POST
        let response = client.receive(Message { msg_type: MessageType::Post, load: String::from("The tale begins...") })?;
        assert!(matches!(response, None));
        assert_eq!(client.post_count, 1);

        // another POST should cause a client error
        let result = client.receive(Message { msg_type: MessageType::Post, load: String::from("...and quickly ends.") });
        let error_msg = result.unwrap_err();
        assert_eq!(error_msg, String::from("Client 'TestClient' cannot ingest more messages."));

        Ok(())
    }

    #[test]
    fn test_server_open() -> CommsResult<()> {
        let mut server = Server::new(String::from("localhost"));

        assert!(server.open("197.0.0.1", Client::new(String::from("TestClient"), 2)).is_ok());
        assert!(server.is_open("197.0.0.1"));

        let conn = server.connections.get("197.0.0.1").unwrap();
        match conn {
            &Connection::Open(ref client) => assert_eq!(client.connected_server, Some("localhost".to_string())),
            _ => return Err("Connection should be open".to_string())
        }

        // opening an already open connection should give an error
        let result = server.open("197.0.0.1", Client::new(String::from("TestClient2"), 100));
        let error_msg = result.unwrap_err();
        assert_eq!(error_msg, String::from("Cannot open connection to '197.0.0.1'. Connection already exists."));

        Ok(())
    }

    #[test]
    fn test_server_send() -> CommsResult<()> {
        let mut server = Server::new(String::from("localhost"));

        server.open("197.0.0.1", Client::new(String::from("TestClient"), 1))?;

        let response = server.send("197.0.0.1", Message { msg_type: MessageType::Get, load: String::new() })?;
        assert_eq!(response.unwrap(), String::from("0"));

        let response = server.send("197.0.0.1", Message { msg_type: MessageType::Post, load: String::from("Another tale") })?;
        assert!(matches!(response, None));

        // Client should have reached its limit. Another POST should halt the connection and give an error.
        let result = server.send("197.0.0.1", Message { msg_type: MessageType::Post, load: String::from("Another abrupt end") });
        let error_msg = result.unwrap_err();
        assert_eq!(error_msg, String::from("Client 'TestClient' cannot ingest more messages."));

        // No more messages can be sent through a halted connection.
        let result = server.send("197.0.0.1", Message { msg_type: MessageType::Post, load: String::from("Maybe this time?") });
        let error_msg = result.unwrap_err();
        assert_eq!(error_msg, String::from("Tried to send a message through a halted connection ('197.0.0.1')."));

        Ok(())
    }

    #[test]
    fn test_server_count_halted() -> CommsResult<()> {
        let to_open = ["197.0.0.1", "197.0.0.2", "197.0.0.3", "197.0.0.4", "197.0.0.5"];
        let to_halt = ["197.0.0.1", "197.0.0.3"];

        let mut server = Server::new(String::from("localhost"));

        to_open.iter().for_each(|&addr| server.open(addr, Client::new(addr.to_string(), 1)).unwrap());

        for addr in to_halt {
            server.send(addr, Message { msg_type: MessageType::Post, load: String::from("Push the limit") })?;
            server.send(addr, Message { msg_type: MessageType::Post, load: String::from("Too much") }).expect_err("Client should halt now");
        }

        assert_eq!(server.count_halted(), 2);

        Ok(())
    }
}