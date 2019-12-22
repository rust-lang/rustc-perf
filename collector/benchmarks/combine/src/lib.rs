use combine::byte::hex_digit;
use combine::{choice, token};
use combine::{ParseError, Parser, RangeStream};

pub fn parse<'a, I: 'a>() -> impl Parser<Input = I, Output = u8>
where
    I: RangeStream<Item = u8, Range = &'a [u8]>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice!(
        token(b'A').map(|_| 1),
        token(b'B').map(|_| 2),
        token(b'C').map(|_| 3),
        token(b'D').map(|_| 4),
        token(b'E').map(|_| 5),
        token(b'F').map(|_| 6),
        token(b'G').map(|_| 7),
        token(b'H').map(|_| 8),
        token(b'I').map(|_| 9),
        token(b'J').map(|_| 10),
        token(b'K').map(|_| 11),
        token(b'L').map(|_| 12),
        token(b'M').map(|_| 13),
        token(b'N').map(|_| 14),
        token(b'O').map(|_| 15),
        token(b'P').map(|_| 16),
        token(b'Q').map(|_| 17),
        token(b'R').map(|_| 18),
        (hex_digit(), hex_digit()).map(|(u, l)| {
            let val = (u << 4) | l;
            val
        })
    )
    .message("while parsing")
}
