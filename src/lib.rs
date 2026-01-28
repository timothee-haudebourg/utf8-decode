//! This crates provides incremental UTF-8 decoders implementing the [`Iterator`] trait.
//! Thoses iterators are wrappers around [`u8`] bytes iterators.
//!
//! [`Iterator`]: std::iter::Iterator
//!
//! ## `Decoder`
//!
//! The [`Decoder`] iterator can be used, for instance, to decode `u8` slices.
//!
//! ```rust
//! use utf8_decode::Decoder;
//! # fn main() -> std::io::Result<()> {
//! let bytes = [72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33];
//!
//! let decoder = Decoder::new(bytes.iter().cloned());
//!
//! let mut string = String::new();
//! for c in decoder {
//!     string.push(c?);
//! }
//!
//! println!("{}", string);
//! # Ok(())
//! # }
//! ```
//!
//! ## `TryDecoder`
//!
//! The [`TryDecoder`] iterator can be used, for instance, to decode UTF-8
//! encoded files.
//!
//! ```rust
//! # use std::{fs::File, io::Read};
//! use utf8_decode::TryDecoder;
//! # fn main() -> std::io::Result<()> {
//! let file = File::open("examples/file.txt")?;
//!
//! let decoder = TryDecoder::new(file.bytes());
//!
//! let mut string = String::new();
//! for c in decoder {
//!     string.push(c?);
//! }
//! # Ok(())
//! # }
//! ```
#![cfg_attr(not(feature = "std"), no_std)]
use core::fmt::{self, Debug, Display, Formatter};

mod fallible;
mod infallible;

pub use fallible::{TryDecoder, try_decode_iter_char};
pub use infallible::Decoder;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Utf8Error {
    pub offset: usize,
    pub len: usize,
}

impl Utf8Error {
    pub const fn new(offset: usize, len: usize) -> Self {
        Self { offset, len }
    }
}

impl Display for Utf8Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "invalid UTF-8 sequence")
    }
}

impl Debug for Utf8Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "invalid UTF-8 sequence")
    }
}

impl core::error::Error for Utf8Error {}

#[cfg(feature = "std")]
impl From<Utf8Error> for std::io::Error {
    fn from(value: Utf8Error) -> Self {
        Self::new(std::io::ErrorKind::InvalidData, value)
    }
}

/// Read the UTF-8 encoded character out of the given slice at position `i`.
///
/// Returns the character and its encoded byte length, moving the `i` value to
/// point to the start of the next character (or end of string).
pub const fn try_decode_char(bytes: &[u8], i: &mut usize) -> Result<Option<(char, u8)>, Utf8Error> {
    let offset = *i;
    match try_decode_codepoint(bytes, offset, i) {
        Ok(Some((codepoint, len))) => match char::from_u32(codepoint) {
            Some(c) => Ok(Some((c, len))),
            None => Err(Utf8Error::new(offset, len as usize)),
        },
        Ok(None) => Ok(None),
        Err(e) => Err(e),
    }
}

/// Read the next Unicode codepoint.
///
/// - `offset` is the byte offset of the codepoint in the byte string. This will
///   be returned in any enventual `Utf8Error`.
///
/// Returns the codepoint as a `u32` and its encoded byte length.
const fn try_decode_codepoint(
    bytes: &[u8],
    offset: usize,
    i: &mut usize,
) -> Result<Option<(u32, u8)>, Utf8Error> {
    if *i < bytes.len() {
        let a = bytes[*i] as u32;

        *i += 1;

        if a & 0x80 == 0x00 {
            // 1 byte.
            Ok(Some((a, 1)))
        } else if a & 0xE0 == 0xC0 {
            // 2 bytes.
            match try_next_slice_byte(bytes, offset, i) {
                Ok(b) => Ok(Some(((a & 0x1F) << 6 | b, 2))),
                Err(e) => Err(e),
            }
        } else if a & 0xF0 == 0xE0 {
            // 3 bytes.
            match try_next_slice_byte(bytes, offset, i) {
                Ok(b) => match try_next_slice_byte(bytes, offset, i) {
                    Ok(c) => Ok(Some(((a & 0x0F) << 12 | b << 6 | c, 3))),
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            }
        } else if a & 0xF8 == 0xF0 {
            // 4 bytes.
            match try_next_slice_byte(bytes, offset, i) {
                Ok(b) => match try_next_slice_byte(bytes, offset, i) {
                    Ok(c) => match try_next_slice_byte(bytes, offset, i) {
                        Ok(d) => Ok(Some(((a & 0x07) << 18 | b << 12 | c << 6 | d, 4))),
                        Err(e) => Err(e),
                    },
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            }
        } else {
            Err(Utf8Error::new(offset, 1))
        }
    } else {
        Ok(None)
    }
}

/// Read the next byte of the UTF-8 character out of the given slice.
///
/// - `offset` is the byte offset of the current codepoint.
///
/// The byte is returned as a `u32` for later shifting.
const fn try_next_slice_byte(bytes: &[u8], offset: usize, i: &mut usize) -> Result<u32, Utf8Error> {
    if *i < bytes.len() {
        let c = bytes[*i];

        *i += 1;

        if c & 0xC0 == 0x80 {
            Ok((c & 0x3F) as u32)
        } else {
            Err(Utf8Error::new(offset, *i - offset))
        }
    } else {
        Err(Utf8Error::new(offset, *i - offset))
    }
}
