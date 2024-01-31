use crate::routes::generate::key::get_key;
use crate::routes::get_package::get_pkg_handler;
use axum::{routing::get, Router};

pub fn create_router() -> Router {
  return Router::new()
    .route("/", get(get_pkg_handler))
    .route("/key", get(get_key));
}
