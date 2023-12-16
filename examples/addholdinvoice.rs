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

    let add_hold_invoice_resp = client
        .invoices()
        .add_hold_invoice(lnd_grpc_rust::invoicesrpc::AddHoldInvoiceRequest {
            hash: vec![0; 32],
            value: 5555,
            ..Default::default()
        })
        .await
        .expect("failed to add hold invoice");

    // We only print it here, note that in real-life code you may want to call `.into_inner()` on
    // the response to get the message.
    println!("{:#?}", add_hold_invoice_resp);
}

fn buffer_as_hex(bytes: Vec<u8>) -> String {
    let hex_str = bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    return hex_str;
}
