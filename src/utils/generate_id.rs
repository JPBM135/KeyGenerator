use snowflake::SnowflakeIdGenerator;
use std::env;

pub fn generate_snowflake_id() -> i64 {
  let machine_id = env::var("MACHINE_ID")
    .unwrap_or(String::from("1"))
    .parse::<i32>()
    .unwrap_or(1);
  let node_id = env::var("NODE_ID")
    .unwrap_or(String::from("1"))
    .parse::<i32>()
    .unwrap_or(1);

  let mut generator = SnowflakeIdGenerator::new(machine_id, node_id);
  let id = generator.generate();
  return id;
}
