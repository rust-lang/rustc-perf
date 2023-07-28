use super::list::{SLCons, SquareListTy};
use crate::board_rep::{
    board::{
        idx::{IdxBoard, RunIdxBoard},
        BoardTy, CellEn, Empty, Filled,
    },
    color::ColorEn,
    piece::{ColoredPiece, PieceEn},
    square::{
        offset::{MaybeSquare, NoSquare, OffsetSquare, OffsetTy, RunOffsetSquare, SomeSquare},
        SquareTy,
    },
};

pub(crate) trait RunAddCastSqs<B: BoardTy, Start: SquareTy, O: OffsetTy>:
    SquareListTy
{
    type Output: SquareListTy;
}
pub(crate) type AddCastSqs<L, B, S, O> = <L as RunAddCastSqs<B, S, O>>::Output;

impl<L: SquareListTy, B: BoardTy, S: SquareTy, O: OffsetTy> RunAddCastSqs<B, S, O> for L
where
    S: RunOffsetSquare<O>,
    L: RunCastMaybeSq<B, OffsetSquare<S, O>, O>,
{
    // We offset the square to start because the initial square is on top of the moving piece.
    type Output = <L as RunCastMaybeSq<B, OffsetSquare<S, O>, O>>::Output;
}

pub(crate) trait RunCastMaybeSq<B: BoardTy, S: MaybeSquare, O: OffsetTy>:
    SquareListTy
{
    type Output: SquareListTy;
}

impl<L: SquareListTy, B: BoardTy, O: OffsetTy> RunCastMaybeSq<B, NoSquare, O> for L {
    type Output = L;
}
impl<L: SquareListTy, B: BoardTy, S: SquareTy, O: OffsetTy> RunCastMaybeSq<B, SomeSquare<S>, O>
    for L
where
    B: RunIdxBoard<S>,
    L: RunCastSqWithContent<B, S, O, IdxBoard<B, S>>,
{
    type Output = <L as RunCastSqWithContent<B, S, O, IdxBoard<B, S>>>::Output;
}

pub(crate) trait RunCastSqWithContent<B: BoardTy, S: SquareTy, O: OffsetTy, C: CellEn>:
    SquareListTy
{
    type Output: SquareListTy;
}

impl<L: SquareListTy, B: BoardTy, S: SquareTy, O: OffsetTy, P: PieceEn, C: ColorEn>
    RunCastSqWithContent<B, S, O, Filled<ColoredPiece<P, C>>> for L
{
    type Output = SLCons<S, L>;
}
impl<L: SquareListTy, B: BoardTy, S: SquareTy, O: OffsetTy> RunCastSqWithContent<B, S, O, Empty>
    for L
where
    S: RunOffsetSquare<O>,
    L: RunCastMaybeSq<B, OffsetSquare<S, O>, O>,
{
    type Output = SLCons<S, <L as RunCastMaybeSq<B, OffsetSquare<S, O>, O>>::Output>;
}
