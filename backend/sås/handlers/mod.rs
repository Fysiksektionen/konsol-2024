use actix_web::{get, post, web, HttpResponse, Result};
use serde_json::json;
use std::sync::Arc;

use crate::services::SettingsService;
use crate::models::Settings;

#[get("/settings")]
pub async fn get_settings(
    service: web::Data<Arc<SettingsService>>,
) -> Result<HttpResponse> {
    match service.get_settings().await {
        Ok(settings) => Ok(HttpResponse::Ok().json(settings)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        })))
    }
}

#[post("/settings")]
pub async fn update_settings(
    service: web::Data<Arc<SettingsService>>,
    settings: web::Json<Settings>,
) -> Result<HttpResponse> {
    match service.update_settings(settings.into_inner()).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({ "status": "success" }))),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "error": e.to_string()
        })))
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_settings)
       .service(update_settings);
}