use actix_web::{get, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
  id: u32,
  name: String,
  discription: String,
}

#[get("/")]
pub async fn healthcheck() -> impl Responder {
  HttpResponse::Ok()
}
#[get("/user")]
pub async fn users() -> impl Responder {
  HttpResponse::Ok().json(User {
    id: 123123123,
    name: "TEEEEEEEEEEEEEEEEEEEEST".into(),
    discription: "vvvvvvvvvvvvv".into(),
  })
}
