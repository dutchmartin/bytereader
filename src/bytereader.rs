use core::fmt::Error;

pub enum Endianness {
    LittleEndian,
    BigEndian,
}
///
///
pub struct ByteReader<'a> {
    byte_array: &'a [u8],
    position: usize,
    endianness: Endianness,
}

fn reverse_byte(b: u8) -> u8 {
    let mut r = b.clone();
    r = (r & 0xF0) >> 4 | (r & 0x0F) << 4;
    r = (r & 0xCC) >> 2 | (r & 0x33) << 2;
    r = (r & 0xAA) >> 1 | (r & 0x55) << 1;
    r
}

#[allow(unused_must_use)]
impl<'a> ByteReader<'a> {
    pub fn new(byte_array: &'a [u8], endian: Endianness) -> ByteReader {
        ByteReader {
            byte_array: byte_array,
            position: 0,
            endianness: endian,
        }
    }

    #[inline]
    fn out_of_bounds(&self) -> bool {
        self.position >= self.byte_array.len()
    }



    #[inline]
    pub fn set_position(&mut self, position : usize){
        self.position = position;
    }

    #[inline]
    pub fn read_u8(&mut self) -> Result<u8, Error> {
        if self.out_of_bounds() {
            return Err(Error::default());
        }
        let value = unsafe { *self.byte_array.get_unchecked(self.position) };
        let ret: u8 = match &self.endianness {
            Endianness::LittleEndian => {
                reverse_byte(value)
            }
            Endianness::BigEndian => {
                value
            }
        };
        self.position +=1;
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use crate::bytereader::{ByteReader, Endianness};
    use core::fmt::Error;

    const FIXTURE : [u8; 9] = [1,2,3,4,5,6,7,8,9];

    #[test]
    fn test_u8_be(){
        let mut reader = ByteReader::new(&FIXTURE, Endianness::BigEndian);
        for i in 1..10{
            assert_eq!(Ok(i), reader.read_u8());
        }
        assert_eq!(Err(Error), reader.read_u8());
    }

    #[test]
    fn test_u8_le(){
        let mut reader = ByteReader::new(&FIXTURE, Endianness::LittleEndian);
        assert_eq!(Ok(128), reader.read_u8());
        assert_eq!(Ok(64), reader.read_u8());
    }
}