#[macro_use]
extern crate mysql;

use mysql::Pool;

fn main() {
  match Pool::new("mysql://root@192.168.64.2:3306").unwrap().get_conn() {
    Ok(conn) => {
      println!("success")
    }
    Err(err) => {
      println!("unable to connect to mysql: {}", err);
      std::process::exit(1);
    }
  }
}
