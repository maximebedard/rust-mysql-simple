extern crate mysql;

use mysql::{Pool, parse_event};

use std::io::Write;

fn main() {
  match Pool::new("mysql://root@192.168.64.2:3306").unwrap().get_conn() {
    Ok(conn) => {
      // println!("success");
      let mut reader = conn.unwrap().binlog_reader().unwrap();
      loop {
        match reader() {
          Ok(data) => {
            parse_event(&data.clone()[1..]);
          }
          Err(err) => println!("error reading from stream {:?}", err),
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
      }
    }
    Err(err) => {
      println!("unable to connect to mysql: {}", err);
      std::process::exit(1);
    }
  }
}
