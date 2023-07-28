use super::{Neg1, Neg2, Offset1DEn, Pos1, Pos2, Zero};
use crate::{
    board_rep::square::rank::{self, RankEn},
    values,
};
use std::marker::PhantomData;

pub(crate) trait MaybeRank {
    fn reify() -> Option<values::Rank>;
}

pub(crate) struct NoRank;
pub(crate) struct SomeRank<R: RankEn>(PhantomData<R>);

impl MaybeRank for NoRank {
    fn reify() -> Option<values::Rank> {
        None
    }
}
impl<R: RankEn> MaybeRank for SomeRank<R> {
    fn reify() -> Option<values::Rank> {
        Some(R::reify())
    }
}

pub(crate) trait RunOffsetRank<O: Offset1DEn>: RankEn {
    type Output: MaybeRank;
}
pub(crate) type OffsetRank<R, O> = <R as RunOffsetRank<O>>::Output;

impl RunOffsetRank<Neg2> for rank::R1 {
    type Output = NoRank;
}
impl RunOffsetRank<Neg2> for rank::R2 {
    type Output = NoRank;
}
impl RunOffsetRank<Neg2> for rank::R3 {
    type Output = SomeRank<rank::R1>;
}
impl RunOffsetRank<Neg2> for rank::R4 {
    type Output = SomeRank<rank::R2>;
}
impl RunOffsetRank<Neg2> for rank::R5 {
    type Output = SomeRank<rank::R3>;
}
impl RunOffsetRank<Neg2> for rank::R6 {
    type Output = SomeRank<rank::R4>;
}
impl RunOffsetRank<Neg2> for rank::R7 {
    type Output = SomeRank<rank::R5>;
}
impl RunOffsetRank<Neg2> for rank::R8 {
    type Output = SomeRank<rank::R6>;
}
impl RunOffsetRank<Neg1> for rank::R1 {
    type Output = NoRank;
}
impl RunOffsetRank<Neg1> for rank::R2 {
    type Output = SomeRank<rank::R1>;
}
impl RunOffsetRank<Neg1> for rank::R3 {
    type Output = SomeRank<rank::R2>;
}
impl RunOffsetRank<Neg1> for rank::R4 {
    type Output = SomeRank<rank::R3>;
}
impl RunOffsetRank<Neg1> for rank::R5 {
    type Output = SomeRank<rank::R4>;
}
impl RunOffsetRank<Neg1> for rank::R6 {
    type Output = SomeRank<rank::R5>;
}
impl RunOffsetRank<Neg1> for rank::R7 {
    type Output = SomeRank<rank::R6>;
}
impl RunOffsetRank<Neg1> for rank::R8 {
    type Output = SomeRank<rank::R7>;
}
impl<R: RankEn> RunOffsetRank<Zero> for R {
    type Output = SomeRank<R>;
}
impl RunOffsetRank<Pos1> for rank::R1 {
    type Output = SomeRank<rank::R2>;
}
impl RunOffsetRank<Pos1> for rank::R2 {
    type Output = SomeRank<rank::R3>;
}
impl RunOffsetRank<Pos1> for rank::R3 {
    type Output = SomeRank<rank::R4>;
}
impl RunOffsetRank<Pos1> for rank::R4 {
    type Output = SomeRank<rank::R5>;
}
impl RunOffsetRank<Pos1> for rank::R5 {
    type Output = SomeRank<rank::R6>;
}
impl RunOffsetRank<Pos1> for rank::R6 {
    type Output = SomeRank<rank::R7>;
}
impl RunOffsetRank<Pos1> for rank::R7 {
    type Output = SomeRank<rank::R8>;
}
impl RunOffsetRank<Pos1> for rank::R8 {
    type Output = NoRank;
}
impl RunOffsetRank<Pos2> for rank::R1 {
    type Output = SomeRank<rank::R3>;
}
impl RunOffsetRank<Pos2> for rank::R2 {
    type Output = SomeRank<rank::R4>;
}
impl RunOffsetRank<Pos2> for rank::R3 {
    type Output = SomeRank<rank::R5>;
}
impl RunOffsetRank<Pos2> for rank::R4 {
    type Output = SomeRank<rank::R6>;
}
impl RunOffsetRank<Pos2> for rank::R5 {
    type Output = SomeRank<rank::R7>;
}
impl RunOffsetRank<Pos2> for rank::R6 {
    type Output = SomeRank<rank::R8>;
}
impl RunOffsetRank<Pos2> for rank::R7 {
    type Output = NoRank;
}
impl RunOffsetRank<Pos2> for rank::R8 {
    type Output = NoRank;
}
