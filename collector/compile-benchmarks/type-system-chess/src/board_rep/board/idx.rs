use super::{Board, BoardRank, BoardRankTy, BoardTy, CellEn};
use crate::board_rep::square::{
    file::{self, FileEn},
    rank::{self, RankEn},
    Square, SquareTy,
};

pub(crate) trait RunIdxRank<With> {
    type Output: CellEn;
}
pub(crate) type IdxRank<B, F> = <B as RunIdxRank<F>>::Output;

impl<A: CellEn, B: CellEn, C: CellEn, D: CellEn, E: CellEn, F: CellEn, G: CellEn, H: CellEn>
    RunIdxRank<file::FA> for BoardRank<A, B, C, D, E, F, G, H>
{
    type Output = A;
}
impl<A: CellEn, B: CellEn, C: CellEn, D: CellEn, E: CellEn, F: CellEn, G: CellEn, H: CellEn>
    RunIdxRank<file::FB> for BoardRank<A, B, C, D, E, F, G, H>
{
    type Output = B;
}
impl<A: CellEn, B: CellEn, C: CellEn, D: CellEn, E: CellEn, F: CellEn, G: CellEn, H: CellEn>
    RunIdxRank<file::FC> for BoardRank<A, B, C, D, E, F, G, H>
{
    type Output = C;
}
impl<A: CellEn, B: CellEn, C: CellEn, D: CellEn, E: CellEn, F: CellEn, G: CellEn, H: CellEn>
    RunIdxRank<file::FD> for BoardRank<A, B, C, D, E, F, G, H>
{
    type Output = D;
}
impl<A: CellEn, B: CellEn, C: CellEn, D: CellEn, E: CellEn, F: CellEn, G: CellEn, H: CellEn>
    RunIdxRank<file::FE> for BoardRank<A, B, C, D, E, F, G, H>
{
    type Output = E;
}
impl<A: CellEn, B: CellEn, C: CellEn, D: CellEn, E: CellEn, F: CellEn, G: CellEn, H: CellEn>
    RunIdxRank<file::FF> for BoardRank<A, B, C, D, E, F, G, H>
{
    type Output = F;
}
impl<A: CellEn, B: CellEn, C: CellEn, D: CellEn, E: CellEn, F: CellEn, G: CellEn, H: CellEn>
    RunIdxRank<file::FG> for BoardRank<A, B, C, D, E, F, G, H>
{
    type Output = G;
}
impl<A: CellEn, B: CellEn, C: CellEn, D: CellEn, E: CellEn, F: CellEn, G: CellEn, H: CellEn>
    RunIdxRank<file::FH> for BoardRank<A, B, C, D, E, F, G, H>
{
    type Output = H;
}

pub(crate) trait RunIdxBoardRank<With> {
    type Output: BoardRankTy;
}
pub(crate) type IdxBoardRank<B, R> = <B as RunIdxBoardRank<R>>::Output;

impl<
        R1: BoardRankTy,
        R2: BoardRankTy,
        R3: BoardRankTy,
        R4: BoardRankTy,
        R5: BoardRankTy,
        R6: BoardRankTy,
        R7: BoardRankTy,
        R8: BoardRankTy,
    > RunIdxBoardRank<rank::R1> for Board<R1, R2, R3, R4, R5, R6, R7, R8>
{
    type Output = R1;
}
impl<
        R1: BoardRankTy,
        R2: BoardRankTy,
        R3: BoardRankTy,
        R4: BoardRankTy,
        R5: BoardRankTy,
        R6: BoardRankTy,
        R7: BoardRankTy,
        R8: BoardRankTy,
    > RunIdxBoardRank<rank::R2> for Board<R1, R2, R3, R4, R5, R6, R7, R8>
{
    type Output = R2;
}
impl<
        R1: BoardRankTy,
        R2: BoardRankTy,
        R3: BoardRankTy,
        R4: BoardRankTy,
        R5: BoardRankTy,
        R6: BoardRankTy,
        R7: BoardRankTy,
        R8: BoardRankTy,
    > RunIdxBoardRank<rank::R3> for Board<R1, R2, R3, R4, R5, R6, R7, R8>
{
    type Output = R3;
}
impl<
        R1: BoardRankTy,
        R2: BoardRankTy,
        R3: BoardRankTy,
        R4: BoardRankTy,
        R5: BoardRankTy,
        R6: BoardRankTy,
        R7: BoardRankTy,
        R8: BoardRankTy,
    > RunIdxBoardRank<rank::R4> for Board<R1, R2, R3, R4, R5, R6, R7, R8>
{
    type Output = R4;
}
impl<
        R1: BoardRankTy,
        R2: BoardRankTy,
        R3: BoardRankTy,
        R4: BoardRankTy,
        R5: BoardRankTy,
        R6: BoardRankTy,
        R7: BoardRankTy,
        R8: BoardRankTy,
    > RunIdxBoardRank<rank::R5> for Board<R1, R2, R3, R4, R5, R6, R7, R8>
{
    type Output = R5;
}
impl<
        R1: BoardRankTy,
        R2: BoardRankTy,
        R3: BoardRankTy,
        R4: BoardRankTy,
        R5: BoardRankTy,
        R6: BoardRankTy,
        R7: BoardRankTy,
        R8: BoardRankTy,
    > RunIdxBoardRank<rank::R6> for Board<R1, R2, R3, R4, R5, R6, R7, R8>
{
    type Output = R6;
}
impl<
        R1: BoardRankTy,
        R2: BoardRankTy,
        R3: BoardRankTy,
        R4: BoardRankTy,
        R5: BoardRankTy,
        R6: BoardRankTy,
        R7: BoardRankTy,
        R8: BoardRankTy,
    > RunIdxBoardRank<rank::R7> for Board<R1, R2, R3, R4, R5, R6, R7, R8>
{
    type Output = R7;
}
impl<
        R1: BoardRankTy,
        R2: BoardRankTy,
        R3: BoardRankTy,
        R4: BoardRankTy,
        R5: BoardRankTy,
        R6: BoardRankTy,
        R7: BoardRankTy,
        R8: BoardRankTy,
    > RunIdxBoardRank<rank::R8> for Board<R1, R2, R3, R4, R5, R6, R7, R8>
{
    type Output = R8;
}

pub(crate) trait RunIdxBoard<With: SquareTy> {
    type Output: CellEn;
}
pub(crate) type IdxBoard<B, S> = <B as RunIdxBoard<S>>::Output;

impl<R: RankEn, F: FileEn, B: BoardTy> RunIdxBoard<Square<R, F>> for B
where
    B: RunIdxBoardRank<R>,
    IdxBoardRank<B, R>: RunIdxRank<F>,
{
    type Output = IdxRank<IdxBoardRank<B, R>, F>;
}
