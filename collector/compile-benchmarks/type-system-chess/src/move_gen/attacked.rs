use super::{
    bishop::{BishopMoveSqs, RunBishopMoveSqs},
    king::{KingMoveSqs, RunKingMoveSqs},
    knight::{KnightMoveSqs, RunKnightMoveSqs},
    list::{SLCons, SLNil, SquareListTy},
    pawn::{PawnAttackSqs, RunPawnAttackSqs},
    queen::{QueenMoveSqs, RunQueenMoveSqs},
    rook::{RookMoveSqs, RunRookMoveSqs},
};
use crate::board_rep::{
    board::{
        idx::{IdxBoard, RunIdxBoard},
        BoardTy, CellEn, Empty, Filled,
    },
    color::{Black, ColorEn, White},
    piece::{Bishop, ColoredPiece, King, Knight, Pawn, PieceEn, Queen, Rook},
    square::{
        set::{AddToSSFromL, EmptySS, RunAddToSSFromL, SquareSetTy},
        AllSqs, SquareTy,
    },
};

pub(crate) trait RunAttacked<MoverC: ColorEn>: BoardTy {
    type Output: SquareSetTy;
}
pub(crate) type Attacked<B, C> = <B as RunAttacked<C>>::Output;

impl<B: BoardTy, MoverC: ColorEn> RunAttacked<MoverC> for B
where
    B: RunAttacksForSqL<MoverC, AllSqs>,
{
    type Output = <B as RunAttacksForSqL<MoverC, AllSqs>>::Output;
}

pub(crate) trait RunAttacksForSqL<MoverC: ColorEn, L: SquareListTy>: BoardTy {
    type Output: SquareSetTy;
}
pub(crate) type AttacksForSqL<B, C, L> = <B as RunAttacksForSqL<C, L>>::Output;

impl<B: BoardTy, MoverC: ColorEn> RunAttacksForSqL<MoverC, SLNil> for B {
    type Output = EmptySS;
}
impl<B: BoardTy, MoverC: ColorEn, S: SquareTy, Next: SquareListTy>
    RunAttacksForSqL<MoverC, SLCons<S, Next>> for B
where
    B: RunAttacksForSqL<MoverC, Next>,
    S: RunAttacksForSq<B, MoverC>,
    AttacksForSqL<B, MoverC, Next>: RunAddToSSFromL<AttacksForSq<S, B, MoverC>>,
{
    type Output = AddToSSFromL<AttacksForSqL<B, MoverC, Next>, AttacksForSq<S, B, MoverC>>;
}

// This squares that contain pieces of the mover's color but it doesn't
// matter because we only use this to check for checks.
pub(crate) trait RunAttacksForSq<B: BoardTy, MoverC: ColorEn>: SquareTy {
    type Output: SquareListTy;
}
pub(crate) type AttacksForSq<S, B, MoverC> = <S as RunAttacksForSq<B, MoverC>>::Output;

impl<S: SquareTy, B: BoardTy, MoverC: ColorEn> RunAttacksForSq<B, MoverC> for S
where
    B: RunIdxBoard<S>,
    S: RunAttacksForTypeAtSq<B, MoverC, IdxBoard<B, S>>,
{
    type Output = <S as RunAttacksForTypeAtSq<B, MoverC, IdxBoard<B, S>>>::Output;
}

pub(crate) trait RunAttacksForTypeAtSq<B: BoardTy, MoverC: ColorEn, Type: CellEn>:
    SquareTy
{
    type Output: SquareListTy;
}

impl<S: SquareTy, B: BoardTy, MoverC: ColorEn> RunAttacksForTypeAtSq<B, MoverC, Empty> for S {
    type Output = SLNil;
}
impl<S: SquareTy, B: BoardTy, P: PieceEn>
    RunAttacksForTypeAtSq<B, White, Filled<ColoredPiece<P, Black>>> for S
{
    type Output = SLNil;
}
impl<S: SquareTy, B: BoardTy, P: PieceEn>
    RunAttacksForTypeAtSq<B, Black, Filled<ColoredPiece<P, White>>> for S
{
    type Output = SLNil;
}

impl<S: SquareTy + RunPawnAttackSqs<C>, B: BoardTy, C: ColorEn>
    RunAttacksForTypeAtSq<B, C, Filled<ColoredPiece<Pawn, C>>> for S
{
    type Output = PawnAttackSqs<S, C>;
}
impl<S: SquareTy + RunKnightMoveSqs, B: BoardTy, C: ColorEn>
    RunAttacksForTypeAtSq<B, C, Filled<ColoredPiece<Knight, C>>> for S
{
    type Output = KnightMoveSqs<S>;
}
impl<S: SquareTy + RunBishopMoveSqs<B>, B: BoardTy, C: ColorEn>
    RunAttacksForTypeAtSq<B, C, Filled<ColoredPiece<Bishop, C>>> for S
{
    type Output = BishopMoveSqs<S, B>;
}
impl<S: SquareTy + RunRookMoveSqs<B>, B: BoardTy, C: ColorEn>
    RunAttacksForTypeAtSq<B, C, Filled<ColoredPiece<Rook, C>>> for S
{
    type Output = RookMoveSqs<S, B>;
}
impl<S: SquareTy + RunQueenMoveSqs<B>, B: BoardTy, C: ColorEn>
    RunAttacksForTypeAtSq<B, C, Filled<ColoredPiece<Queen, C>>> for S
{
    type Output = QueenMoveSqs<S, B>;
}
impl<S: SquareTy + RunKingMoveSqs, B: BoardTy, C: ColorEn>
    RunAttacksForTypeAtSq<B, C, Filled<ColoredPiece<King, C>>> for S
{
    type Output = KingMoveSqs<S>;
}
