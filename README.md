# UTF-8 decode

This crates provides incremental UTF-8 decoders implementing the `Iterator` trait.
Thoses iterators are wrappers around `u8` bytes iterators.

## Decoder

The `Decoder` struct wraps `Iterator<Item = u8>` iterators.
You can use it, for instance, to decode `u8` slices.

```rust
extern crate utf8_decode;

use utf8_decode::Decoder;

fn main() -> std::io::Result<()> {
    let bytes = [72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33, 32, 240, 159, 140, 141];

    let decoder = Decoder::new(bytes.iter().cloned());

    let mut string = String::new();
    for c in decoder {
        string.push(c?);
    }

    println!("{}", string);

    Ok(())
}
```

## UnsafeDecoder

The `UnsafeDecoder` wraps `Iterator<Item = std::io::Result<u8>>` iterators.
You can use it, for instance, to decode UTF-8 encoded files.

```rust
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
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
