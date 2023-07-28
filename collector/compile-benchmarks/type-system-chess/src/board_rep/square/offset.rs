use crate::values;
use std::marker::PhantomData;

use self::{
    file::{NoFile, OffsetFile, RunOffsetFile, SomeFile},
    rank::{NoRank, OffsetRank, RunOffsetRank, SomeRank},
};
use super::{file::FileEn, rank::RankEn, Square, SquareTy};

pub mod file;
pub mod rank;

pub(crate) trait Offset1DEn {
    fn reify() -> i8;
}
pub(crate) struct Neg2;
pub(crate) struct Neg1;
pub(crate) struct Zero;
pub(crate) struct Pos1;
pub(crate) struct Pos2;

impl Offset1DEn for Neg2 {
    fn reify() -> i8 {
        -2
    }
}
impl Offset1DEn for Neg1 {
    fn reify() -> i8 {
        -1
    }
}
impl Offset1DEn for Zero {
    fn reify() -> i8 {
        0
    }
}
impl Offset1DEn for Pos1 {
    fn reify() -> i8 {
        1
    }
}
impl Offset1DEn for Pos2 {
    fn reify() -> i8 {
        2
    }
}

pub(crate) trait OffsetTy {
    fn reify() -> values::Offset;
}
pub(crate) struct Offset<R: Offset1DEn, F: Offset1DEn>(PhantomData<(R, F)>);

impl<R: Offset1DEn, F: Offset1DEn> OffsetTy for Offset<R, F> {
    fn reify() -> values::Offset {
        values::Offset {
            rank: R::reify(),
            file: F::reify(),
        }
    }
}

pub(crate) trait MaybeSquare {
    fn reify() -> Option<values::Square>;
}

pub(crate) struct NoSquare;
pub(crate) struct SomeSquare<S: SquareTy>(PhantomData<S>);

impl MaybeSquare for NoSquare {
    fn reify() -> Option<values::Square> {
        None
    }
}
impl<S: SquareTy> MaybeSquare for SomeSquare<S> {
    fn reify() -> Option<values::Square> {
        Some(S::reify())
    }
}

pub(crate) trait RunSquareFromMaybeTuple {
    type Output: MaybeSquare;
}

impl RunSquareFromMaybeTuple for (NoRank, NoFile) {
    type Output = NoSquare;
}
impl<R: RankEn> RunSquareFromMaybeTuple for (SomeRank<R>, NoFile) {
    type Output = NoSquare;
}
impl<F: FileEn> RunSquareFromMaybeTuple for (NoRank, SomeFile<F>) {
    type Output = NoSquare;
}
impl<R: RankEn, F: FileEn> RunSquareFromMaybeTuple for (SomeRank<R>, SomeFile<F>) {
    type Output = SomeSquare<Square<R, F>>;
}

pub(crate) trait RunOffsetSquare<O: OffsetTy> {
    type Output: MaybeSquare;
}
pub(crate) type OffsetSquare<S, O> = <S as RunOffsetSquare<O>>::Output;

impl<RO, FO, R, F> RunOffsetSquare<Offset<RO, FO>> for Square<R, F>
where
    RO: Offset1DEn,
    FO: Offset1DEn,
    R: RunOffsetRank<RO>,
    F: RunOffsetFile<FO>,
    (OffsetRank<R, RO>, OffsetFile<F, FO>): RunSquareFromMaybeTuple,
{
    type Output = <(OffsetRank<R, RO>, OffsetFile<F, FO>) as RunSquareFromMaybeTuple>::Output;
}
