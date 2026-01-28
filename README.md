# UTF-8 decode

[![CI](https://github.com/timothee-haudebourg/utf8-decode/workflows/CI/badge.svg)](https://github.com/timothee-haudebourg/utf8-decode/actions)
[![Crate informations](https://img.shields.io/crates/v/utf8-decode.svg?style=flat-square)](https://crates.io/crates/utf8-decode)
[![License](https://img.shields.io/crates/l/utf8-decode.svg?style=flat-square)](https://github.com/timothee-haudebourg/utf8-decode#license)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/utf8-decode)

<!-- cargo-rdme start -->

This crates provides incremental UTF-8 decoders implementing the
[`Iterator`] trait, wrapping around [`u8`] bytes iterators.

It also provide the `const`-compatible [`try_decode_char`] to decode UTF-8
byte streams, even in `const` contexts.

[`u8`]: std::primitive::u8
[`Iterator`]: std::iter::Iterator
[`try_decode_char`]: https://docs.rs/utf8-decode/latest/utf8_decode/fn.try_decode_char.html

### `Decoder`

The [`Decoder`] iterator can be used, for instance, to decode `u8` slices.

```rust
use utf8_decode::Decoder;
let bytes = [72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33];

let decoder = Decoder::new(bytes.iter().cloned());

let mut string = String::new();
for c in decoder {
    string.push(c?);
}

println!("{}", string);
```

### `TryDecoder`

The [`TryDecoder`] iterator can be used, for instance, to decode UTF-8
encoded files.

```rust
use utf8_decode::TryDecoder;
let file = File::open("examples/file.txt")?;

let decoder = TryDecoder::new(file.bytes());

let mut string = String::new();
for c in decoder {
    string.push(c?);
}
```

[`TryDecoder`]: https://docs.rs/utf8-decode/latest/utf8_decode/fallible/struct.TryDecoder.html

<!-- cargo-rdme end -->

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
