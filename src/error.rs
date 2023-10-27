use std::fmt::{self, Formatter};

use actix_web::{HttpResponse, ResponseError};

#[derive(Debug, Clone)]
pub enum BackofficeError {
    NotFound,
    IncorrectBaseUrl,
    MissingAuthorizationHeader,
    InvalidRedirectUrl,
    ClientCertificateError(CertificateError),
    TlsError,
    ServerInitError,
}

impl std::fmt::Display for BackofficeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BackofficeError::NotFound => write!(f, "Not found"),
            BackofficeError::MissingAuthorizationHeader => {
                write!(f, "Missing authorization header")
            }
            BackofficeError::IncorrectBaseUrl => {
                write!(f, "Incorrect base URL")
            }
            BackofficeError::InvalidRedirectUrl => {
                write!(f, "Incorrect redirect URL")
            }
            BackofficeError::ClientCertificateError(c) => {
                write!(f, "Incorrect Client certificate {c:?}")
            }
            BackofficeError::TlsError => {
                write!(f, "Failed to setup TLS")
            }
            BackofficeError::ServerInitError => write!(f, "Failed to start server"),
        }
    }
}

impl std::error::Error for BackofficeError {}

impl ResponseError for BackofficeError {
    fn error_response(&self) -> HttpResponse {
        match self {
            BackofficeError::NotFound => HttpResponse::NotFound().finish(),
            BackofficeError::MissingAuthorizationHeader => HttpResponse::Unauthorized().finish(),
            BackofficeError::IncorrectBaseUrl => HttpResponse::InternalServerError().finish(),
            BackofficeError::InvalidRedirectUrl => HttpResponse::InternalServerError().finish(),
            BackofficeError::ClientCertificateError(_) => {
                HttpResponse::InternalServerError().finish()
            }
            BackofficeError::TlsError => HttpResponse::InternalServerError().finish(),
            BackofficeError::ServerInitError => HttpResponse::InternalServerError().finish(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum CertificateError {
    Pkcs12ArchiveNotFound(String),
    Pkcs12IdentityGeneration(String),
    Pem8ClientKeyNotFound(String),
    Pem8ClientCertNotFound(String),
    Pem8IdentityGeneration(String),
    NoCertificateFiles,
    RootCertificatesError(String),
}
