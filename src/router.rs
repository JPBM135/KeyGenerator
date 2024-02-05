use crate::routes::accounts::create::create_account_handler;
use crate::routes::generate::key::get_key;
use crate::routes::get_package::get_pkg_handler;
use axum::{
  routing::{get, post},
  Router,
};

pub fn create_router() -> Router {
  return Router::new()
    .route("/", get(get_pkg_handler))
    .route("/generate/key", get(get_key))
    .route("/accounts/create", post(create_account_handler));
}
