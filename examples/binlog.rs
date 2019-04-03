extern crate mysql;

use mysql::{Pool};

fn main() {
  match Pool::new("mysql://root@192.168.64.2:3306").unwrap().get_conn() {
    Ok(mut conn) => {
      println!("success");
      let cccc = conn.as_mut();
      let result = cccc.start_binlog_sync().unwrap();
      loop {
        // cccc.ping();
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
