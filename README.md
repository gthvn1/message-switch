# message-switch

- Rewrite the [XAPI message-switch](https://github.com/xapi-project/message-switch) in Rust.

## A in-memory message broker

- One server process requests from many clients
- Protocol is based on HTTP
- Client can:
  - *Login*: login
  - *Create*: creates a persistent queue
  - *Send*: enqueue a message in particular queue
  - *Ack*: signal that a message has been processed
    - a processed message should be deleted
  - *Subscribe*: subscribes to a named queue
  - *Transfer*: poll for new messages in subscribed queue

## How to use it?

- Start the server: `cargo run --bin server`
  - It will create and listen on a socket
- Run the client: `carg run --bin client`
  - It will send a message to the socket
