use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use crate::error_handlers::CustomError;

#[get("/employees")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let employees = web::block(|| Employee::find_all()).await.unwrap();
    OK(HttpResponse::OK().json(employees))
}

#[get("/employees/{id}")]
async fn find(id: web::Path) -> Result<HttpResponse, CustomError> {
    let employee = Employees::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all());
}