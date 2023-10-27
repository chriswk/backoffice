use crate::error::BackofficeError;
use crate::types::BackofficeResult;
use clap::{Args, Parser};
use reqwest::Url;
use std::path::PathBuf;
use tracing::{info, instrument};

#[derive(Args, Debug, Clone)]
pub struct TlsArgs {
    /// Should we bind TLS
    #[clap(env, long, default_value_t = false)]
    pub tls_enable: bool,
    /// Server key to use for TLS - Needs to be a path to a file
    #[clap(env, long)]
    pub tls_server_key: Option<PathBuf>,
    #[clap(env, long)]
    /// Server Cert to use for TLS - Needs to be a path to a file
    pub tls_server_cert: Option<PathBuf>,
    /// Port to listen for https connection on (will use the interfaces already defined)
    #[clap(env, long, default_value_t = 5143)]
    pub tls_server_port: u16,
}
#[derive(Args, Debug, Clone)]
pub struct HttpServerArgs {
    /// Which port should this server listen for HTTP traffic on
    #[clap(short, long, env, default_value_t = 5180)]
    pub port: u16,

    /// Which interfaces should we listen for HTTP traffic on
    #[clap(short, long, env, default_value = "0.0.0.0")]
    pub interface: String,

    /// Which base path should we mount the application under
    #[clap(short, long, env, default_value = "/")]
    pub base_path: String,

    #[clap(short, long, env, default_value_t = num_cpus::get_physical())]
    pub workers: usize,

    #[clap(flatten)]
    pub tls: TlsArgs,
}

impl HttpServerArgs {
    pub fn http_server_tuple(&self) -> (String, u16) {
        (self.interface.clone(), self.port)
    }
    pub fn https_server_tuple(&self) -> (String, u16) {
        (self.interface.clone(), self.tls.tls_server_port)
    }
}

#[derive(Clone, Debug, Parser)]
pub struct BackofficeSettings {
    /// All emails verified to belong to this domain from Googles tokeninfo will be added as admin users
    #[clap(long, env, default_value = "getunleash.io")]
    pub auto_accept_domain: String,

    /// The client id to use to authenticate against Google
    #[clap(long, env)]
    pub google_client_id: String,

    /// The Client secret to use to verify requests
    #[clap(long, env)]
    pub google_client_secret: String,

    /// The resolvable url you are hosting this application on. Defaults to http://localhost:5180
    #[clap(long, env, default_value = "http://localhost:5180", value_parser = parse_url)]
    pub backoffice_url: Url,

    #[clap(flatten)]
    pub http_args: HttpServerArgs,
}

fn parse_url(url: &str) -> Result<Url, String> {
    Url::parse(url).map_err(|p| format!("Invalid url. Our parser {p:?}"))
}

impl BackofficeSettings {
    #[instrument]
    pub fn redirect_url(&self) -> BackofficeResult<String> {
        let u = self.backoffice_url.clone();
        let f = u
            .join(&self.http_args.base_path)
            .map_err(|_| BackofficeError::InvalidRedirectUrl)?;
        let redirect = f
            .join("/auth/verify")
            .map_err(|_| BackofficeError::InvalidRedirectUrl)
            .map(|u| u.to_string());
        info!("Found to be {redirect:?}");
        redirect
    }
}
