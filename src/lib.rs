/// This is part of public interface so it's re-exported.
pub extern crate tonic;

pub use error::ConnectError;
use error::InternalConnectError;
use http_body::combinators::UnsyncBoxBody;
use hyper::client::HttpConnector;
use hyper::Uri;
use hyper_rustls::HttpsConnector;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tonic::codegen::{Bytes, InterceptedService};
use tonic::Status;

type Service = InterceptedService<
    hyper::Client<HttpsConnector<HttpConnector>, UnsyncBoxBody<Bytes, Status>>,
    MacaroonInterceptor,
>;

/// Convenience type alias for autopilot client.
#[cfg(feature = "autopilotrpc")]
pub type AutopilotClient = autopilotrpc::autopilot_client::AutopilotClient<Service>;

/// Convenience type alias for chain kit client.
#[cfg(feature = "chainrpc")]
pub type ChainKitClient = chainrpc::chain_kit_client::ChainKitClient<Service>;

/// Convenience type alias for chain notifier client.
#[cfg(feature = "chainrpc")]
pub type ChainNotifierClient = chainrpc::chain_notifier_client::ChainNotifierClient<Service>;

/// Convenience type alias for dev client.
#[cfg(feature = "devrpc")]
pub type DevClient = devrpc::dev_client::DevClient<Service>;

/// Convenience type alias for lightning client.
#[cfg(feature = "lightningrpc")]
pub type LightningClient = lnrpc::lightning_client::LightningClient<Service>;

/// Convenience type alias for wallet client.
#[cfg(feature = "walletrpc")]
pub type WalletKitClient = walletrpc::wallet_kit_client::WalletKitClient<Service>;

/// Convenience type alias for peers service client.
#[cfg(feature = "peersrpc")]
pub type PeersClient = peersrpc::peers_client::PeersClient<Service>;

/// Convenience type alias for signer client.
#[cfg(feature = "signrpc")]
pub type SignerClient = signrpc::signer_client::SignerClient<Service>;

/// Convenience type alias for router client.
#[cfg(feature = "routerrpc")]
pub type RouterClient = routerrpc::router_client::RouterClient<Service>;

/// Convenience type alias for invoices client.
#[cfg(feature = "invoicesrpc")]
pub type InvoicesClient = invoicesrpc::invoices_client::InvoicesClient<Service>;

/// Convenience type alias for versioner service client.
#[cfg(feature = "versionrpc")]
pub type VersionerClient = verrpc::versioner_client::VersionerClient<Service>;

/// Convenience type alias for watchtower service client.
#[cfg(feature = "watchtowerrpc")]
pub type WatchtowerClient = watchtowerrpc::watchtower_client::WatchtowerClient<Service>;

/// Convenience type alias for watchtower service client.
#[cfg(feature = "wtclientrpc")]
pub type WatchtowerClientClient =
    wtclientrpc::watchtower_client_client::WatchtowerClientClient<Service>;

/// The client returned by `connect` function
///
/// This is a convenience type which you most likely want to use instead of raw client.
#[derive(Clone)]
pub struct Client {
    #[cfg(feature = "autopilotrpc")]
    autopilot: AutopilotClient,
    #[cfg(feature = "chainrpc")]
    chain_kit: ChainKitClient,
    #[cfg(feature = "chainrpc")]
    chain_notifier: ChainNotifierClient,
    #[cfg(feature = "devrpc")]
    dev: DevClient,
    #[cfg(feature = "lightningrpc")]
    lightning: LightningClient,
    #[cfg(feature = "walletrpc")]
    wallet: WalletKitClient,
    #[cfg(feature = "signrpc")]
    signer: SignerClient,
    #[cfg(feature = "peersrpc")]
    peers: PeersClient,
    #[cfg(feature = "versionrpc")]
    version: VersionerClient,
    #[cfg(feature = "routerrpc")]
    router: RouterClient,
    #[cfg(feature = "invoicesrpc")]
    invoices: InvoicesClient,
    #[cfg(feature = "watchtowerrpc")]
    watchtower: WatchtowerClient,
    #[cfg(feature = "wtclientrpc")]
    wtclient: WatchtowerClientClient,
}

impl Client {
    /// Returns the autopilot client.
    #[cfg(feature = "autopilotrpc")]
    pub fn autopilot(&mut self) -> &mut AutopilotClient {
        &mut self.autopilot
    }

    /// Returns the chainkit client.
    #[cfg(feature = "chainrpc")]
    pub fn chain_kit(&mut self) -> &mut ChainKitClient {
        &mut self.chain_kit
    }

    /// Returns the chain notifier client.
    #[cfg(feature = "chainrpc")]
    pub fn chain_notifier(&mut self) -> &mut ChainNotifierClient {
        &mut self.chain_notifier
    }

    /// Returns the dev client.
    #[cfg(feature = "devrpc")]
    pub fn dev(&mut self) -> &mut DevClient {
        &mut self.dev
    }

    /// Returns the lightning client.
    #[cfg(feature = "lightningrpc")]
    pub fn lightning(&mut self) -> &mut LightningClient {
        &mut self.lightning
    }

    /// Returns the wallet client.
    #[cfg(feature = "walletrpc")]
    pub fn wallet(&mut self) -> &mut WalletKitClient {
        &mut self.wallet
    }

    /// Returns the signer client.
    #[cfg(feature = "signrpc")]
    pub fn signer(&mut self) -> &mut SignerClient {
        &mut self.signer
    }

    /// Returns the versioner client.
    #[cfg(feature = "versionrpc")]
    pub fn versioner(&mut self) -> &mut VersionerClient {
        &mut self.version
    }

    /// Returns the peers client.
    #[cfg(feature = "peersrpc")]
    pub fn peers(&mut self) -> &mut PeersClient {
        &mut self.peers
    }

    /// Returns the router client.
    #[cfg(feature = "routerrpc")]
    pub fn router(&mut self) -> &mut RouterClient {
        &mut self.router
    }

    /// Returns the invoices client.
    #[cfg(feature = "invoicesrpc")]
    pub fn invoices(&mut self) -> &mut InvoicesClient {
        &mut self.invoices
    }

    /// Returns the watch tower client.
    #[cfg(feature = "watchtowerrpc")]
    pub fn watchtowerrpc(&mut self) -> &mut WatchtowerClient {
        &mut self.watchtower
    }

    /// Returns the wt client.
    #[cfg(feature = "wtclientrpc")]
    pub fn wtclientrpc(&mut self) -> &mut WatchtowerClientClient {
        &mut self.wtclient
    }
}

/// [`tonic::Status`] is re-exported as `Error` for convenience.
pub type Error = tonic::Status;

mod error;

macro_rules! try_map_err {
    ($result:expr, $mapfn:expr) => {
        match $result {
            Ok(value) => value,
            Err(error) => return Err($mapfn(error).into()),
        }
    };
}

#[cfg(feature = "autopilotrpc")]
pub mod autopilotrpc {
    tonic::include_proto!("autopilotrpc");
}

#[cfg(feature = "chainrpc")]
pub mod chainrpc {
    tonic::include_proto!("chainrpc");
}

#[cfg(feature = "devrpc")]
pub mod devrpc {
    tonic::include_proto!("devrpc");
}

#[cfg(feature = "invoicesrpc")]
pub mod invoicesrpc {
    tonic::include_proto!("invoicesrpc");
}

/// Messages and other types generated by `tonic`/`prost`
///
/// This is the go-to module you will need to look in to find documentation on various message
/// types. However it may be better to start from methods on the [`LightningClient`](lnrpc::lightning_client::LightningClient) type.
#[cfg(feature = "lightningrpc")]
pub mod lnrpc {
    tonic::include_proto!("lnrpc");
}

#[cfg(feature = "walletrpc")]
pub mod walletrpc {
    tonic::include_proto!("walletrpc");
}

#[cfg(feature = "signrpc")]
pub mod signrpc {
    tonic::include_proto!("signrpc");
}

#[cfg(feature = "peersrpc")]
pub mod peersrpc {
    tonic::include_proto!("peersrpc");
}

#[cfg(feature = "routerrpc")]
pub mod routerrpc {
    tonic::include_proto!("routerrpc");
}

#[cfg(feature = "versionrpc")]
pub mod verrpc {
    tonic::include_proto!("verrpc");
}

#[cfg(feature = "watchtowerrpc")]
pub mod watchtowerrpc {
    tonic::include_proto!("watchtowerrpc");
}

#[cfg(feature = "wtclientrpc")]
pub mod wtclientrpc {
    tonic::include_proto!("wtclientrpc");
}

/// Supplies requests with macaroon
#[derive(Clone)]
pub struct MacaroonInterceptor {
    macaroon: String,
}

impl tonic::service::Interceptor for MacaroonInterceptor {
    fn call(&mut self, mut request: tonic::Request<()>) -> Result<tonic::Request<()>, Error> {
        request.metadata_mut().insert(
            "macaroon",
            tonic::metadata::MetadataValue::from_str(&self.macaroon)
                .expect("hex produced non-ascii"),
        );
        Ok(request)
    }
}

async fn load_macaroon(
    path: impl AsRef<Path> + Into<PathBuf>,
) -> Result<String, InternalConnectError> {
    let macaroon =
        tokio::fs::read(&path)
            .await
            .map_err(|error| InternalConnectError::ReadFile {
                file: path.into(),
                error,
            })?;
    Ok(hex::encode(macaroon))
}

/// Connects to LND using given address and credentials
///
/// This function does all required processing of the cert file and macaroon file, so that you
/// don't have to. The address must begin with "https://", though.
///
/// This is considered the recommended way to connect to LND. An alternative function to use
/// already-read certificate or macaroon data is currently **not** provided to discourage such use.
/// LND occasionally changes that data which would lead to errors and in turn in worse application.
///
/// If you have a motivating use case for use of direct data feel free to open an issue and
/// explain.
pub async fn connect<CP, MP>(
    address: String,
    cert_file: CP,
    macaroon_file: MP,
) -> Result<Client, ConnectError>
where
    CP: AsRef<Path> + Into<PathBuf> + std::fmt::Debug,
    MP: AsRef<Path> + Into<PathBuf> + std::fmt::Debug,
{
    let connector = hyper_rustls::HttpsConnectorBuilder::new()
        .with_tls_config(tls::config(cert_file).await?)
        .https_or_http()
        .enable_http2()
        .build();
    let macaroon = load_macaroon(macaroon_file).await?;

    let svc = InterceptedService::new(
        hyper::Client::builder().build(connector),
        MacaroonInterceptor { macaroon },
    );
    let uri =
        Uri::from_str(address.as_str()).map_err(|error| InternalConnectError::InvalidAddress {
            address,
            error: Box::new(error),
        })?;

    let client = Client {
        #[cfg(feature = "autopilotrpc")]
        autopilot: autopilotrpc::autopilot_client::AutopilotClient::with_origin(
            svc.clone(),
            uri.clone(),
        ),
        #[cfg(feature = "chainrpc")]
        chain_kit: chainrpc::chain_kit_client::ChainKitClient::with_origin(
            svc.clone(),
            uri.clone(),
        ),
        #[cfg(feature = "chainrpc")]
        chain_notifier: chainrpc::chain_notifier_client::ChainNotifierClient::with_origin(
            svc.clone(),
            uri.clone(),
        ),
        #[cfg(feature = "devrpc")]
        dev: devrpc::dev_client::DevClient::with_origin(svc.clone(), uri.clone()),
        #[cfg(feature = "lightningrpc")]
        lightning: lnrpc::lightning_client::LightningClient::with_origin(svc.clone(), uri.clone()),
        #[cfg(feature = "walletrpc")]
        wallet: walletrpc::wallet_kit_client::WalletKitClient::with_origin(
            svc.clone(),
            uri.clone(),
        ),
        #[cfg(feature = "peersrpc")]
        peers: peersrpc::peers_client::PeersClient::with_origin(svc.clone(), uri.clone()),
        #[cfg(feature = "signrpc")]
        signer: signrpc::signer_client::SignerClient::with_origin(svc.clone(), uri.clone()),
        #[cfg(feature = "versionrpc")]
        version: verrpc::versioner_client::VersionerClient::with_origin(svc.clone(), uri.clone()),
        #[cfg(feature = "routerrpc")]
        router: routerrpc::router_client::RouterClient::with_origin(svc.clone(), uri.clone()),
        #[cfg(feature = "invoicesrpc")]
        invoices: invoicesrpc::invoices_client::InvoicesClient::with_origin(
            svc.clone(),
            uri.clone(),
        ),
        #[cfg(feature = "watchtowerrpc")]
        watchtower: watchtowerrpc::watchtower_client::WatchtowerClient::with_origin(
            svc.clone(),
            uri.clone(),
        ),
        #[cfg(feature = "wtclientrpc")]
        wtclient: wtclientrpc::watchtower_client_client::WatchtowerClientClient::with_origin(
            svc.clone(),
            uri.clone(),
        ),
    };
    Ok(client)
}

mod tls {
    use crate::error::{ConnectError, InternalConnectError};
    use rustls::{
        client::{ClientConfig, ServerCertVerified, ServerCertVerifier},
        Certificate, Error as TLSError, ServerName,
    };
    use std::{
        path::{Path, PathBuf},
        sync::Arc,
        time::SystemTime,
    };

    pub(crate) async fn config(
        path: impl AsRef<Path> + Into<PathBuf>,
    ) -> Result<ClientConfig, ConnectError> {
        Ok(ClientConfig::builder()
            .with_safe_defaults()
            .with_custom_certificate_verifier(Arc::new(CertVerifier::load(path).await?))
            .with_no_client_auth())
    }

    pub(crate) struct CertVerifier {
        certs: Vec<Vec<u8>>,
    }

    impl CertVerifier {
        pub(crate) async fn load(
            path: impl AsRef<Path> + Into<PathBuf>,
        ) -> Result<Self, InternalConnectError> {
            let contents = try_map_err!(tokio::fs::read(&path).await, |error| {
                InternalConnectError::ReadFile {
                    file: path.into(),
                    error,
                }
            });
            let mut reader = &*contents;

            let certs = try_map_err!(rustls_pemfile::certs(&mut reader), |error| {
                InternalConnectError::ParseCert {
                    file: path.into(),
                    error,
                }
            });

            Ok(CertVerifier { certs })
        }
    }

    impl ServerCertVerifier for CertVerifier {
        fn verify_server_cert(
            &self,
            end_entity: &Certificate,
            intermediates: &[Certificate],
            _server_name: &ServerName,
            _scts: &mut dyn Iterator<Item = &[u8]>,
            _ocsp_response: &[u8],
            _now: SystemTime,
        ) -> Result<ServerCertVerified, TLSError> {
            let mut certs = intermediates
                .iter()
                .map(|c| c.0.clone())
                .collect::<Vec<Vec<u8>>>();
            certs.push(end_entity.0.clone());
            certs.sort();

            let mut our_certs = self.certs.clone();
            our_certs.sort();

            if self.certs.len() != certs.len() {
                return Err(TLSError::General(format!(
                    "Mismatched number of certificates (Expected: {}, Presented: {})",
                    self.certs.len(),
                    certs.len()
                )));
            }
            for (c, p) in our_certs.iter().zip(certs.iter()) {
                if *p != *c {
                    return Err(TLSError::General(
                        "Server certificates do not match ours".to_string(),
                    ));
                }
            }
            Ok(ServerCertVerified::assertion())
        }
    }
}
