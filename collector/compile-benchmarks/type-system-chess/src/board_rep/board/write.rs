use crate::board_rep::square::{
    file::{self, FileEn},
    offset::{MaybeSquare, NoSquare, SomeSquare},
    rank, Square, SquareTy,
};

use super::{Board, BoardRank, BoardRankTy, BoardTy, CellEn};

pub(crate) trait RunWriteToBoardRank<F: FileEn, C: CellEn>: BoardRankTy {
    type Output: BoardRankTy;
}
pub(crate) type WriteToBoardRank<B, F, C> = <B as RunWriteToBoardRank<F, C>>::Output;

impl<
        A: CellEn,
        B: CellEn,
        C: CellEn,
        D: CellEn,
        E: CellEn,
        F: CellEn,
        G: CellEn,
        H: CellEn,
        CE: CellEn,
    > RunWriteToBoardRank<file::FA, CE> for BoardRank<A, B, C, D, E, F, G, H>
{
    type Output = BoardRank<CE, B, C, D, E, F, G, H>;
}
impl<
        A: CellEn,
        B: CellEn,
        C: CellEn,
        D: CellEn,
        E: CellEn,
        F: CellEn,
        G: CellEn,
        H: CellEn,
        CE: CellEn,
    > RunWriteToBoardRank<file::FB, CE> for BoardRank<A, B, C, D, E, F, G, H>
{
    type Output = BoardRank<A, CE, C, D, E, F, G, H>;
}
impl<
        A: CellEn,
        B: CellEn,
        C: CellEn,
        D: CellEn,
        E: CellEn,
        F: CellEn,
        G: CellEn,
        H: CellEn,
        CE: CellEn,
    > RunWriteToBoardRank<file::FC, CE> for BoardRank<A, B, C, D, E, F, G, H>
{
    type Output = BoardRank<A, B, CE, D, E, F, G, H>;
}
impl<
        A: CellEn,
        B: CellEn,
        C: CellEn,
        D: CellEn,
        E: CellEn,
        F: CellEn,
        G: CellEn,
        H: CellEn,
        CE: CellEn,
    > RunWriteToBoardRank<file::FD, CE> for BoardRank<A, B, C, D, E, F, G, H>
{
    type Output = BoardRank<A, B, C, CE, E, F, G, H>;
}
impl<
        A: CellEn,
        B: CellEn,
        C: CellEn,
        D: CellEn,
        E: CellEn,
        F: CellEn,
        G: CellEn,
        H: CellEn,
        CE: CellEn,
    > RunWriteToBoardRank<file::FE, CE> for BoardRank<A, B, C, D, E, F, G, H>
{
    type Output = BoardRank<A, B, C, D, CE, F, G, H>;
}
impl<
        A: CellEn,
        B: CellEn,
        C: CellEn,
        D: CellEn,
        E: CellEn,
        F: CellEn,
        G: CellEn,
        H: CellEn,
        CE: CellEn,
    > RunWriteToBoardRank<file::FF, CE> for BoardRank<A, B, C, D, E, F, G, H>
{
    type Output = BoardRank<A, B, C, D, E, CE, G, H>;
}
impl<
        A: CellEn,
        B: CellEn,
        C: CellEn,
        D: CellEn,
        E: CellEn,
        F: CellEn,
        G: CellEn,
        H: CellEn,
        CE: CellEn,
    > RunWriteToBoardRank<file::FG, CE> for BoardRank<A, B, C, D, E, F, G, H>
{
    type Output = BoardRank<A, B, C, D, E, F, CE, H>;
}
impl<
        A: CellEn,
        B: CellEn,
        C: CellEn,
        D: CellEn,
        E: CellEn,
        F: CellEn,
        G: CellEn,
        H: CellEn,
        CE: CellEn,
    > RunWriteToBoardRank<file::FH, CE> for BoardRank<A, B, C, D, E, F, G, H>
{
    type Output = BoardRank<A, B, C, D, E, F, G, CE>;
}

pub(crate) trait RunWriteToBoard<S: SquareTy, C: CellEn>: BoardTy {
    type Output: BoardTy;
}
pub(crate) type WriteToBoard<B, S, C> = <B as RunWriteToBoard<S, C>>::Output;

impl<
        R1: BoardRankTy,
        R2: BoardRankTy,
        R3: BoardRankTy,
        R4: BoardRankTy,
        R5: BoardRankTy,
        R6: BoardRankTy,
        R7: BoardRankTy,
        R8: BoardRankTy,
        F: FileEn,
        C: CellEn,
    > RunWriteToBoard<Square<rank::R1, F>, C> for Board<R1, R2, R3, R4, R5, R6, R7, R8>
where
    R1: RunWriteToBoardRank<F, C>,
{
    type Output = Board<WriteToBoardRank<R1, F, C>, R2, R3, R4, R5, R6, R7, R8>;
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
        F: FileEn,
        C: CellEn,
    > RunWriteToBoard<Square<rank::R2, F>, C> for Board<R1, R2, R3, R4, R5, R6, R7, R8>
where
    R2: RunWriteToBoardRank<F, C>,
{
    type Output = Board<R1, WriteToBoardRank<R2, F, C>, R3, R4, R5, R6, R7, R8>;
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
        F: FileEn,
        C: CellEn,
    > RunWriteToBoard<Square<rank::R3, F>, C> for Board<R1, R2, R3, R4, R5, R6, R7, R8>
where
    R3: RunWriteToBoardRank<F, C>,
{
    type Output = Board<R1, R2, WriteToBoardRank<R3, F, C>, R4, R5, R6, R7, R8>;
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
        F: FileEn,
        C: CellEn,
    > RunWriteToBoard<Square<rank::R4, F>, C> for Board<R1, R2, R3, R4, R5, R6, R7, R8>
where
    R4: RunWriteToBoardRank<F, C>,
{
    type Output = Board<R1, R2, R3, WriteToBoardRank<R4, F, C>, R5, R6, R7, R8>;
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
        F: FileEn,
        C: CellEn,
    > RunWriteToBoard<Square<rank::R5, F>, C> for Board<R1, R2, R3, R4, R5, R6, R7, R8>
where
    R5: RunWriteToBoardRank<F, C>,
{
    type Output = Board<R1, R2, R3, R4, WriteToBoardRank<R5, F, C>, R6, R7, R8>;
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
        F: FileEn,
        C: CellEn,
    > RunWriteToBoard<Square<rank::R6, F>, C> for Board<R1, R2, R3, R4, R5, R6, R7, R8>
where
    R6: RunWriteToBoardRank<F, C>,
{
    type Output = Board<R1, R2, R3, R4, R5, WriteToBoardRank<R6, F, C>, R7, R8>;
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
        F: FileEn,
        C: CellEn,
    > RunWriteToBoard<Square<rank::R7, F>, C> for Board<R1, R2, R3, R4, R5, R6, R7, R8>
where
    R7: RunWriteToBoardRank<F, C>,
{
    type Output = Board<R1, R2, R3, R4, R5, R6, WriteToBoardRank<R7, F, C>, R8>;
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
        F: FileEn,
        C: CellEn,
    > RunWriteToBoard<Square<rank::R8, F>, C> for Board<R1, R2, R3, R4, R5, R6, R7, R8>
where
    R8: RunWriteToBoardRank<F, C>,
{
    type Output = Board<R1, R2, R3, R4, R5, R6, R7, WriteToBoardRank<R8, F, C>>;
}

pub(crate) trait RunMaybeWriteToBoard<S: MaybeSquare, C: CellEn>: BoardTy {
    type Output: BoardTy;
}
pub(crate) type MaybeWriteToBoard<B, S, C> = <B as RunMaybeWriteToBoard<S, C>>::Output;

impl<B: BoardTy, C: CellEn> RunMaybeWriteToBoard<NoSquare, C> for B {
    type Output = B;
}
impl<B: BoardTy, S: SquareTy, C: CellEn> RunMaybeWriteToBoard<SomeSquare<S>, C> for B
where
    B: RunWriteToBoard<S, C>,
{
    type Output = WriteToBoard<B, S, C>;
}
