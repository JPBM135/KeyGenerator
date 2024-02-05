use crate::constants::EMAIL_REGEX;
use crate::services::{accounts::create::create_account, database::connect::establish_connection};
use crate::utils::get_key_fingerprint::get_key_fingerprint;
use crate::utils::response::create_response;
use axum::{extract::Json, response::IntoResponse};
use rsa::pkcs8::DecodePublicKey;
use rsa::RsaPublicKey;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateAccountBody {
  email: String,
  public_key: String,
}

#[derive(Serialize, Deserialize)]
struct CreateAccountResponse {
  id: i64,
  email: String,
  public_key: String,
  public_key_fingerprint: String,
  created_at: String,
}

#[axum::debug_handler]
pub async fn create_account_handler(Json(body): Json<CreateAccountBody>) -> impl IntoResponse {
  let conn = establish_connection();

  if conn.is_err() {
    println!("Failed to connect to the database {:?}", conn.err());

    let error = crate::utils::response::Error {
      code: "database_error".to_string(),
      message: "Failed to connect to the database".to_string(),
    };

    return create_response::<()>(None, Some(error));
  }

  let validation_result = validate_account(&body.email, &body.public_key);

  if validation_result.is_err() {
    return create_response::<()>(None, Some(validation_result.err().unwrap()));
  }

  let decoded_public_key = RsaPublicKey::from_public_key_pem(&body.public_key);

  if decoded_public_key.is_err() {
    let error = crate::utils::response::Error {
      code: "invalid_public_key".to_string(),
      message: "Invalid public key".to_string(),
    };

    return create_response::<()>(None, Some(error));
  }

  let fingerprint = get_key_fingerprint(&body.public_key);

  let mut conn = conn.unwrap();

  let account = create_account(&mut conn, body.email, body.public_key, fingerprint).await;

  match account {
    Ok(account) => {
      let response_body = CreateAccountResponse {
        id: account.id,
        email: account.email,
        public_key: Vec::from(account.public_key)
          .iter()
          .map(|b| format!("{:02x}", b))
          .collect::<String>(),
        public_key_fingerprint: Vec::from(account.public_key_fingerprint)
          .iter()
          .map(|b| format!("{:02x}", b))
          .collect::<String>(),
        created_at: account.created_at.to_string(),
      };

      return create_response(Some(response_body), None);
    }
    Err(err) => {
      println!("Failed to create account: {:?}", err);

      let error = crate::utils::response::Error {
        code: "database_error".to_string(),
        message: "Failed to create account".to_string(),
      };

      return create_response::<()>(None, Some(error));
    }
  }
}

fn validate_account(email: &str, public_key: &str) -> Result<(), crate::utils::response::Error> {
  if email.is_empty() || public_key.is_empty() {
    return Err(crate::utils::response::Error {
      code: "invalid_body".to_string(),
      message: "Email and public_key are required".to_string(),
    });
  }

  if email.len() > 512 {
    return Err(crate::utils::response::Error {
      code: "invalid_email".to_string(),
      message: "The email must be less than 512 characters".to_string(),
    });
  }

  // if email.matches(EMAIL_REGEX).count() != 1 {
  //   return Err(crate::utils::response::Error {
  //     code: "invalid_email".to_string(),
  //     message: "Invalid email".to_string(),
  //   });
  // }

  Ok(())
}
