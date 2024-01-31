use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::{self, json, Value};

#[derive(Serialize, Deserialize)]
pub struct Error {
  pub code: String,
  pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct Response<T: Serialize> {
  data: Option<T>,
  error: Option<Error>,
}

pub fn create_response<T: Serialize>(data: Option<T>, error: Option<Error>) -> Json<Value> {
  let response = Response { data, error };

  return Json(json!(response));
}
