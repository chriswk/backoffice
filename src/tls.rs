use std::path::PathBuf;
use std::{fs, fs::File, io::BufReader};

use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

use crate::cli::TlsArgs;
use crate::error::{BackofficeError, CertificateError};
use crate::types::BackofficeResult;

pub(crate) fn build_upstream_certificate(
    upstream_certificate: Option<PathBuf>,
) -> BackofficeResult<Option<reqwest::tls::Certificate>> {
    upstream_certificate
        .map(|cert| {
            fs::read(cert)
                .map_err(|e| {
                    BackofficeError::ClientCertificateError(
                        CertificateError::RootCertificatesError(format!("{e:?}")),
                    )
                })
                .and_then(|bytes| {
                    reqwest::Certificate::from_pem(&bytes).map_err(|e| {
                        BackofficeError::ClientCertificateError(
                            CertificateError::RootCertificatesError(format!("{e:?}")),
                        )
                    })
                })
                .map(Some)
        })
        .unwrap_or(Ok(None))
}

pub fn config(tls_config: TlsArgs) -> Result<ServerConfig, BackofficeError> {
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();
    let mut cert_file = BufReader::new(
        File::open(
            tls_config
                .tls_server_cert
                .expect("No TLS server cert")
                .as_path(),
        )
        .map_err(|_| BackofficeError::TlsError)?,
    );
    let mut key_file = BufReader::new(
        File::open(tls_config.tls_server_key.expect("No server key").as_path())
            .expect("Could not read cert file"),
    );
    let cert_chain = certs(&mut cert_file)
        .expect("Could not build cert chain")
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(&mut key_file)
        .expect("Could not build pkcs8 private keys")
        .into_iter()
        .map(PrivateKey)
        .collect();
    config
        .with_single_cert(cert_chain, keys.remove(0))
        .map_err(|_e| BackofficeError::TlsError)
}
