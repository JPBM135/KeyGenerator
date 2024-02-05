use sha2::{Digest, Sha512};

pub fn get_key_fingerprint(public_key: &str) -> String {
  let hash = Sha512::digest(public_key.as_bytes());

  let result_str = format!("{:02x}", hash);

  return result_str;
}
