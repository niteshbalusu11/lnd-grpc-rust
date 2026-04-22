use super::*;

fn assert_clone<T: Clone>() {}
fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

#[test]
fn test_lnd_client_is_clone() {
    assert_clone::<LndClient>();
}

#[test]
fn test_lnd_node_clients_is_clone() {
    assert_clone::<LndNodeClients>();
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

#[test]
fn test_lnd_node_config_new() {
    let config = LndNodeConfig::new("alice", "cert", "macaroon", "localhost:10009");

    assert_eq!(config.alias, "alice");
    assert_eq!(config.cert, "cert");
    assert_eq!(config.macaroon, "macaroon");
    assert_eq!(config.socket, "localhost:10009");
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
async fn test_connect_returns_err_with_invalid_cert_hex() {
    let result = connect(
        "not-hex".to_string(),
        "deadbeef".to_string(),
        "localhost:10009".to_string(),
    )
    .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_my_channel_new_with_invalid_cert() {
    let uri = "https://localhost:10009".parse::<Uri>().unwrap();
    let invalid_cert = vec![0, 1, 2, 3];
    let channel = MyChannel::new(Some(invalid_cert), uri).await;
    assert!(channel.is_err());
}

#[tokio::test]
async fn test_connect_nodes_empty() {
    let clients = connect_nodes(Vec::new()).await.unwrap();

    assert!(clients.is_empty());
    assert_eq!(clients.len(), 0);
}

#[tokio::test]
async fn test_connect_nodes_rejects_empty_alias() {
    let result = connect_nodes(vec![LndNodeConfig::new(
        "",
        "cert",
        "macaroon",
        "localhost:10009",
    )])
    .await;

    assert!(matches!(result, Err(LndConnectError::EmptyAlias)));
}

#[tokio::test]
async fn test_connect_nodes_rejects_duplicate_alias() {
    let result = connect_nodes(vec![
        LndNodeConfig::new("alice", "cert", "macaroon", "localhost:10009"),
        LndNodeConfig::new("alice", "cert", "macaroon", "localhost:10010"),
    ])
    .await;

    assert!(matches!(result, Err(LndConnectError::DuplicateAlias(alias)) if alias == "alice"));
}

#[tokio::test]
async fn test_connect_nodes_error_includes_alias() {
    let result = connect_nodes(vec![LndNodeConfig::new(
        "alice",
        "not-hex",
        "deadbeef",
        "localhost:10009",
    )])
    .await;

    match result {
        Err(LndConnectError::NodeConnect { alias, .. }) => assert_eq!(alias, "alice"),
        _ => panic!("expected node connection error"),
    }
}

#[tokio::test]
async fn test_lnd_node_clients_accessors() {
    let uri = "http://localhost:10009".parse::<Uri>().unwrap();
    let channel = MyChannel::new(None, uri).await.unwrap();
    let alice = build_lnd_client(channel.clone(), "deadbeef".to_string());
    let bob = build_lnd_client(channel, "0201036c6e640224030a10".to_string());
    let mut nodes = std::collections::HashMap::new();

    nodes.insert("alice".to_string(), alice);
    nodes.insert("bob".to_string(), bob);

    let mut clients = LndNodeClients { nodes };

    assert_eq!(clients.len(), 2);
    assert!(!clients.is_empty());
    assert!(clients.contains("alice"));
    assert!(clients.get("alice").is_some());
    assert!(clients.get_mut("alice").is_some());
    assert!(clients.get_cloned("alice").is_some());
    assert!(clients.get("carol").is_none());

    let mut aliases = clients.aliases().collect::<Vec<_>>();
    aliases.sort_unstable();
    assert_eq!(aliases, vec!["alice", "bob"]);

    let mut iter_aliases = clients.iter().map(|(alias, _)| alias).collect::<Vec<_>>();
    iter_aliases.sort_unstable();
    assert_eq!(iter_aliases, vec!["alice", "bob"]);

    for (_, client) in clients.iter_mut() {
        let _ = client.lightning();
    }

    let inner = clients.into_inner();
    assert_eq!(inner.len(), 2);
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
