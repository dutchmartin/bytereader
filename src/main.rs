extern crate bytereader;
use bytereader::byte::ByteReader;
use bytereader::hex::Hex;

fn main() {
    let mut r = ByteReader::new("./bytereader.exe".to_string());
    let mut buf = vec![0u8; 5];
    r.read_next(&mut buf);
    println!("Bytes: {:?}", buf);
    println!("Hex: {}", buf.to_hex());
    r.read_at(12, &mut buf);
    println!("Bytes: {:?}", buf);
    println!("Hex: {}", buf.to_hex());
    r.read_next(&mut buf);
    println!("Bytes: {:?}", buf);
    println!("Hex: {}", buf.to_hex());
    r.read_at(3, &mut buf);
    println!("Bytes: {:?}", buf);
    println!("Hex: {}", buf.to_hex());
    let mut buf = vec![0u8; 6];
    r.read_next(&mut buf);
    println!("Bytes: {:?}", buf);
    println!("Hex: {}", buf.to_hex());
    // 16 bytes passed
    println!("16 bytes passed");
    r.jump(48);
    let mut buf = vec![0u8; 16];
    r.read_next(&mut buf);
    println!("Bytes: {:?}", buf);
    println!("Hex: {}", buf.to_hex());
    r.jump(-64);
    r.read_next(&mut buf);
    println!("Bytes: {:?}", buf);
    println!("Hex: {}", buf.to_hex());
    r.read_to_end(&mut buf);
    println!("Bytes: {:?}", buf);
    println!("Hex: {}", buf.to_hex());
    r.jump(-400);
    let mut buf = vec![0u8; 16];
    r.read_next(&mut buf);
    println!("Bytes: {:?}", buf);
    println!("Hex: {}", buf.to_hex());
}
