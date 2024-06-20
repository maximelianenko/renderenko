
use actix_multipart::{form::MultipartFormConfig, MultipartError};

use actix_web::{error::Error, App, HttpRequest, HttpServer};

use renderenko_api::post::render::render_config;

fn error_handler(err:MultipartError, req: &HttpRequest) -> Error {
    println!("Error: {:?}\n Request: {:?}",err,req);
    Error::from(err)
    // Error::from(RenderenkoError {name: "something"})
}
#[actix_web::main] 
async fn main() -> std::io::Result<()> {
    // let config:MultipartFormConfig = MultipartFormConfig::default();
    // config.error_handler(error_handler);
    HttpServer::new(|| {
        App::new()
            .configure(render_config)
            .app_data(MultipartFormConfig::default().error_handler(error_handler))
            // .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}