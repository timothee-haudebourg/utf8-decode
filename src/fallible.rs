use crate::Utf8Error;

/// UTF-8 decoder iterator, with fallible source.
///
/// Transforms the given [`Result<u8, E>`] iterator into a [`Result<char, E>`]
/// iterator.
///
/// ## Example
///
/// The `TryDecoder` iterator can be used, for instance, to decode UTF-8 encoded files.
///
/// ```rust
/// # use std::{fs::File, io::Read};
/// use utf8_decode::TryDecoder;
/// # fn main() -> std::io::Result<()> {
/// let file = File::open("examples/file.txt")?;
///
/// let decoder = TryDecoder::new(file.bytes());
///
/// let mut string = String::new();
/// for c in decoder {
///     string.push(c?);
/// }
/// # Ok(())
/// # }
/// ```
pub struct TryDecoder<R> {
    bytes: R,
    offset: usize,
}

impl<R> TryDecoder<R> {
    /// Creates a new `Decoder` iterator from the given [`Result<u8>`](std::io::Result) source
    /// iterator.
    pub fn new(source: R) -> TryDecoder<R> {
        TryDecoder {
            bytes: source,
            offset: 0,
        }
    }
}

impl<R, E> Iterator for TryDecoder<R>
where
    R: Iterator<Item = Result<u8, E>>,
    E: From<Utf8Error>,
{
    type Item = Result<char, E>;

    fn next(&mut self) -> Option<Result<char, E>> {
        Some(
            try_decode_iter_char(self.offset, &mut self.bytes)
                .transpose()?
                .map(|(c, len)| {
                    self.offset += len as usize;
                    c
                }),
        )
    }
}

/// Read the next Unicode character out of the given fallible byte iterator.
pub fn try_decode_iter_char<E>(
    offset: usize,
    iter: &mut impl Iterator<Item = Result<u8, E>>,
) -> Result<Option<(char, u8)>, E>
where
    E: From<Utf8Error>,
{
    match try_decode_iter_codepoint(offset, iter)? {
        Some((codepoint, len)) => match char::from_u32(codepoint) {
            Some(c) => Ok(Some((c, len))),
            None => Err(Utf8Error::new(offset, len as usize).into()),
        },
        None => Ok(None),
    }
}

/// Read the next Unicode codepoint.
///
/// - `offset` is the byte offset of the codepoint in the byte string. This will
///   be returned in any enventual `Utf8Error`.
///
/// Returns the codepoint as a `u32` and its encoded byte length.
fn try_decode_iter_codepoint<E>(
    offset: usize,
    iter: &mut impl Iterator<Item = Result<u8, E>>,
) -> Result<Option<(u32, u8)>, E>
where
    E: From<Utf8Error>,
{
    match iter.next() {
        Some(Ok(a)) => {
            let a = a as u32;
            if a & 0x80 == 0x00 {
                // 1 byte.
                Ok(Some((a, 1)))
            } else if a & 0xE0 == 0xC0 {
                // 2 bytes.
                let b = try_next_iter_byte(iter, offset, 1)?;
                Ok(Some(((a & 0x1F) << 6 | b, 2)))
            } else if a & 0xF0 == 0xE0 {
                // 3 bytes.
                let b = try_next_iter_byte(iter, offset, 1)?;
                let c = try_next_iter_byte(iter, offset, 2)?;
                Ok(Some(((a & 0x0F) << 12 | b << 6 | c, 3)))
            } else if a & 0xF8 == 0xF0 {
                // 4 bytes.
                let b = try_next_iter_byte(iter, offset, 1)?;
                let c = try_next_iter_byte(iter, offset, 2)?;
                let d = try_next_iter_byte(iter, offset, 3)?;
                Ok(Some(((a & 0x07) << 18 | b << 12 | c << 6 | d, 4)))
            } else {
                Err(Utf8Error::new(offset, 1).into())
            }
        }
        Some(Err(e)) => Err(e),
        None => Ok(None),
    }
}

/// Read the next byte of the UTF-8 character out of the given byte iterator.
///
/// - `offset` is the byte offset of the current codepoint.
/// - `len` is the number of parsed bytes of the current codepoint (excluding
///   this one).
///
/// The byte is returned as a `u32` for later shifting.
fn try_next_iter_byte<E>(
    iter: &mut impl Iterator<Item = Result<u8, E>>,
    offset: usize,
    len: usize,
) -> Result<u32, E>
where
    E: From<Utf8Error>,
{
    match iter.next() {
        Some(Ok(c)) => {
            if c & 0xC0 == 0x80 {
                Ok((c & 0x3F) as u32)
            } else {
                Err(Utf8Error::new(offset, len + 1).into())
            }
        }
        Some(Err(e)) => Err(e),
        None => Err(Utf8Error::new(offset, len).into()),
    }
}
