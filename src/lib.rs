use hyper::{Request, Response, Uri};
use hyper_openssl::client::legacy::HttpsConnector;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use openssl::{
    ssl::{SslConnector, SslMethod},
    x509::X509,
};
pub use prost;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt,
    task::Poll,
};
use tonic_openssl::ALPN_H2_WIRE;
use tower::Service;

#[cfg(test)]
mod tests;

pub mod autopilotrpc {
    tonic::include_proto!("autopilotrpc");
}

pub mod chainrpc {
    tonic::include_proto!("chainrpc");
}

pub mod devrpc {
    tonic::include_proto!("devrpc");
}

pub mod invoicesrpc {
    tonic::include_proto!("invoicesrpc");
}

pub mod lnrpc {
    tonic::include_proto!("lnrpc");
}

pub mod lnclipb {
    tonic::include_proto!("lnclipb");
}

pub mod neutrinorpc {
    tonic::include_proto!("neutrinorpc");
}

pub mod peersrpc {
    tonic::include_proto!("peersrpc");
}

pub mod routerrpc {
    tonic::include_proto!("routerrpc");
}

pub mod signrpc {
    tonic::include_proto!("signrpc");
}

pub mod verrpc {
    tonic::include_proto!("verrpc");
}

pub mod walletrpc {
    tonic::include_proto!("walletrpc");
}

pub mod watchtowerrpc {
    tonic::include_proto!("watchtowerrpc");
}

pub mod wtclientrpc {
    tonic::include_proto!("wtclientrpc");
}

/// [`tonic::Status`] is re-exported as `LndClientError` for convenience.
pub type LndClientError = tonic::Status;

pub type LndAutopilotClient = crate::autopilotrpc::autopilot_client::AutopilotClient<
    tonic::codegen::InterceptedService<MyChannel, MacaroonInterceptor>,
>;

pub type LndChainClient = crate::chainrpc::chain_notifier_client::ChainNotifierClient<
    tonic::codegen::InterceptedService<MyChannel, MacaroonInterceptor>,
>;

pub type LndDevClient = crate::devrpc::dev_client::DevClient<
    tonic::codegen::InterceptedService<MyChannel, MacaroonInterceptor>,
>;

pub type LndInvoicesClient = crate::invoicesrpc::invoices_client::InvoicesClient<
    tonic::codegen::InterceptedService<MyChannel, MacaroonInterceptor>,
>;

pub type LndLightningClient = crate::lnrpc::lightning_client::LightningClient<
    tonic::codegen::InterceptedService<MyChannel, MacaroonInterceptor>,
>;

pub type LndStateClient = crate::lnrpc::state_client::StateClient<
    tonic::codegen::InterceptedService<MyChannel, MacaroonInterceptor>,
>;

pub type LndWalletUnlockerClient = crate::lnrpc::wallet_unlocker_client::WalletUnlockerClient<
    tonic::codegen::InterceptedService<MyChannel, MacaroonInterceptor>,
>;

pub type LndNeutrinoClient = crate::neutrinorpc::neutrino_kit_client::NeutrinoKitClient<
    tonic::codegen::InterceptedService<MyChannel, MacaroonInterceptor>,
>;

pub type LndPeersClient = crate::peersrpc::peers_client::PeersClient<
    tonic::codegen::InterceptedService<MyChannel, MacaroonInterceptor>,
>;

pub type LndRouterClient = crate::routerrpc::router_client::RouterClient<
    tonic::codegen::InterceptedService<MyChannel, MacaroonInterceptor>,
>;

pub type LndSignerClient = crate::signrpc::signer_client::SignerClient<
    tonic::codegen::InterceptedService<MyChannel, MacaroonInterceptor>,
>;

pub type LndVersionerClient = crate::verrpc::versioner_client::VersionerClient<
    tonic::codegen::InterceptedService<MyChannel, MacaroonInterceptor>,
>;

pub type LndWalletClient = crate::walletrpc::wallet_kit_client::WalletKitClient<
    tonic::codegen::InterceptedService<MyChannel, MacaroonInterceptor>,
>;

pub type LndWatchtowerClient = crate::watchtowerrpc::watchtower_client::WatchtowerClient<
    tonic::codegen::InterceptedService<MyChannel, MacaroonInterceptor>,
>;

pub type LndWtcClient = crate::wtclientrpc::watchtower_client_client::WatchtowerClientClient<
    tonic::codegen::InterceptedService<MyChannel, MacaroonInterceptor>,
>;

#[derive(Clone)]
pub struct LndClient {
    autopilot: LndAutopilotClient,
    chain: LndChainClient,
    dev: LndDevClient,
    invoices: LndInvoicesClient,
    lightning: LndLightningClient,
    state: LndStateClient,
    wallet_unlocker: LndWalletUnlockerClient,
    neutrino: LndNeutrinoClient,
    peers: LndPeersClient,
    router: LndRouterClient,
    signer: LndSignerClient,
    versioner: LndVersionerClient,
    wallet: LndWalletClient,
    watchtower: LndWatchtowerClient,
    wtc: LndWtcClient,
}

impl LndClient {
    pub fn autopilot(&mut self) -> &mut LndAutopilotClient {
        &mut self.autopilot
    }

    pub fn chain(&mut self) -> &mut LndChainClient {
        &mut self.chain
    }

    pub fn dev(&mut self) -> &mut LndDevClient {
        &mut self.dev
    }

    pub fn invoices(&mut self) -> &mut LndInvoicesClient {
        &mut self.invoices
    }

    pub fn lightning(&mut self) -> &mut LndLightningClient {
        &mut self.lightning
    }

    pub fn state(&mut self) -> &mut LndStateClient {
        &mut self.state
    }

    pub fn wallet_unlocker(&mut self) -> &mut LndWalletUnlockerClient {
        &mut self.wallet_unlocker
    }

    pub fn neutrino(&mut self) -> &mut LndNeutrinoClient {
        &mut self.neutrino
    }

    pub fn peers(&mut self) -> &mut LndPeersClient {
        &mut self.peers
    }

    pub fn router(&mut self) -> &mut LndRouterClient {
        &mut self.router
    }

    pub fn signer(&mut self) -> &mut LndSignerClient {
        &mut self.signer
    }

    pub fn versioner(&mut self) -> &mut LndVersionerClient {
        &mut self.versioner
    }

    pub fn wallet(&mut self) -> &mut LndWalletClient {
        &mut self.wallet
    }

    pub fn watchtower(&mut self) -> &mut LndWatchtowerClient {
        &mut self.watchtower
    }

    pub fn wtc(&mut self) -> &mut LndWtcClient {
        &mut self.wtc
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LndNodeConfig {
    pub alias: String,
    pub cert: String,
    pub macaroon: String,
    pub socket: String,
}

impl LndNodeConfig {
    pub fn new(
        alias: impl Into<String>,
        cert: impl Into<String>,
        macaroon: impl Into<String>,
        socket: impl Into<String>,
    ) -> Self {
        Self {
            alias: alias.into(),
            cert: cert.into(),
            macaroon: macaroon.into(),
            socket: socket.into(),
        }
    }
}

#[derive(Clone)]
pub struct LndNodeClients {
    nodes: HashMap<String, LndClient>,
}

impl LndNodeClients {
    pub fn get(&self, alias: &str) -> Option<&LndClient> {
        self.nodes.get(alias)
    }

    pub fn get_mut(&mut self, alias: &str) -> Option<&mut LndClient> {
        self.nodes.get_mut(alias)
    }

    pub fn get_cloned(&self, alias: &str) -> Option<LndClient> {
        self.nodes.get(alias).cloned()
    }

    pub fn contains(&self, alias: &str) -> bool {
        self.nodes.contains_key(alias)
    }

    pub fn aliases(&self) -> impl Iterator<Item = &str> {
        self.nodes.keys().map(String::as_str)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &LndClient)> {
        self.nodes
            .iter()
            .map(|(alias, client)| (alias.as_str(), client))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&str, &mut LndClient)> {
        self.nodes
            .iter_mut()
            .map(|(alias, client)| (alias.as_str(), client))
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn into_inner(self) -> HashMap<String, LndClient> {
        self.nodes
    }
}

#[derive(Debug)]
pub enum LndConnectError {
    EmptyAlias,
    DuplicateAlias(String),
    NodeConnect { alias: String, message: String },
}

impl fmt::Display for LndConnectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyAlias => write!(f, "node alias cannot be empty"),
            Self::DuplicateAlias(alias) => write!(f, "duplicate node alias: {alias}"),
            Self::NodeConnect { alias, message } => {
                write!(f, "failed to connect to node '{alias}': {message}")
            }
        }
    }
}

impl Error for LndConnectError {}

/// Supplies requests with macaroon
#[derive(Clone)]
pub struct MacaroonInterceptor {
    macaroon: String,
}

impl tonic::service::Interceptor for MacaroonInterceptor {
    fn call(
        &mut self,
        mut request: tonic::Request<()>,
    ) -> Result<tonic::Request<()>, LndClientError> {
        request.metadata_mut().insert(
            "macaroon",
            tonic::metadata::MetadataValue::try_from(&self.macaroon)
                .expect("hex produced non-ascii"),
        );
        Ok(request)
    }
}

async fn get_channel(
    cert: String,
    socket: String,
) -> Result<MyChannel, Box<dyn std::error::Error>> {
    let lnd_address = format!("https://{}", socket).to_string();
    let pem = hex::decode(cert)?;
    let uri = lnd_address.parse::<Uri>()?;
    let channel = MyChannel::new(Some(pem), uri).await?;
    Ok(channel)
}

fn build_lnd_client(channel: MyChannel, macaroon: String) -> LndClient {
    let interceptor = MacaroonInterceptor { macaroon };
    LndClient {
        autopilot: crate::autopilotrpc::autopilot_client::AutopilotClient::with_interceptor(
            channel.clone(),
            interceptor.clone(),
        ),
        chain: crate::chainrpc::chain_notifier_client::ChainNotifierClient::with_interceptor(
            channel.clone(),
            interceptor.clone(),
        ),
        dev: crate::devrpc::dev_client::DevClient::with_interceptor(
            channel.clone(),
            interceptor.clone(),
        ),
        invoices: crate::invoicesrpc::invoices_client::InvoicesClient::with_interceptor(
            channel.clone(),
            interceptor.clone(),
        ),
        lightning: crate::lnrpc::lightning_client::LightningClient::with_interceptor(
            channel.clone(),
            interceptor.clone(),
        ),
        state: crate::lnrpc::state_client::StateClient::with_interceptor(
            channel.clone(),
            interceptor.clone(),
        ),
        wallet_unlocker:
            crate::lnrpc::wallet_unlocker_client::WalletUnlockerClient::with_interceptor(
                channel.clone(),
                interceptor.clone(),
            ),
        neutrino: crate::neutrinorpc::neutrino_kit_client::NeutrinoKitClient::with_interceptor(
            channel.clone(),
            interceptor.clone(),
        ),
        peers: crate::peersrpc::peers_client::PeersClient::with_interceptor(
            channel.clone(),
            interceptor.clone(),
        ),
        router: crate::routerrpc::router_client::RouterClient::with_interceptor(
            channel.clone(),
            interceptor.clone(),
        ),
        signer: crate::signrpc::signer_client::SignerClient::with_interceptor(
            channel.clone(),
            interceptor.clone(),
        ),
        versioner: crate::verrpc::versioner_client::VersionerClient::with_interceptor(
            channel.clone(),
            interceptor.clone(),
        ),
        wallet: crate::walletrpc::wallet_kit_client::WalletKitClient::with_interceptor(
            channel.clone(),
            interceptor.clone(),
        ),
        watchtower: crate::watchtowerrpc::watchtower_client::WatchtowerClient::with_interceptor(
            channel.clone(),
            interceptor.clone(),
        ),
        wtc: crate::wtclientrpc::watchtower_client_client::WatchtowerClientClient::with_interceptor(
            channel.clone(),
            interceptor.clone(),
        ),
    }
}

pub async fn connect(
    cert: String,
    macaroon: String,
    socket: String,
) -> Result<LndClient, Box<dyn std::error::Error>> {
    let channel = get_channel(cert, socket).await?;
    Ok(build_lnd_client(channel, macaroon))
}

pub async fn connect_nodes(
    nodes: impl IntoIterator<Item = LndNodeConfig>,
) -> Result<LndNodeClients, LndConnectError> {
    let mut configs = Vec::new();
    let mut aliases = HashSet::new();

    for config in nodes {
        if config.alias.is_empty() {
            return Err(LndConnectError::EmptyAlias);
        }

        if !aliases.insert(config.alias.clone()) {
            return Err(LndConnectError::DuplicateAlias(config.alias));
        }

        configs.push(config);
    }

    let mut clients = HashMap::with_capacity(configs.len());
    for config in configs {
        let alias = config.alias;
        let client = connect(config.cert, config.macaroon, config.socket)
            .await
            .map_err(|source| LndConnectError::NodeConnect {
                alias: alias.clone(),
                message: source.to_string(),
            })?;
        clients.insert(alias, client);
    }

    Ok(LndNodeClients { nodes: clients })
}

#[derive(Clone)]
pub struct MyChannel {
    uri: Uri,
    client: MyClient,
}

#[derive(Clone)]
enum MyClient {
    ClearText(Client<HttpConnector, tonic::body::Body>),
    Tls(Client<HttpsConnector<HttpConnector>, tonic::body::Body>),
}

impl MyChannel {
    pub async fn new(certificate: Option<Vec<u8>>, uri: Uri) -> Result<Self, Box<dyn Error>> {
        let mut http = HttpConnector::new();
        http.enforce_http(false);
        let client = match certificate {
            None => MyClient::ClearText(
                Client::builder(TokioExecutor::new())
                    .http2_only(true)
                    .build(http),
            ),
            Some(pem) => {
                let ca = X509::from_pem(&pem[..])?;
                let mut connector = SslConnector::builder(SslMethod::tls())?;
                connector.cert_store_mut().add_cert(ca)?;
                connector.set_alpn_protos(ALPN_H2_WIRE)?;
                let mut https = HttpsConnector::with_connector(http, connector)?;
                https.set_callback(|c, _| {
                    c.set_verify_hostname(false);
                    Ok(())
                });
                MyClient::Tls(
                    Client::builder(TokioExecutor::new())
                        .http2_only(true)
                        .build(https),
                )
            }
        };

        Ok(Self { client, uri })
    }
}

impl Service<Request<tonic::body::Body>> for MyChannel {
    type Response = Response<tonic::body::Body>;
    type Error = hyper_util::client::legacy::Error;
    type Future = std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>,
    >;

    fn poll_ready(&mut self, _: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, mut req: Request<tonic::body::Body>) -> Self::Future {
        let uri = Uri::builder()
            .scheme(self.uri.scheme().unwrap().clone())
            .authority(self.uri.authority().unwrap().clone())
            .path_and_query(req.uri().path_and_query().unwrap().clone())
            .build()
            .unwrap();
        *req.uri_mut() = uri;
        let fut = match &self.client {
            MyClient::ClearText(client) => {
                let client = client.clone();
                client.request(req)
            }
            MyClient::Tls(client) => {
                let client = client.clone();
                client.request(req)
            }
        };
        Box::pin(async move {
            let res = fut.await?;
            let (parts, body) = res.into_parts();
            let body = tonic::body::Body::new(body);
            Ok(Response::from_parts(parts, body))
        })
    }
}
