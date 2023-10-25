use actix_web::{get, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
  id: u32,
  name: String,
}

#[get("/")]
pub async fn users() -> impl Responder {
  HttpResponse::Ok().json(User {
    id: 2,
    name: "Mosu".into(),
  })
}