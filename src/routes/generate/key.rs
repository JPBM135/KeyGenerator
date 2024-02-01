use crate::utils::response::{create_response, Error};
use axum::{extract::Query, response::IntoResponse};
use rand::{prelude::ThreadRng, rngs::StdRng, thread_rng, CryptoRng, Rng, RngCore, SeedableRng};
use rsa::{
  pkcs1::EncodeRsaPrivateKey, pkcs1::EncodeRsaPublicKey, pkcs1::LineEnding, RsaPrivateKey,
  RsaPublicKey,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct GetKeyResponse {
  private_key: String,
  public_key: String,
}

#[derive(Deserialize)]
pub struct GetKeyQuery {
  bits: Option<u32>,
  seed: Option<String>,
}

fn create_rng_thread(seed: Option<String>) -> impl RngCore + CryptoRng {
  match seed {
    Some(seed) => {
      let seed = seed.as_bytes();
      assert!(seed.len() == 32, "Seed must be exactly 32 bytes long");

      let user_seed: [u8; 32] = seed
        .try_into()
        .expect("Slice length doesn't match array length");

      let rng = rand::rngs::StdRng::from_seed(user_seed);
      return rng;
    }
    None => return StdRng::from_rng(thread_rng()).unwrap(),
  }
}

#[axum::debug_handler]
pub async fn get_key(Query(query): Query<GetKeyQuery>) -> impl IntoResponse {
  if query.seed.is_some() {
    let seed = query.seed.as_ref().unwrap();

    if seed.len() != 32 {
      let error = Error {
        code: "invalid_seed".to_string(),
        message: "Seed must be exactly 32 bytes long".to_string(),
      };

      return create_response::<()>(None, Some(error));
    }
  }

  let mut rng = create_rng_thread(query.seed);

  let bits: Option<usize> = match query.bits {
    Some(4096) => Some(4096),
    Some(2048) => Some(2048),
    Some(1024) => Some(1024),
    Some(_) => None,
    None => Some(2048),
  };

  if bits.is_none() {
    let error = Error {
      code: "invalid_bits".to_string(),
      message: "Bits must be 1024, 2048, or 4096".to_string(),
    };

    return create_response::<()>(None, Some(error));
  }

  let priv_key = RsaPrivateKey::new(&mut rng, bits.expect("Invalid bits"));

  if priv_key.is_err() {
    let error = Error {
      code: "key_generation_failed".to_string(),
      message: "Failed to generate key".to_string(),
    };

    return create_response::<()>(None, Some(error));
  }

  let priv_key = priv_key.unwrap();

  let pub_key = RsaPublicKey::from(&priv_key);

  let response = GetKeyResponse {
    private_key: EncodeRsaPrivateKey::to_pkcs1_pem(&priv_key, LineEnding::LF)
      .unwrap()
      .to_string(),
    public_key: EncodeRsaPublicKey::to_pkcs1_pem(&pub_key, LineEnding::LF)
      .unwrap()
      .to_string(),
  };

  return create_response(Some(&response), None);
}
