use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub mod common;

#[test]
fn test_get_rustaceans() {
  // Setup
  let client = Client::new();
  let rustacean1 = common::create_test_rustacean(&client);
  let rustacean2 = common::create_test_rustacean(&client);

  // Test
  let response = client.get(format!("{}/rustaceans", common::APP_HOST)).send().unwrap();
  assert_eq!(response.status(), StatusCode::OK);
  let json = response.json::<Value>().unwrap();
  assert!(json.as_array().unwrap().contains(&rustacean1));
  assert!(json.as_array().unwrap().contains(&rustacean2));

  // Teardown
  common::delete_test_rustacean(&client, rustacean1);
  common::delete_test_rustacean(&client, rustacean2);
}

#[test]
fn test_create_rustaceans() {
  // Setup
  let client = Client::new();

  // Test
  let response = client.post(format!("{}/rustaceans", common::APP_HOST))
    .json(&json!({
      "name": "John Doe",
      "email": "johndoe@test.com" 
    }))
    .send()
    .unwrap();
  assert_eq!(response.status(), StatusCode::CREATED);

  let rustacean: Value = response.json().unwrap();
  assert_eq!(rustacean, json!({
    "id": rustacean["id"],
    "name": "John Doe",
    "email": "johndoe@test.com",
    "created_at": rustacean["created_at"]
  }));

  // Teardown
  common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_view_rustaceans() {
  // Setup
  let client = Client::new();
  let rustacean = common::create_test_rustacean(&client);

  // Test
  let response = client.get(format!("{}/rustaceans/{}", common::APP_HOST, rustacean["id"]))
    .send()
    .unwrap();
  assert_eq!(response.status(), StatusCode::OK);

  let rustacean: Value = response.json().unwrap();
  assert_eq!(rustacean, json!({
    "id": rustacean["id"],
    "name": "John Doe",
    "email": "johndoe@test.com",
    "created_at": rustacean["created_at"]
  }));

  let response = client
    .get(format!("{}/rustaceans/99999999999", common::APP_HOST))
    .send()
    .unwrap();    
  assert_eq!(response.status(), StatusCode::NOT_FOUND);

  // Teardown
  common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_update_rustaceans() {
  // Setup
  let client = Client::new();
  let rustacean = common::create_test_rustacean(&client);

  // Test
  let response = client.put(format!("{}/rustaceans/{}", common::APP_HOST, rustacean["id"]))
    .json(&json!({
      "name": "Johnny Doe",
      "email": "johnnydoe@test.com" 
    }))
    .send()
    .unwrap();
  assert_eq!(response.status(), StatusCode::OK);

  let rustacean: Value = response.json().unwrap();
  assert_eq!(rustacean, json!({
    "id": rustacean["id"],
    "name": "Johnny Doe",
    "email": "johnnydoe@test.com",
    "created_at": rustacean["created_at"]
  }));

  // Teardown
  common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_delete_rustaceans() {
  // Setup
  let client = Client::new();
  let rustacean = common::create_test_rustacean(&client);

  // Test
  let response = client.delete(format!("{}/rustaceans/{}", common::APP_HOST, rustacean["id"]))
    .send()
    .unwrap();
  assert_eq!(response.status(), StatusCode::NO_CONTENT);
}