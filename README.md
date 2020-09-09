# msg-rpc
An rpc server implementation using msgpack

## Usage

Add the crate to the dependencies

```toml
[dependencies]
msgrpc = {git = "https://github.com/flotte-goes-smart/msg-rpc/tree/main"}
```


```rust
pub fn main() {
    let mut server = RPCServer::new("127.0.0.1:".to_string());
    let mut receiver = Arc::clone(&server.receiver);
    thread::spawn(move || {
        server.start();        
    });
    for handler in receiver {
        // handle the message and return a response
    }
}
```