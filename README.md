# Async Chat Server with Tokio

This project is an asynchronous chat server implemented using the Tokio runtime in Rust. It allows multiple clients to connect, send messages, and broadcast them to all other connected clients. The server listens on a TCP socket and handles connections concurrently using async/await.

## Building and Executing the Project
1. Install Cargo and Rust if you haven't already by following these instructions: rustup.rs.
2. In the shell, run the command cargo run in the application's root directory.
3. The chat server will be bound to localhost at port 8080.

## Connecting to Server as a Client
1. Open a new shell session and run the command: telnet 127.0.0.1 8080 to connect to the server as a user.
2. Type your messages and press Enter to send them to the group chat.
3. You can open multiple shell sessions, each running telnet 127.0.0.1 8080, to simulate a group chat where all the users are you.
4. In Telnet, press Enter while the message string is empty to refresh the Telnet session. This allows you to see if any other users have sent a message to the group chat.
