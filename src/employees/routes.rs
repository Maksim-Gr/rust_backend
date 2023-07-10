use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use crate::employees::Employee;
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

#[post("/employees")]
async fn create(employee: web::Json) -> Result<HttpResponse, CustomError> {
    let employee = Employee::create(employee.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[put("/employee{id}")]
async fn update(
    id: web::Path,
    employee: web::Json,
) -> Result<HttpResponse, CustomError> {
    let employee = Employee::update(id.into_inner(), employee.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[delete("employee{id}")]
async fn delete(id: web::Path) -> Result<HttpResponse, CustomError> {
    let deleted_employee = Employee::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({"deleted": deleted_employee})))
}

pub fn init_routes(config: &mut web::ServiceConfig) {

    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);

}