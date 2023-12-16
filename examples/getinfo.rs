// This example only fetches and prints the node info to the standard output similarly to
// `lncli getinfo`.
//
// This program accepts four arguments: host, port, cert file, macaroon file
use std::fs;

#[tokio::main]
async fn main() {
    // Read the contents of the file into a vector of bytes
    let cert_bytes = fs::read("path/to/tlscert").expect("FailedToReadTlsCertFile");
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
    let hex_str = bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    return hex_str;
}
