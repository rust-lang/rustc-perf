use super::piece::ColoredPieceTy;
use crate::values;
use std::marker::PhantomData;

pub mod idx;
pub mod write;

pub(crate) trait CellEn {
    fn reify() -> values::Cell;
}
pub(crate) struct Empty;
pub(crate) struct Filled<P: ColoredPieceTy>(PhantomData<P>);

impl CellEn for Empty {
    fn reify() -> values::Cell {
        values::Cell::Empty
    }
}
impl<P: ColoredPieceTy> CellEn for Filled<P> {
    fn reify() -> values::Cell {
        values::Cell::Filled(P::reify())
    }
}

pub(crate) trait BoardRankTy {
    fn reify() -> values::BoardRank;
}
pub(crate) struct BoardRank<
    A: CellEn,
    B: CellEn,
    C: CellEn,
    D: CellEn,
    E: CellEn,
    F: CellEn,
    G: CellEn,
    H: CellEn,
>(PhantomData<(A, B, C, D, E, F, G, H)>);

impl<A: CellEn, B: CellEn, C: CellEn, D: CellEn, E: CellEn, F: CellEn, G: CellEn, H: CellEn>
    BoardRankTy for BoardRank<A, B, C, D, E, F, G, H>
{
    fn reify() -> values::BoardRank {
        values::BoardRank {
            a: A::reify(),
            b: B::reify(),
            c: C::reify(),
            d: D::reify(),
            e: E::reify(),
            f: F::reify(),
            g: G::reify(),
            h: H::reify(),
        }
    }
}

pub(crate) trait BoardTy {
    fn reify() -> values::Board;
}
pub(crate) struct Board<
    R1: BoardRankTy,
    R2: BoardRankTy,
    R3: BoardRankTy,
    R4: BoardRankTy,
    R5: BoardRankTy,
    R6: BoardRankTy,
    R7: BoardRankTy,
    R8: BoardRankTy,
>(PhantomData<(R1, R2, R3, R4, R5, R6, R7, R8)>);

impl<
        R1: BoardRankTy,
        R2: BoardRankTy,
        R3: BoardRankTy,
        R4: BoardRankTy,
        R5: BoardRankTy,
        R6: BoardRankTy,
        R7: BoardRankTy,
        R8: BoardRankTy,
    > BoardTy for Board<R1, R2, R3, R4, R5, R6, R7, R8>
{
    fn reify() -> values::Board {
        values::Board {
            r1: R1::reify(),
            r2: R2::reify(),
            r3: R3::reify(),
            r4: R4::reify(),
            r5: R5::reify(),
            r6: R6::reify(),
            r7: R7::reify(),
            r8: R8::reify(),
        }
    }
}
