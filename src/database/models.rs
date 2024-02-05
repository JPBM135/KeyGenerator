use super::schema::accounts;
use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Account {
  pub id: i64,
  pub email: String,
  pub public_key: Vec<u8>,
  pub public_key_fingerprint: Vec<u8>,
  pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = accounts)]
pub struct NewAccount<'a> {
  pub id: &'a i64,
  pub email: &'a str,
  pub public_key: &'a [u8],
  pub public_key_fingerprint: &'a [u8],
}
