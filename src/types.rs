use crate::error::BackofficeError;
use actix_web::{web::Json, HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shadow_rs::shadow;

pub(crate) type BackofficeResult<T> = Result<T, BackofficeError>;
pub(crate) type BackofficeJsonResult<T> = Result<Json<T>, BackofficeError>;
pub(crate) type BackofficeResultWithCode<T> = Result<HttpResponse<T>, BackofficeError>;
pub(crate) type BackofficeEmptyResult = Result<HttpResponse, BackofficeError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Status {
    Ok,
    Ready,
    NotReady,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct BuildInfo {
    pub package_version: String,
    pub app_name: String,
    pub git_commit_date: DateTime<Utc>,
    pub package_major: String,
    pub package_minor: String,
    pub package_patch: String,
    pub package_version_pre: Option<String>,
    pub branch: String,
    pub tag: String,
    pub rust_version: String,
    pub rust_channel: String,
    pub short_commit_hash: String,
    pub full_commit_hash: String,
    pub build_os: String,
    pub build_target: String,
}

shadow!(build); // Get build information set to build placeholder
impl Default for BuildInfo {
    fn default() -> Self {
        BuildInfo {
            package_version: build::PKG_VERSION.into(),
            app_name: build::PROJECT_NAME.into(),
            package_major: build::PKG_VERSION_MAJOR.into(),
            package_minor: build::PKG_VERSION_MINOR.into(),
            package_patch: build::PKG_VERSION_PATCH.into(),
            package_version_pre: if build::PKG_VERSION_PRE.is_empty() {
                None
            } else {
                Some(build::PKG_VERSION_PRE.into())
            },
            branch: build::BRANCH.into(),
            tag: build::TAG.into(),
            rust_version: build::RUST_VERSION.into(),
            rust_channel: build::RUST_CHANNEL.into(),
            short_commit_hash: build::SHORT_COMMIT.into(),
            full_commit_hash: build::COMMIT_HASH.into(),
            git_commit_date: DateTime::parse_from_rfc3339(build::COMMIT_DATE_3339)
                .expect("shadow-rs did not give proper date")
                .into(),
            build_os: build::BUILD_OS.into(),
            build_target: build::BUILD_TARGET.into(),
        }
    }
}
