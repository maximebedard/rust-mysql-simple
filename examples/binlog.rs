extern crate mysql;

use mysql::{Pool};
use std::io::{Cursor, Result};
use byteorder::{LittleEndian, BigEndian, ReadBytesExt};
// use std::iter::Iterator;
// use std::io::Read;

fn main() {
  match Pool::new("mysql://root@192.168.64.2:3306").unwrap().get_conn() {
    Ok(conn) => {
      println!("success");
      let mut reader = conn.unwrap().binlog_reader().unwrap();
      loop {
        match reader() {
          Ok(data) => {
            let event = Event::try_from(data);
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

struct Header {
  timestamp: i32,
  type_code: i8,
  server_id: i32,
  event_length: i32,
  next_position: i32,
  flags: i16
}

struct Data {

}

struct Event {
  header: Header,
  data: Data,
}

impl Event {
  fn try_from(buffer: Vec<u8>) -> Result<Event> {
    let mut cursor = Cursor::new(buffer);

    // skip magic bytes;
    // std::io::copy(&mut cursor.take(4), &mut std::io::sink());

    let timestamp = cursor.read_i32::<LittleEndian>()?;
    let type_code = cursor.read_i8()?;
    let server_id = cursor.read_i32::<LittleEndian>()?;
    let event_length = cursor.read_i32::<LittleEndian>()?;
    let next_position = cursor.read_i32::<LittleEndian>()?;
    let flags = cursor.read_i16::<LittleEndian>()?;

    let event = Event {
      header: Header {
        timestamp,
        type_code,
        server_id,
        event_length,
        next_position,
        flags,
      },
      data: Data {
        binlog_version: 0,
      },
    };

    Ok(event)
  }
}
