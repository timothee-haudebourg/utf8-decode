extern crate utf8_decode;

use std::fs::File;
use std::io::Read;
use utf8_decode::UnsafeDecoder;

fn main() -> std::io::Result<()> {
    let file = File::open("examples/file.txt")?;

    let decoder = UnsafeDecoder::new(file.bytes());

    let mut string = String::new();
    for c in decoder {
        string.push(c?);
    }

    println!("{}", string);

    Ok(())
}
