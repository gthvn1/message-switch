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
