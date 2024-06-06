use std::env;

pub fn get_super_user_from_env() -> (String, String) {
  let username = env::var("SUPER_USER_USERNAME").unwrap();
  let password = env::var("SUPER_USER_PASSWORD").unwrap();

  (username, password)
}
