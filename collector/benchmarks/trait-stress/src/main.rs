use std::marker::PhantomData;

pub trait Parser<Input> {
    type Output;

    fn or<P2>(self, p: P2) -> Or<Self, P2>
    where
        Self: Sized,
        P2: Parser<Input, Output = Self::Output>,
    {
        Or(self, p)
    }
}

#[macro_export]
macro_rules! choice {
    ($first : expr) => {
        $first
    };
    ($first : expr, $($rest : expr),+) => {
        $first.or(choice!($($rest),+))
    }
}

pub struct Or<P1, P2>(P1, P2);
impl<Input, P1, P2> Parser<Input> for Or<P1, P2>
where
    P1: Parser<Input, Output = P2::Output>,
    P2: Parser<Input>,
{
    type Output = P2::Output;
}

pub struct P<I, O>(PhantomData<fn(I) -> O>);
impl<Input, O> Parser<Input> for P<Input, O> {
    type Output = O;
}

pub fn token<I>(_: u8) -> impl Parser<I, Output = u8> {
    P(PhantomData)
}

fn mcc_payload_item<I>() -> impl Parser<I, Output = u8> {
    choice!(
        token(b'G'),
        token(b'G'),
        token(b'G'),
        token(b'G'),
        token(b'G'),
        token(b'G'),
        token(b'G'),
        token(b'G'),
        token(b'G'),
        token(b'G'),
        token(b'G'),
        token(b'G'),
        token(b'G'),
        token(b'G'),
        token(b'G'),
        token(b'G')
    )
}

fn main() {}
