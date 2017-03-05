use std::fmt::Write;
pub trait Hex {
    fn to_hex(&self) -> String;
}
impl Hex for u8 {
    fn to_hex(&self) -> String {
        return format!("{:X}", *self);
    }
}
impl Hex for [u8] {
    fn to_hex(&self) -> String {
        let mut s = String::new();
        for &byte in self.iter() {
            let mut c = format!("{:X}", byte);
            if c.len() == 1 {
                c = format!("0{}", c);
            }
            write!(&mut s, "{}", c).unwrap();
        }
        return s;
    }
}
