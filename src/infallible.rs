use crate::{TryDecoder, Utf8Error};

/// UTF-8 decoder iterator.
///
/// Transforms the given [`u8`] iterator into a [`Result<char, Utf8Error>`]
/// iterator.
///
/// ## Example
///
/// The `Decoder` iterator can be used, for instance, to decode `u8` slices.
///
/// ```rust
/// use utf8_decode::Decoder;
/// # fn main() -> std::io::Result<()> {
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
/// # Ok(())
/// # }
/// ```
pub struct Decoder<R>(TryDecoder<InfallibleInput<R>>);

impl<R> Decoder<R> {
    /// Creates a new `Decoder` iterator from the given [`u8`] source iterator.
    pub fn new(source: R) -> Self {
        Self(TryDecoder::new(InfallibleInput(source)))
    }
}

impl<R> Iterator for Decoder<R>
where
    R: Iterator<Item = u8>,
{
    type Item = Result<char, Utf8Error>;

    fn next(&mut self) -> Option<Result<char, Utf8Error>> {
        self.0.next()
    }
}

struct InfallibleInput<T>(T);

impl<T> Iterator for InfallibleInput<T>
where
    T: Iterator<Item = u8>,
{
    type Item = Result<u8, Utf8Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(Ok)
    }
}
