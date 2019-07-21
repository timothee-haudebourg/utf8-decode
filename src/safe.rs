use std::io::{Result, Error, ErrorKind};
use std::convert::TryFrom;

/// Read the next byte of the UTF-8 character out of the given byte iterator.
/// The byte is returned as a `u32` for later shifting.
/// Returns an `InvalidData` error if the byte is not part of a valid UTF-8 sequence.
/// Returns an `UnexpectedEof` error if the input iterator returns `None`.
fn next_byte<I: Iterator<Item=u8>>(iter: &mut I) -> Result<u32> {
    match iter.next() {
        Some(c) => {
            if c & 0xC0 == 0x80 {
                Ok((c & 0x3F) as u32)
            } else {
                Err(Error::new(ErrorKind::InvalidData, "invalid UTF-8 sequence."))
            }
        },
        None => Err(Error::new(ErrorKind::UnexpectedEof, "unexpected end of UTF-8 sequence."))
    }
}

/// Read the next Unicode codepoint given its first byte.
/// The first input byte is given as a `u32` for later shifting.
/// Returns an `InvalidData` error the input iterator does not output a valid UTF-8 sequence.
/// Returns an `UnexpectedEof` error if the input iterator returns `None` before the end of the
/// UTF-8 character.
fn raw_decode_from<I: Iterator<Item=u8>>(a: u32, iter: &mut I) -> Result<u32> {
    if a & 0x80 == 0x00 {
        Ok(a)
    } else if a & 0xE0 == 0xC0 {
        let b = next_byte(iter)?;
        Ok((a & 0x1F) << 6 | b)
    } else if a & 0xF0 == 0xE0 {
        let b = next_byte(iter)?;
        let c = next_byte(iter)?;
        Ok((a & 0x0F) << 12 | b << 6 | c)
    } else if a & 0xF8 == 0xF0 {
        let b = next_byte(iter)?;
        let c = next_byte(iter)?;
        let d = next_byte(iter)?;
        Ok((a & 0x07) << 18 | b << 12 | c << 6 | d)
    } else {
        Err(Error::new(ErrorKind::InvalidData, "invalid UTF-8 sequence."))
    }
}

/// Read the next Unicode character given its first byte.
/// Returns an `InvalidData` error the input iterator does not output a valid UTF-8 sequence.
/// Returns an `UnexpectedEof` error if the input iterator returns `None` before the end of the
/// UTF-8 character.
fn decode_from<I: Iterator<Item=u8>>(a: u32, iter: &mut I) -> Result<char> {
    match char::try_from(raw_decode_from(a, iter)?) {
        Ok(c) => Ok(c),
        Err(_) => Err(Error::new(ErrorKind::InvalidData, "invalid UTF-8 sequence."))
    }
}

/// Read the next Unicode character out of the given [`u8`](u8) iterator.
///
/// Returns `None` is the input iterator directly outputs `None`.
/// Returns an [`InvalidData`](std::io::ErrorKind::InvalidData) error the input iterator does not
/// output a valid UTF-8 sequence.
/// Returns an [`UnexpectedEof`](std::io::ErrorKind::UnexpectedEof) error if the input iterator
/// returns `None` before the end of an UTF-8 character.
pub fn decode<I: Iterator<Item=u8>>(iter: &mut I) -> Option<Result<char>> {
	match iter.next() {
		Some(a) => Some(decode_from(a as u32, iter)),
		None => None
	}
}

/// UTF-8 decoder iterator.
///
/// Transform the given [`u8`](u8) iterator into a [`io::Result<char>`](std::io::Result) iterator.
/// This iterator cannot be used to decode an [`io::Read`](std::io::Read) source, since the input
/// iterator would be over [`io::Result<u8>`](std::io::Result) and not `u8`. However in this case
/// you can use the [`UnsafeDecoder`](crate::UnsafeDecoder) iterator.
///
/// ## Example
/// The `Decoder` iterator can be used, for instance, to decode `u8` slices.
/// ```rust
/// let bytes = [72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33];
///
/// let decoder = Decoder::new(bytes.iter().cloned());
///
/// let mut string = String::new();
/// for c in decoder {
///     string.push(c?);
/// }
///
/// println!("{}", string);
/// ```
///
/// ## Errors
/// A call to [`next`](Iterator::next) returns an [`InvalidData`](std::io::ErrorKind::InvalidData)
/// error if the input iterator does not output a valid UTF-8 sequence, or an
/// [`UnexpectedEof`](std::io::ErrorKind::UnexpectedEof) if the stream ends before the end of a
/// valid character.
pub struct Decoder<R: Iterator<Item=u8>> {
	bytes: R
}

impl<R: Iterator<Item=u8>> Decoder<R> {
    /// Creates a new `Decoder` iterator from the given `u8` source iterator.
	pub fn new(source: R) -> Decoder<R> {
		Decoder {
			bytes: source
		}
	}
}

impl<R: Iterator<Item=u8>> Iterator for Decoder<R> {
	type Item = Result<char>;

	fn next(&mut self) -> Option<Result<char>> {
		decode(&mut self.bytes)
	}
}
