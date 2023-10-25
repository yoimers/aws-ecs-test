extern crate aws_test;

use actix_web::{App, HttpServer};
use std::io::Result;

#[actix_web::main]
async fn main() -> Result<()> {
  HttpServer::new(|| App::new().service(aws_test::users))
    .bind(("0.0.0.0", 8080))? // Docker用の設定です
    .run()
    .await
}
