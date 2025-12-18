use super::*;

fn assert_clone<T: Clone>() {}
fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

#[test]
fn test_lnd_client_is_clone() {
    assert_clone::<LndClient>();
}

#[test]
fn test_my_channel_is_clone() {
    assert_clone::<MyChannel>();
}

#[test]
fn test_macaroon_interceptor_is_clone() {
    assert_clone::<MacaroonInterceptor>();
}

#[test]
fn test_my_channel_is_send() {
    assert_send::<MyChannel>();
}

#[test]
fn test_my_channel_is_sync() {
    assert_sync::<MyChannel>();
}

#[test]
fn test_macaroon_interceptor_clone() {
    let interceptor = MacaroonInterceptor {
        macaroon: "0201036c6e640224030a10".to_string(),
    };
    let cloned = interceptor.clone();
    assert_eq!(interceptor.macaroon, cloned.macaroon);
}

#[tokio::test]
async fn test_my_channel_new_cleartext() {
    let uri = "http://localhost:10009".parse::<Uri>().unwrap();
    let channel = MyChannel::new(None, uri.clone()).await;
    assert!(channel.is_ok());
    let channel = channel.unwrap();
    assert_eq!(channel.uri, uri);
}

#[tokio::test]
async fn test_my_channel_clone_preserves_uri() {
    let uri = "http://localhost:10009".parse::<Uri>().unwrap();
    let channel = MyChannel::new(None, uri).await.unwrap();
    let cloned = channel.clone();
    assert_eq!(channel.uri, cloned.uri);
}

#[tokio::test]
async fn test_my_channel_new_with_invalid_cert() {
    let uri = "https://localhost:10009".parse::<Uri>().unwrap();
    let invalid_cert = vec![0, 1, 2, 3];
    let channel = MyChannel::new(Some(invalid_cert), uri).await;
    assert!(channel.is_err());
}

#[test]
fn test_interceptor_call() {
    let mut interceptor = MacaroonInterceptor {
        macaroon: "deadbeef".to_string(),
    };
    let request = tonic::Request::new(());
    let result = tonic::service::Interceptor::call(&mut interceptor, request);
    assert!(result.is_ok());
    let request = result.unwrap();
    let macaroon_header = request.metadata().get("macaroon");
    assert!(macaroon_header.is_some());
    assert_eq!(macaroon_header.unwrap().to_str().unwrap(), "deadbeef");
}
