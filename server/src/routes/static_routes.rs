use actix_files::{Files, NamedFile};
use actix_web::{web, Result};
use std::path::PathBuf;

use crate::shared::utils::errors::{ServerError, ExternalError};

async fn index_file() -> Result<NamedFile, ServerError> {
    let path: PathBuf = ["../../ErrorDashboardClient/client/dist", "index.html"].iter().collect();
    NamedFile::open(path).map_err(|err| ServerError::ExternalError(ExternalError::Io(err)))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(Files::new("/assets", "../../ErrorDashboardClient/client/dist/assets"));
    cfg.service(web::resource("/").route(web::get().to(index_file)));
    cfg.default_service(web::route().to(index_file));
}