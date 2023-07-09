use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/employees")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let employees = web::block(|| Employee::find_all()).await.unwrap();
    OK(HttpResponse::OK().json(employees))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all());
}