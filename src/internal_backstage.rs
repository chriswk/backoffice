use crate::metrics::actix_web_metrics::PrometheusMetricsHandler;
use crate::types::Status;
use crate::types::{BackofficeJsonResult, BuildInfo};
use actix_web::{
    get,
    web::{self, Json},
};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct BackofficeStatus {
    pub status: Status,
}

impl BackofficeStatus {
    pub fn ok() -> Self {
        BackofficeStatus { status: Status::Ok }
    }
    pub fn not_ready() -> Self {
        BackofficeStatus {
            status: Status::NotReady,
        }
    }

    pub fn ready() -> Self {
        BackofficeStatus {
            status: Status::Ready,
        }
    }
}

#[get("/health")]
async fn health() -> BackofficeJsonResult<BackofficeStatus> {
    Ok(Json(BackofficeStatus::ok()))
}

#[get("/info")]
async fn info() -> BackofficeJsonResult<BuildInfo> {
    let data = BuildInfo::default();
    Ok(Json(data))
}

#[get("/ready")]
async fn ready() -> BackofficeJsonResult<BackofficeStatus> {
    Ok(Json(BackofficeStatus::ready()))
}

pub fn configure_internal_backstage(
    cfg: &mut web::ServiceConfig,
    metrics_handler: PrometheusMetricsHandler,
) {
    cfg.service(health)
        .service(info)
        .service(ready)
        .service(web::resource("/metrics").route(web::get().to(metrics_handler)));
}
