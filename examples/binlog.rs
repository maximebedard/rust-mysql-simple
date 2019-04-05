extern crate mysql;

use mysql::{Pool};
use std::io::Cursor;
use byteorder::{LittleEndian, BigEndian, ReadBytesExt};

fn main() {
  match Pool::new("mysql://root@192.168.64.2:3306").unwrap().get_conn() {
    Ok(conn) => {
      println!("success");
      let mut reader = conn.unwrap().binlog_reader().unwrap();
      loop {
        match reader() {
          Ok(data) => {
            // println!("{:?}\r\n", packet);
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

impl Parser {
    pub fn new(reader: T) -> Parser<T> {
        Parser{
            reader: reader,
            field_types: Vec::with_capacity(100)
        }
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.reader
    }

    fn skip(&mut self, n: u64) -> Result<u64> {
        std::io::copy(&mut self.reader.by_ref().take(n), &mut std::io::sink())
    }

    pub fn read_event_header(&mut self) -> Result<EventHeader> {
        let mut data = [0; 19];
        self.reader.read(&mut data)?;

        let mut cursor = Cursor::new(&data);

        let timestamp = cursor.read_i32::<LittleEndian>()?;
        let type_code = cursor.read_i8()?;
        let server_id = cursor.read_i32::<LittleEndian>()?;
        let event_len = cursor.read_i32::<LittleEndian>()?;
        let next_pos = cursor.read_i32::<LittleEndian>()?;
        let flags = cursor.read_i16::<LittleEndian>()?;

        Ok(EventHeader::new(
            timestamp,
            type_code,
            server_id,
            event_len,
            next_pos,
            flags
        ))
    }
  }
}
