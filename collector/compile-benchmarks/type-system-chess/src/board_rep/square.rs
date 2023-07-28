pub mod file;
pub mod offset;
pub mod rank;
pub mod set;

use self::{
    file::{FileEn, FileEq, RunFileEq},
    rank::{RankEn, RankEq, RunRankEq},
};
use crate::{
    move_gen::list::{SLCons, SLNil},
    util::{And, Bool, RunAnd},
    values,
};
use std::marker::PhantomData;

pub(crate) trait SquareTy {
    fn reify() -> values::Square;
}
pub(crate) struct Square<R: RankEn, F: FileEn>(PhantomData<(R, F)>);

impl<R: RankEn, F: FileEn> SquareTy for Square<R, F> {
    fn reify() -> values::Square {
        values::Square {
            rank: R::reify(),
            file: F::reify(),
        }
    }
}

pub(crate) trait RunSquareEq<A: SquareTy>: SquareTy {
    type Output: Bool;
}
pub(crate) type SquareEq<A, B> = <A as RunSquareEq<B>>::Output;

impl<R1: RankEn, F1: FileEn, R2: RankEn, F2: FileEn> RunSquareEq<Square<R1, F1>> for Square<R2, F2>
where
    R1: RunRankEq<R2>,
    F1: RunFileEq<F2>,
    RankEq<R1, R2>: RunAnd<FileEq<F1, F2>>,
{
    type Output = And<RankEq<R1, R2>, FileEq<F1, F2>>;
}

// I can't think of a more elegant way to do this.
pub(crate) type AllSqs =
SLCons<Square<rank::R1, file::FA>,
SLCons<Square<rank::R1, file::FB>,
SLCons<Square<rank::R1, file::FC>,
SLCons<Square<rank::R1, file::FD>,
SLCons<Square<rank::R1, file::FE>,
SLCons<Square<rank::R1, file::FF>,
SLCons<Square<rank::R1, file::FG>,
SLCons<Square<rank::R1, file::FH>,
SLCons<Square<rank::R2, file::FA>,
SLCons<Square<rank::R2, file::FB>,
SLCons<Square<rank::R2, file::FC>,
SLCons<Square<rank::R2, file::FD>,
SLCons<Square<rank::R2, file::FE>,
SLCons<Square<rank::R2, file::FF>,
SLCons<Square<rank::R2, file::FG>,
SLCons<Square<rank::R2, file::FH>,
SLCons<Square<rank::R3, file::FA>,
SLCons<Square<rank::R3, file::FB>,
SLCons<Square<rank::R3, file::FC>,
SLCons<Square<rank::R3, file::FD>,
SLCons<Square<rank::R3, file::FE>,
SLCons<Square<rank::R3, file::FF>,
SLCons<Square<rank::R3, file::FG>,
SLCons<Square<rank::R3, file::FH>,
SLCons<Square<rank::R4, file::FA>,
SLCons<Square<rank::R4, file::FB>,
SLCons<Square<rank::R4, file::FC>,
SLCons<Square<rank::R4, file::FD>,
SLCons<Square<rank::R4, file::FE>,
SLCons<Square<rank::R4, file::FF>,
SLCons<Square<rank::R4, file::FG>,
SLCons<Square<rank::R4, file::FH>,
SLCons<Square<rank::R5, file::FA>,
SLCons<Square<rank::R5, file::FB>,
SLCons<Square<rank::R5, file::FC>,
SLCons<Square<rank::R5, file::FD>,
SLCons<Square<rank::R5, file::FE>,
SLCons<Square<rank::R5, file::FF>,
SLCons<Square<rank::R5, file::FG>,
SLCons<Square<rank::R5, file::FH>,
SLCons<Square<rank::R6, file::FA>,
SLCons<Square<rank::R6, file::FB>,
SLCons<Square<rank::R6, file::FC>,
SLCons<Square<rank::R6, file::FD>,
SLCons<Square<rank::R6, file::FE>,
SLCons<Square<rank::R6, file::FF>,
SLCons<Square<rank::R6, file::FG>,
SLCons<Square<rank::R6, file::FH>,
SLCons<Square<rank::R7, file::FA>,
SLCons<Square<rank::R7, file::FB>,
SLCons<Square<rank::R7, file::FC>,
SLCons<Square<rank::R7, file::FD>,
SLCons<Square<rank::R7, file::FE>,
SLCons<Square<rank::R7, file::FF>,
SLCons<Square<rank::R7, file::FG>,
SLCons<Square<rank::R7, file::FH>,
SLCons<Square<rank::R8, file::FA>,
SLCons<Square<rank::R8, file::FB>,
SLCons<Square<rank::R8, file::FC>,
SLCons<Square<rank::R8, file::FD>,
SLCons<Square<rank::R8, file::FE>,
SLCons<Square<rank::R8, file::FF>,
SLCons<Square<rank::R8, file::FG>,
SLCons<Square<rank::R8, file::FH>,
SLNil,
>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>;

pub(crate) mod all_sqs {
    #![allow(unused)]

    use super::{file, rank, Square};

    pub(crate) type A1 = Square<rank::R1, file::FA>;
    pub(crate) type B1 = Square<rank::R1, file::FB>;
    pub(crate) type C1 = Square<rank::R1, file::FC>;
    pub(crate) type D1 = Square<rank::R1, file::FD>;
    pub(crate) type E1 = Square<rank::R1, file::FE>;
    pub(crate) type F1 = Square<rank::R1, file::FF>;
    pub(crate) type G1 = Square<rank::R1, file::FG>;
    pub(crate) type H1 = Square<rank::R1, file::FH>;
    pub(crate) type A2 = Square<rank::R2, file::FA>;
    pub(crate) type B2 = Square<rank::R2, file::FB>;
    pub(crate) type C2 = Square<rank::R2, file::FC>;
    pub(crate) type D2 = Square<rank::R2, file::FD>;
    pub(crate) type E2 = Square<rank::R2, file::FE>;
    pub(crate) type F2 = Square<rank::R2, file::FF>;
    pub(crate) type G2 = Square<rank::R2, file::FG>;
    pub(crate) type H2 = Square<rank::R2, file::FH>;
    pub(crate) type A3 = Square<rank::R3, file::FA>;
    pub(crate) type B3 = Square<rank::R3, file::FB>;
    pub(crate) type C3 = Square<rank::R3, file::FC>;
    pub(crate) type D3 = Square<rank::R3, file::FD>;
    pub(crate) type E3 = Square<rank::R3, file::FE>;
    pub(crate) type F3 = Square<rank::R3, file::FF>;
    pub(crate) type G3 = Square<rank::R3, file::FG>;
    pub(crate) type H3 = Square<rank::R3, file::FH>;
    pub(crate) type A4 = Square<rank::R4, file::FA>;
    pub(crate) type B4 = Square<rank::R4, file::FB>;
    pub(crate) type C4 = Square<rank::R4, file::FC>;
    pub(crate) type D4 = Square<rank::R4, file::FD>;
    pub(crate) type E4 = Square<rank::R4, file::FE>;
    pub(crate) type F4 = Square<rank::R4, file::FF>;
    pub(crate) type G4 = Square<rank::R4, file::FG>;
    pub(crate) type H4 = Square<rank::R4, file::FH>;
    pub(crate) type A5 = Square<rank::R5, file::FA>;
    pub(crate) type B5 = Square<rank::R5, file::FB>;
    pub(crate) type C5 = Square<rank::R5, file::FC>;
    pub(crate) type D5 = Square<rank::R5, file::FD>;
    pub(crate) type E5 = Square<rank::R5, file::FE>;
    pub(crate) type F5 = Square<rank::R5, file::FF>;
    pub(crate) type G5 = Square<rank::R5, file::FG>;
    pub(crate) type H5 = Square<rank::R5, file::FH>;
    pub(crate) type A6 = Square<rank::R6, file::FA>;
    pub(crate) type B6 = Square<rank::R6, file::FB>;
    pub(crate) type C6 = Square<rank::R6, file::FC>;
    pub(crate) type D6 = Square<rank::R6, file::FD>;
    pub(crate) type E6 = Square<rank::R6, file::FE>;
    pub(crate) type F6 = Square<rank::R6, file::FF>;
    pub(crate) type G6 = Square<rank::R6, file::FG>;
    pub(crate) type H6 = Square<rank::R6, file::FH>;
    pub(crate) type A7 = Square<rank::R7, file::FA>;
    pub(crate) type B7 = Square<rank::R7, file::FB>;
    pub(crate) type C7 = Square<rank::R7, file::FC>;
    pub(crate) type D7 = Square<rank::R7, file::FD>;
    pub(crate) type E7 = Square<rank::R7, file::FE>;
    pub(crate) type F7 = Square<rank::R7, file::FF>;
    pub(crate) type G7 = Square<rank::R7, file::FG>;
    pub(crate) type H7 = Square<rank::R7, file::FH>;
    pub(crate) type A8 = Square<rank::R8, file::FA>;
    pub(crate) type B8 = Square<rank::R8, file::FB>;
    pub(crate) type C8 = Square<rank::R8, file::FC>;
    pub(crate) type D8 = Square<rank::R8, file::FD>;
    pub(crate) type E8 = Square<rank::R8, file::FE>;
    pub(crate) type F8 = Square<rank::R8, file::FF>;
    pub(crate) type G8 = Square<rank::R8, file::FG>;
    pub(crate) type H8 = Square<rank::R8, file::FH>;
}
