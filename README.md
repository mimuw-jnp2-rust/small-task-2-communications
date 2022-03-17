# Small Task 2 - Communications

## Description

In the second task we will attempt to model basic server - client communications. Our simple, made up protocol defines three types of messages: `Handshake`, `Get` and `Post`. In this model, all messages are sent from the server to the clients and all clients have a limit of how many `Post` requests they can consume - after reaching that limit they stop responding (we call that a _halted_ state).

Your task is to implement both the server and client side behavior, according to method descriptions and tests provided in `main.rs`. Messages are sent from the server using the `.send()` method and consumed by clients using their `.receive()` method. There is no real networking in this task - `send()` should call `receive()` on an appropriate client.

## Hints

- the `format!()` macro can be used to create a `String` instance from a formatted string (akin to the `println!()` macro)

- `.as_ref()` method implemented for the `Option` type allows for borrowing of the option's contents