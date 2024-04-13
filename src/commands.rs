use argon2::{
  password_hash::{
      rand_core::OsRng,
      PasswordHasher, SaltString
  },
  Argon2
};
use diesel_async::{AsyncPgConnection, AsyncConnection};

use crate::models::NewUser;
use crate::repositories::{UserRepository, RoleRepository};

async fn load_db_connection() -> AsyncPgConnection {
  let db_url = std::env::var("DATABASE_URL")
    .expect("Cannot retrieve DATABASE_URL environment variable");
  
  AsyncPgConnection::establish(&db_url).await
    .expect("Cannot establish database connection")
}

pub async fn create_users(username: String, password: String, role_codes: Vec<String>) {
  let mut c = load_db_connection().await;

  let salt = SaltString::generate(&mut OsRng);
  let argon2 = Argon2::default();
  let password_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap();
  let new_user = NewUser {
    username,
    password: password_hash.to_string(),
  };
  let user = UserRepository::create(&mut c, new_user, role_codes).await.unwrap();
  println!("User created: {:?}", user);
  let roles = RoleRepository::find_by_user(&mut c, &user).await.unwrap();
  println!("Roles assigned: {:?}", roles);
}

pub async fn list_users() {
  let mut c = load_db_connection().await;

  let users = UserRepository::find_with_roles(&mut c).await.unwrap();
  for user in users {
    println!("{:?}", user);
  }
}

pub async fn delete_user(id: i32) {
  let mut c = load_db_connection().await;

  UserRepository::delete(&mut c, id).await.unwrap();
}