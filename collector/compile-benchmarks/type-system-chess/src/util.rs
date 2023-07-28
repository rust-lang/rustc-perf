pub(crate) trait Bool {
    fn reify() -> bool;
}
pub(crate) struct True;
pub(crate) struct False;

impl Bool for True {
    fn reify() -> bool {
        true
    }
}
impl Bool for False {
    fn reify() -> bool {
        false
    }
}

pub(crate) trait RunOr<A: Bool>: Bool {
    type Output: Bool;
}
pub(crate) type Or<A, B> = <A as RunOr<B>>::Output;

impl RunOr<True> for True {
    type Output = True;
}
impl RunOr<True> for False {
    type Output = True;
}
impl RunOr<False> for True {
    type Output = True;
}
impl RunOr<False> for False {
    type Output = False;
}

pub(crate) trait RunAnd<A: Bool>: Bool {
    type Output: Bool;
}
pub(crate) type And<A, B> = <A as RunAnd<B>>::Output;

impl RunAnd<True> for True {
    type Output = True;
}
impl RunAnd<True> for False {
    type Output = False;
}
impl RunAnd<False> for True {
    type Output = False;
}
impl RunAnd<False> for False {
    type Output = False;
}

pub(crate) trait RunNot: Bool {
    type Output: Bool;
}
pub(crate) type Not<B> = <B as RunNot>::Output;

impl RunNot for True {
    type Output = False;
}
impl RunNot for False {
    type Output = True;
}

pub mod board_creator {
    #![allow(unused)]

    use crate::board_rep::{
        board::{Empty, Filled},
        color,
        piece::{self, ColoredPiece},
    };

    pub(crate) type WP = Filled<ColoredPiece<piece::Pawn, color::White>>;
    pub(crate) type WB = Filled<ColoredPiece<piece::Bishop, color::White>>;
    pub(crate) type WN = Filled<ColoredPiece<piece::Knight, color::White>>;
    pub(crate) type WR = Filled<ColoredPiece<piece::Rook, color::White>>;
    pub(crate) type WQ = Filled<ColoredPiece<piece::Queen, color::White>>;
    pub(crate) type WK = Filled<ColoredPiece<piece::King, color::White>>;
    pub(crate) type BP = Filled<ColoredPiece<piece::Pawn, color::Black>>;
    pub(crate) type BB = Filled<ColoredPiece<piece::Bishop, color::Black>>;
    pub(crate) type BN = Filled<ColoredPiece<piece::Knight, color::Black>>;
    pub(crate) type BR = Filled<ColoredPiece<piece::Rook, color::Black>>;
    pub(crate) type BQ = Filled<ColoredPiece<piece::Queen, color::Black>>;
    pub(crate) type BK = Filled<ColoredPiece<piece::King, color::Black>>;
    pub(crate) type __ = Empty;
}
