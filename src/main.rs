use actix_web::{web, App, HttpServer};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = backoffice::cli::BackofficeSettings::parse();
    let app_config = config.clone();
    let (metrics_handler, request_metrics) = backoffice::prom_metrics::instantiate(None);
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_config.clone()))
            .wrap(actix_web::middleware::Compress::default())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(request_metrics.clone())
            .service(web::scope("/auth").configure(backoffice::auth::configure_auth))
            .service(web::scope("/internal-backstage").configure(|service_cfg| {
                backoffice::internal_backstage::configure_internal_backstage(
                    service_cfg,
                    metrics_handler.clone(),
                )
            }))
    })
    .workers(config.http_args.workers.clone())
    .shutdown_timeout(5)
    .client_request_timeout(std::time::Duration::from_secs(5));
    let server = if config.http_args.clone().tls.tls_enable {
        let server_config =
            backoffice::tls::config(config.http_args.tls.clone()).expect("Failed to configure TLS");
        server
            .bind_rustls_021(config.http_args.https_server_tuple(), server_config)?
            .bind(config.http_args.http_server_tuple())
    } else {
        server.bind(config.http_args.http_server_tuple())
    }?;
    tokio::select! {
        _ = server.run() => {
            tracing::info!("Actix is shutting down");
        }
    }
    Ok(())
}
