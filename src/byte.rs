use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::io::Read;
use std::fs;
use std::fs::Metadata;

pub struct ByteReader {
    file: File,
    position: u64,
    pub meta: Metadata,
}

#[allow(unused_must_use)]
impl ByteReader {
    pub fn new(filename: String) -> ByteReader {
        ByteReader {
            file: File::open(&filename).unwrap(),
            position: 0,
            meta: fs::metadata(&filename).unwrap(),
        }
    }
    pub fn read_next(&mut self, mut buf: &mut Vec<u8>) {
        self.position += buf.len() as u64;
        self.file.read(&mut buf);
    }
    pub fn read_at(&mut self, position: u64, mut buf: &mut Vec<u8>) {
        self.file.seek(SeekFrom::Start(position - 1));
        self.file.read(&mut buf);
        let pos;
        if position > self.position {
            pos = -1 * (position - self.position + buf.len() as u64 - 1) as i64;
        } else {
            pos = (self.position - position - buf.len() as u64 + 1) as i64;
        }
        self.file.seek(SeekFrom::Current(pos));
    }
    pub fn jump(&mut self, amount: i64) {
        let pos = self.position as i64 + amount;
        self.position = pos as u64;
        self.file.seek(SeekFrom::Current(amount));
    }
    pub fn read_to_end(&mut self, mut buf: &mut Vec<u8>) {
        self.file.read_to_end(&mut buf);
        self.position = self.meta.len() - 1;
    }
}
