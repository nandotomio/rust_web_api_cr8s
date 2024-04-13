use crate::{repositories::UserRepository, rocket_routes::{server_error, DbConn}};
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;
use rocket::response::status::Custom;
use argon2::{PasswordHash, PasswordVerifier};

#[derive(serde::Deserialize)]
pub struct Credentials {
  pub username: String,
  pub password: String,
}

#[rocket::post("/login", format = "json", data="<credentials>")]
pub async fn login(mut db: Connection<DbConn>, credentials: Json<Credentials>) -> Result<Value, Custom<Value>> {
  UserRepository::find_by_username(&mut db, &credentials.username).await
    .map(|user| {
      let argon2 = argon2::Argon2::default();
      let db_hash = PasswordHash::new(&user.password).unwrap();
      let result = argon2.verify_password(credentials.password.as_bytes(), &db_hash);
      if result.is_ok() {
        return json!("Success")
      }
      json!("Unauthorized")
    })
    .map_err(|e| server_error(e.into()))
}