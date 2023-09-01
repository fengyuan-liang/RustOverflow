use actix_web::{App, HttpServer, middleware};
use pkg::xlog::env_logger::init_logger;
use routers::router::*;


#[path = "../routers/mod.rs"]
mod routers;

#[path = "../lib/mod.rs"]
mod lib;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    init_logger();

    let app = move || {
        App::new()
            // 日志中间件
            .wrap(middleware::Logger::default())
            .configure(general_routes)
    };
    HttpServer::new(app).bind("0.0.0.0:3000")?.run().await
}
