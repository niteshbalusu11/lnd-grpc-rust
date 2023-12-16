use std::fs;
// This program connects to LND and prints out all incoming invoices as they settle.
// This program accepts four arguments: host, port, cert file, macaroon file

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

    let mut invoice_stream = client
        .lightning()
        .subscribe_invoices(lnd_grpc_rust::lnrpc::InvoiceSubscription {
            add_index: 0,
            settle_index: 0,
        })
        .await
        .expect("Failed to call subscribe_invoices")
        .into_inner();

    while let Some(invoice) = invoice_stream
        .message()
        .await
        .expect("Failed to receive invoices")
    {
        if let Some(state) = lnd_grpc_rust::lnrpc::invoice::InvoiceState::from_i32(invoice.state) {
            // If this invoice was Settled we can do something with it
            if state == lnd_grpc_rust::lnrpc::invoice::InvoiceState::Settled {
                println!("{:?}", invoice);
            }
        }
    }
}

fn buffer_as_hex(bytes: Vec<u8>) -> String {
    let hex_str = bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    return hex_str;
}
