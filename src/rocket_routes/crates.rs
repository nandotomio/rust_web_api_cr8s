use rocket::serde::json::{Json, serde_json::json, Value};
use rocket::response::status::{Custom, NoContent};
use rocket::http::Status;

use crate::models::{NewCrate, Crate};
use crate::repositories::CrateRepository;
use crate::rocket_routes::DbConn;

use super::server_error;

#[rocket::get("/crates")]
pub async fn get_crates(db: DbConn) -> Result<Value, Custom<Value>> {
  db.run(|conn| {
    CrateRepository::find_multiple(conn, 100)
      .map(|crates| json!(crates))
      .map_err(|e| server_error(e.into()))
  }).await
}

#[rocket::get("/crates/<id>")]
pub async fn view_crate(id: i32, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::find(c, id)
            .map(|a_crate| json!(a_crate))
            .map_err(|e| match e {
                diesel::result::Error::NotFound => Custom(Status::NotFound, json!("Not found")),
                _ => server_error(e.into())
            })
    }).await
}

#[rocket::post("/crates", format = "json", data = "<new_crate>")]
pub async fn create_crate(new_crate: Json<NewCrate>, db: DbConn) -> Result<Custom<Value>, Custom<Value>> {
  db.run(move |conn| {
    CrateRepository::create(conn, new_crate.into_inner())
      .map(|a_crate| Custom(Status::Created, json!(a_crate)))
      .map_err(|e| server_error(e.into()))
  }).await
}

#[rocket::put("/crates/<id>", format = "json", data = "<a_crate>")]
pub async fn update_crate(id: i32, a_crate: Json<Crate>, db: DbConn) -> Result<Value, Custom<Value>> {
  db.run(move |conn| {
    CrateRepository::update(conn, id, a_crate.into_inner())
      .map(|a_crate| json!(a_crate))
      .map_err(|e| server_error(e.into()))
  }).await
}

#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(id: i32, db: DbConn) -> Result<NoContent, Custom<Value>> {
  db.run(move |conn| {
    CrateRepository::delete(conn, id)
      .map(|_| NoContent)
      .map_err(|e| server_error(e.into()))
  }).await
}