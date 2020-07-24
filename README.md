# http_endless_header

Small attack script that tries to overload a TCP Connection by sending an infinite amount of data. If the server is compliant it should abort the connection before running out of memory.

Also supports encrypted connections. Simply change:

```rust
const SERVER: &str = "127.0.0.1:8443";
const PATH: &str = "/app/";
const ENCRYPTED: bool = true;
```

to match your setup.
