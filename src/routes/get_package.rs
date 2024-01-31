use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Project {
  name: String,
  description: String,
  version: String,
}

// return the project info in JSON format
pub async fn get_pkg_handler() -> String {
  let project = Project {
    name: env!("CARGO_PKG_NAME").to_string(),
    description: env!("CARGO_PKG_DESCRIPTION").to_string(),
    version: env!("CARGO_PKG_VERSION").to_string(),
  };

  return serde_json::to_string(&project).unwrap();
}
