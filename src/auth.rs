use actix_web::{get, http, web, HttpResponse};
use reqwest::Url;
use tracing::{info, instrument};

use crate::cli::BackofficeSettings;
use crate::error::BackofficeError;
use crate::types::BackofficeEmptyResult;

const AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
#[instrument(skip(cli))]
#[get("/login")]
async fn login(cli: web::Data<BackofficeSettings>) -> BackofficeEmptyResult {
    let url = Url::parse_with_params(
        AUTH_URL,
        &[
            ("response_type", "code"),
            ("client_id", &cli.google_client_id.clone()),
            ("redirect_uri", &cli.redirect_url()?),
            ("scope", "openid profile email"),
        ],
    )
    .map_err(|_| BackofficeError::IncorrectBaseUrl)?;
    info!("Data {}", url.to_string());
    Ok(HttpResponse::Found()
        .append_header((http::header::LOCATION, url.as_str()))
        .finish())
}

pub fn configure_auth(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
}
