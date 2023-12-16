use std::fs;

///
/// To use this demo:
/// - create a 3 node network in polar (https://lightningpolar.com/) -- Alice, Bob and Carol
/// - Create channels between Alice and Carol, and between Alice and Bob.
/// - Create an invoice from Carol. Use the `decodepayreq` and `lookupinvoice` lncli commands to get preimage associated with Carol's invoice
/// - Run this program with Alice's LND credentials, and the preimage from step above. For instance:
///   `cargo run --example htlc_interceptor 127.0.0.1 10003 /Users/justin/.polar/networks/2/volumes/lnd/alice/tls.cert /Users/justin/.polar/networks/2/volumes/lnd/alice/data/chain/bitcoin/regtest/admin.macaroon <preimage>`
/// - Have Bob pay the invoice via Polar UI
/// - This program should output HTLC information. Polar should print success message.
///

#[tokio::main]
async fn main() {
    let mut args = std::env::args_os();
    args.next().expect("not even zeroth arg given");

    // Read the contents of the file into a vector of bytes
    let cert_bytes = fs::read("path/to/tlscert").expect("FailedToReadTlsCertFile");
    let mac_bytes = fs::read("path/to/macaroon").expect("FailedToReadMacaroonFile");

    // Convert the bytes to a hex string
    let cert = buffer_as_hex(cert_bytes);
    let macaroon = buffer_as_hex(mac_bytes);

    let socket = "localhost:10001".to_string();

    let preimage = args.next().expect("missing argument: preimage");

    let preimage = hex::decode(preimage.into_string().expect("preimage is invalid UTF-8")).unwrap();

    let mut client = lnd_grpc_rust::connect(cert, macaroon, socket)
        .await
        .expect("failed to connect");

    let (tx, rx) =
        tokio::sync::mpsc::channel::<lnd_grpc_rust::routerrpc::ForwardHtlcInterceptResponse>(1024);
    let stream = tokio_stream::wrappers::ReceiverStream::new(rx);

    let mut htlc_stream = client
        .router()
        .htlc_interceptor(stream)
        .await
        .expect("Failed to call htlc_interceptor")
        .into_inner();

    while let Some(htlc) = htlc_stream
        .message()
        .await
        .expect("Failed to receive htlcs")
    {
        println!("htlc {:?}", htlc);
        let response = lnd_grpc_rust::routerrpc::ForwardHtlcInterceptResponse {
            incoming_circuit_key: htlc.incoming_circuit_key,
            action: 0,                  // this will claim the htlc
            preimage: preimage.clone(), // this would be for a real preimage
            failure_code: 0,
            failure_message: vec![],
        };
        tx.send(response).await.unwrap();
    }
}

fn buffer_as_hex(bytes: Vec<u8>) -> String {
    let hex_str = bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    return hex_str;
}
