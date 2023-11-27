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

  let new_user = NewUser {
    username,
    password
  };
  let user = UserRepository::create(&mut c, new_user, role_codes).await.unwrap();
  println!("User created: {:?}", user);
  let roles = RoleRepository::find_by_user(&mut c, &user).await.unwrap();
  println!("Roles assigned: {:?}", roles);
}
