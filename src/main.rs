mod constants;
mod database;
mod router;
mod routes;
mod services;
mod utils;
use dotenvy::dotenv;
use router::create_router;

#[tokio::main]
async fn main() {
  dotenv().ok();

  let app = create_router();

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  println!("Listening on http://localhost:3000");
  axum::serve(listener, app).await.unwrap();
}
