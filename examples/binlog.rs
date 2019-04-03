extern crate mysql;

use mysql::{Pool};

fn main() {
  match Pool::new("mysql://root@192.168.64.2:3306").unwrap().get_conn() {
    Ok(mut conn) => {
      println!("success");
      let result = conn.as_mut().start_binlog_sync().unwrap();
      loop {
        std::thread::sleep(std::time::Duration::from_millis(10));
      }
      println!("wierouweoiruw");
    }
    Err(err) => {
      println!("unable to connect to mysql: {}", err);
      std::process::exit(1);
    }
  }
}
