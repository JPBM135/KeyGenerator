use crate::database::{
  models::{Account, NewAccount},
  schema::accounts,
};
use crate::utils::generate_id;
use diesel::{PgConnection, RunQueryDsl, SelectableHelper};

pub async fn create_account(
  conn: &mut PgConnection, email: String, public_key: String, public_key_fingerprint: String,
) -> Result<Account, diesel::result::Error> {
  let new_account = NewAccount {
    id: &generate_id::generate_snowflake_id(),
    email: &email,
    public_key: &public_key.as_bytes(),
    public_key_fingerprint: &public_key_fingerprint.as_bytes(),
  };

  diesel::insert_into(accounts::table)
    .values(&new_account)
    .returning(Account::as_returning())
    .get_result(conn)
}
