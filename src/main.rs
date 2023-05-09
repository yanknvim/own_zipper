use std::io::SeekFrom;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::process::exit;

use inflate::inflate_bytes;

fn main() {
    let zip_file = File::open("foo.zip").unwrap();
    let mut reader = BufReader::new(zip_file);

    let local_file_signature: [u8; 4] = [0x50, 0x4b, 0x03, 0x04];
    let central_directory_signature: [u8; 4] = [0x50, 0x4b, 0x01, 0x02];

    loop {
        let mut buf = [0u8; 4];
        reader.read(&mut buf).unwrap();
        match buf {
            [0x50, 0x4b, 0x03, 0x04] => (),
            [0x50, 0x4b, 0x01, 0x02] => exit(0),
            _ => panic!("Err: invalid signature"),
        }

        reader.seek(SeekFrom::Current(4)).unwrap();

        let mut buf = [0u8; 2];
        reader.read(&mut buf).unwrap();
        let comp_method = u16::from_le_bytes(buf);

        reader.seek(SeekFrom::Current(8)).unwrap();

        let mut buf = [0u8; 4];
        reader.read(&mut buf).unwrap();
        let comp_size = u32::from_le_bytes(buf);

        let mut buf = [0u8; 4];
        reader.read(&mut buf).unwrap();
        let uncomp_size = u32::from_le_bytes(buf);

        let mut buf = [0u8; 2];
        reader.read(&mut buf).unwrap();
        let file_name_length = u16::from_le_bytes(buf) as usize;

        let mut buf = [0u8; 2];
        reader.read(&mut buf).unwrap();
        let extra_field_length = u16::from_le_bytes(buf) as usize;

        let mut buf = vec![0u8; file_name_length];
        reader.read_exact(&mut buf).unwrap();
        let file_name = String::from_utf8(buf).unwrap();
        
        let mut buf = vec![0u8; extra_field_length];
        reader.read_exact(&mut buf).unwrap();
        let extra_field = buf;

        println!("{}", file_name);

        let mut buf = vec![0u8; comp_size as usize];
        reader.read(&mut buf).unwrap();
        let file_content = if comp_method == 8 {
            inflate_bytes(&buf).unwrap()
        } else {
            buf
        };

        println!("{}", String::from_utf8(file_content).unwrap());
    }
    
}
