use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub mod common;

#[test]
fn test_get_crates() {
  // Setup
  let client = Client::new();
  let rustacean = common::create_test_rustacean(&client);
  let a_crate1 = common::create_test_crate(&client, &rustacean);
  let a_crate2 = common::create_test_crate(&client, &rustacean);

  // Test
  let response = client.get(format!("{}/crates", common::APP_HOST)).send().unwrap();
  assert_eq!(response.status(), StatusCode::OK);
  let json = response.json::<Value>().unwrap();
  assert!(json.as_array().unwrap().contains(&a_crate1));
  assert!(json.as_array().unwrap().contains(&a_crate2));

  // Teardown
  common::delete_test_crate(&client, a_crate1);
  common::delete_test_crate(&client, a_crate2);
  common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_create_create() {
  let client = Client::new();
  let rustacean = common::create_test_rustacean(&client);

  let response = client.post(format!("{}/crates", common::APP_HOST))
    .json(&json!({
      "rustacean_id": rustacean["id"],
      "code": "foo",
      "name": "Foo crate",
      "version": "0.1",
      "description": "Foo crate description",
    }))
    .send()
    .unwrap();
  assert_eq!(response.status(), StatusCode::CREATED);

  let a_crate: Value = response.json().unwrap();
  assert_eq!(a_crate, json!({
    "id": a_crate["id"],
    "rustacean_id": rustacean["id"],
    "code": "foo",
    "name": "Foo crate",
    "version": "0.1",
    "description": "Foo crate description",
    "created_at": a_crate["created_at"]
  }));

  // Teardown
  common::delete_test_crate(&client, a_crate);
  common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_view_create() {
  let client = Client::new();
  let rustacean = common::create_test_rustacean(&client);
  let a_crate = common::create_test_crate(&client, &rustacean);

  let response = client.get(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
    .send()
    .unwrap();
  assert_eq!(response.status(), StatusCode::OK);

  let a_crate: Value = response.json().unwrap();
  assert_eq!(a_crate, json!({
    "id": a_crate["id"],
    "rustacean_id": rustacean["id"],
    "code": "foo",
    "name": "Foo crate",
    "version": "0.1",
    "description": "Foo crate description",
    "created_at": a_crate["created_at"]
  }));

  let response = client
    .get(format!("{}/crates/99999999999", common::APP_HOST))
    .send()
    .unwrap();
  assert_eq!(response.status(), StatusCode::NOT_FOUND);

  // Teardown
  common::delete_test_crate(&client, a_crate);
  common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_update_create() {
  let client = Client::new();
  let rustacean = common::create_test_rustacean(&client);
  let a_crate = common::create_test_crate(&client, &rustacean);

  let response = client.put(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
    .json(&json!({
      "rustacean_id": rustacean["id"],
      "code": "fooz",
      "name": "Fooz crate",
      "version": "0.2",
      "description": "Fooz crate description",
    }))
    .send()
    .unwrap();
  assert_eq!(response.status(), StatusCode::OK);

  let a_crate: Value = response.json().unwrap();
  assert_eq!(a_crate, json!({
    "id": a_crate["id"],
    "rustacean_id": rustacean["id"],
    "code": "fooz",
    "name": "Fooz crate",
    "version": "0.2",
    "description": "Fooz crate description",
    "created_at": a_crate["created_at"]
  }));

  // Test author-switching and a really long description
  let rustacean2 = common::create_test_rustacean(&client);
  let response = client.put(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
    .json(&json!({
      "rustacean_id": rustacean2["id"],
      "code": "fooz",
      "name": "Fooz crate",
      "version": "0.2",
      "description": "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
    }))
    .send()
    .unwrap();
  assert_eq!(response.status(), StatusCode::OK);
  let a_crate: Value = response.json().unwrap();
  assert_eq!(a_crate, json!({
    "id": a_crate["id"],
    "rustacean_id": rustacean2["id"],
    "code": "fooz",
    "name": "Fooz crate",
    "version": "0.2",
    "description": "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
    "created_at": a_crate["created_at"]
  }));

  // Teardown
  common::delete_test_crate(&client, a_crate);
  common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_delete_create() {
  let client = Client::new();
  let rustacean = common::create_test_rustacean(&client);
  let a_crate = common::create_test_crate(&client, &rustacean);

  let response = client.delete(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
    .send()
    .unwrap();
  assert_eq!(response.status(), StatusCode::NO_CONTENT);

  // Teardown
  common::delete_test_rustacean(&client, rustacean);
}