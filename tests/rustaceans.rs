use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{serde_json::json, Value};

fn create_test_rustacean(client: &Client) -> Value {
  let response = client.post("http://localhost:8000/rustaceans")
    .json(&json!({
      "name": "John Doe",
      "email": "johndoe@test.com" 
    }))
    .send()
    .unwrap();
  assert_eq!(response.status(), StatusCode::CREATED);

  response.json().unwrap()
}

fn delete_test_rustacean(client: &Client, rustacean: Value) {
  let response = client.delete(format!("http://localhost:8000/rustaceans/{}", rustacean["id"]))
    .send()
    .unwrap();
  assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[test]
fn test_get_rustaceans() {
  // Setup
  let client = Client::new();
  let rustacean1 = create_test_rustacean(&client);
  let rustacean2 = create_test_rustacean(&client);

  // Test
  let response = client.get("http://localhost:8000/rustaceans").send().unwrap();
  assert_eq!(response.status(), StatusCode::OK);
  let json = response.json::<Value>().unwrap();
  assert!(json.as_array().unwrap().contains(&rustacean1));
  assert!(json.as_array().unwrap().contains(&rustacean2));

  // Teardown
  delete_test_rustacean(&client, rustacean1);
  delete_test_rustacean(&client, rustacean2);
}

#[test]
fn test_create_rustaceans() {
  // Setup
  let client = Client::new();

  // Test
  let response = client.post("http://localhost:8000/rustaceans")
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
  delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_view_rustaceans() {
  // Setup
  let client = Client::new();
  let rustacean = create_test_rustacean(&client);

  // Test
  let response = client.get(format!("http://localhost:8000/rustaceans/{}", rustacean["id"]))
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

  // Teardown
  delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_update_rustaceans() {
  // Setup
  let client = Client::new();
  let rustacean = create_test_rustacean(&client);

  // Test
  let response = client.put(format!("http://localhost:8000/rustaceans/{}", rustacean["id"]))
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
  delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_delete_rustaceans() {
  // Setup
  let client = Client::new();
  let rustacean = create_test_rustacean(&client);

  // Test
  let response = client.delete(format!("http://localhost:8000/rustaceans/{}", rustacean["id"]))
    .send()
    .unwrap();
  assert_eq!(response.status(), StatusCode::NO_CONTENT);
}