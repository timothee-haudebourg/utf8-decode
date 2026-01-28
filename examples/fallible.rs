use std::{fs::File, io::Read};
use utf8_decode::TryDecoder;

fn main() -> std::io::Result<()> {
    let file = File::open("examples/file.txt")?;

    let decoder = TryDecoder::new(file.bytes());

    let mut string = String::new();
    for c in decoder {
        string.push(c?);
    }

    println!("{}", string);

    Ok(())
}
