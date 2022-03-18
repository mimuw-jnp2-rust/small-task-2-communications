# Small Task 2 - Communications

## Description

In the second task we will attempt to model basic one client -  multiple servers communications. Our simple, made up protocol defines three types of messages: `Handshake`, `GetCount` and `Post`. In this model, a server is only capable of serving one client. A client can establish connections to multiple servers however. Upon successful reception of a message, the server should respond accordingly with a `Response`.

Your task is to implement both the client and server side behavior, according to method descriptions and tests provided in `main.rs`. Messages are sent from the client using the `.send()` method and consumed by servers using their `.receive()` method. There is no real networking in this task - `send()` should call `receive()` on an appropriate server.

Below you can find the description of our protocol:

- `Handshake` - should be sent by the client as the first request to establish a proper connection. The message should contain the ip of the client (in the `load` field). The server should respond with a `HandshakeReceived` response if the connection has been established succesfully or an `UnexpectedHandshake` error if its already connected to a client.

- `GetCount` - asks the server for its current number of consumed `Post` requests. The server should respond with a `GetCount` response containing a value equal to its `post_count` at the time of handling the request.

- `Post` - sends whatever load to the server to ingest. Consuming this request should increase the number of received `Post` request on the server by one. If the server has not reached its limit yet, it should respond with `PostReceived`. Otherwise, it should report the `ServerLimitReached` error.