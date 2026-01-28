use utf8_decode::{Decoder, Utf8Error};

fn main() -> Result<(), Utf8Error> {
    let bytes = [
        72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33, 32, 240, 159, 140, 141,
    ];

    let decoder = Decoder::new(bytes.iter().cloned());

    let mut string = String::new();
    for c in decoder {
        string.push(c?);
    }

    println!("{}", string);

    Ok(())
}
