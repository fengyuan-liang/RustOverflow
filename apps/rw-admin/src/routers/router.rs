use crate::lib::fab::fab;
use actix_web::{get, web, Responder};

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
    cfg.service(hello);
    cfg.service(fab1);
}

#[get("/")]
async fn index() -> impl Responder {
    "Hello, rustOverflow!"
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[get("/fab/{n}")]
async fn fab1(param: web::Path<i32>) -> impl Responder {
    let n = param.into_inner();
    format!("fab: {}!", fab(n))
}
