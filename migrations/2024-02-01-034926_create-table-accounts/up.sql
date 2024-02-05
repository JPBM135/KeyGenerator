-- Your SQL goes here
create table accounts (
  id bigint primary key,
  email varchar(255) not null unique,
  public_key bytea not null,
  public_key_fingerprint bytea not null,
  created_at timestamp not null default now()
)