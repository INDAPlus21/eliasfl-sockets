# eliasfl-sockets

Command to start client: `cargo run --bin client -- 127.0.0.1` <- 127.0.0.1 can be changed to the ip address of the server

Command to start server: `cargo run --bin server`

The server should be run on a Raspberry Pi zero and client commands `on` and `off` should toggle the power LED.

Bytes transferer from client to server have the following format:
[content_length: u8, command: u8]
Where command is `1` for on and `2` for off
