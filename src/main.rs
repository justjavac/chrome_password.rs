use std::env;
use std::path::PathBuf;
use tabled::builder::Builder;
use tabled::object::Columns;
use tabled::{Modify, Width};

fn main() {
  let user_profile = env::var("LOCALAPPDATA").unwrap();
  let local_state_path = PathBuf::from(&user_profile).join("Google/Chrome/User Data/Local State");
  let login_data_path = PathBuf::from(&user_profile).join("Google/Chrome/User Data/Default/Login Data");

  let master_key = chrome_password::get_master_key(&local_state_path);
  let password = chrome_password::get_password(&login_data_path, &master_key);

  print(&password);
}

fn print(password: &Vec<Vec<String>>) {
  let mut builder = Builder::default();
  builder.set_columns(["url", "username", "password"]);
  for p in password {
    builder.add_record(p);
  }
  let table = builder
    .build()
    .with(Modify::new(Columns::first()).with(Width::wrap(50).keep_words()));
  println!("{}", table);
}
