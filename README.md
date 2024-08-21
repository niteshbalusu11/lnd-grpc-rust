# LND gRPC Client in Rust.

Rust implementation of LND RPC client using async gRPC library `tonic_openssl`.

## About

This crate implements LND GRPC using [`tonic_openssl`](https://docs.rs/tonic-openssl/latest/tonic_openssl/) and [`prost`](https://docs.rs/prost/).
Apart from being up-to-date at the time of writing (:D) it also allows `async` usage.
It contains vendored `*.proto` files so LND source code is not *required*
but accepts an environment variable `LND_REPO_DIR` which overrides the vendored `*.proto` files.
This can be used to test new features in non-released `lnd`.
(Actually, the motivating project using this library was that case. :))

## Usage

There's no setup needed beyond adding the crate to your `Cargo.toml`.
If you need to change the `*.proto` files from which the client is generated, set the environment variable `LND_REPO_DIR` to a directory with cloned [`lnd`](https://github.com/lightningnetwork/lnd.git) during build.

Here's an example of retrieving information from LND (`[getinfo](https://api.lightning.community/#getinfo)` call).
You can find the same example in crate root for your convenience.

Connect function takes cert and macaroon in hex format.

```rust
use std::fs;

#[tokio::main]
async fn main() {
        // Read the contents of the file into a vector of bytes
        let cert_bytes = fs::read("/path/to/tls.cert").expect("FailedToReadTlsCertFile");
        let mac_bytes = fs::read("path/to/macaroon").expect("FailedToReadMacaroonFile");
    
    
            // Convert the bytes to a hex string
        let cert = buffer_as_hex(cert_bytes);
        let macaroon = buffer_as_hex(mac_bytes);
    
        let socket = "localhost:10001".to_string();
    
        let mut client = lnd_grpc_rust::connect(cert, macaroon, socket)
        .await
        .expect("failed to connect");

    let info = client
        .lightning()
        // All calls require at least empty parameter
        .get_info(lnd_grpc_rust::lnrpc::GetInfoRequest {})
        .await
        .expect("failed to get info");

    // We only print it here, note that in real-life code you may want to call `.into_inner()` on
    // the response to get the message.
    println!("{:#?}", info);
}

fn buffer_as_hex(bytes: Vec<u8>) -> String {
    let hex_str = bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>();

    return hex_str;
}
```

## License

MIT
