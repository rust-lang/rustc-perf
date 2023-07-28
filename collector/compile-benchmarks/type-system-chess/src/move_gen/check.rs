use super::{
    attacked::{Attacked, RunAttacked},
    list::{SLCons, SLNil, SquareListTy},
};
use crate::{
    board_rep::{
        board::{
            idx::{IdxBoard, RunIdxBoard},
            BoardTy, CellEn, Empty, Filled,
        },
        color::{Black, ColorEn, White},
        piece::{Bishop, ColoredPiece, King, Knight, Pawn, PieceEn, Queen, Rook},
        square::{
            offset::MaybeSquare,
            set::{IsOccupied, RunIsOccupied, SquareSetTy},
            AllSqs, SquareTy,
        },
    },
    state::{CastleStateTy, State, StateTy},
    util::{Bool, False, Or, RunOr, True},
};

pub(crate) trait RunIsCheck<MoverC: ColorEn>: StateTy {
    type Output: Bool;
}
pub(crate) type IsCheck<S, MoverC> = <S as RunIsCheck<MoverC>>::Output;

impl<B: BoardTy, MoverC: ColorEn, C: ColorEn, EP: MaybeSquare, CA: CastleStateTy> RunIsCheck<MoverC>
    for State<C, B, EP, CA>
where
    B: RunAttacked<MoverC>,
    B: RunSqLIsCheck<Attacked<B, MoverC>, MoverC, AllSqs>,
{
    type Output = <B as RunSqLIsCheck<Attacked<B, MoverC>, MoverC, AllSqs>>::Output;
}

pub(crate) trait RunSqLIsCheck<Attacked: SquareSetTy, MoverC: ColorEn, SL: SquareListTy>:
    BoardTy
{
    type Output: Bool;
}
pub(crate) type SqLIsCheck<B, A, C, L> = <B as RunSqLIsCheck<A, C, L>>::Output;

impl<B: BoardTy, Attacked: SquareSetTy, MoverC: ColorEn> RunSqLIsCheck<Attacked, MoverC, SLNil>
    for B
{
    type Output = False;
}
impl<B: BoardTy, Attacked: SquareSetTy, MoverC: ColorEn, S: SquareTy, Next: SquareListTy>
    RunSqLIsCheck<Attacked, MoverC, SLCons<S, Next>> for B
where
    B: RunSqIsCheck<Attacked, MoverC, S>,
    B: RunSqLIsCheck<Attacked, MoverC, Next>,
    SqIsCheck<B, Attacked, MoverC, S>: RunOr<SqLIsCheck<B, Attacked, MoverC, Next>>,
{
    type Output = Or<SqIsCheck<B, Attacked, MoverC, S>, SqLIsCheck<B, Attacked, MoverC, Next>>;
}

pub(crate) trait RunSqIsCheck<Attacked: SquareSetTy, MoverC: ColorEn, S: SquareTy>:
    BoardTy
{
    type Output: Bool;
}
pub(crate) type SqIsCheck<B, A, C, S> = <B as RunSqIsCheck<A, C, S>>::Output;

impl<B: BoardTy, A: SquareSetTy, MoverC: ColorEn, S: SquareTy> RunSqIsCheck<A, MoverC, S> for B
where
    B: RunIdxBoard<S>,
    A: RunIsOccupied<S>,
    (): RunSqIsCheckWC<MoverC, IsOccupied<A, S>, IdxBoard<B, S>>,
{
    type Output = <() as RunSqIsCheckWC<MoverC, IsOccupied<A, S>, IdxBoard<B, S>>>::Output;
}

pub(crate) trait RunSqIsCheckWC<MoverC: ColorEn, IsAttacked: Bool, C: CellEn> {
    type Output: Bool;
}

impl<MoverC: ColorEn, C: CellEn> RunSqIsCheckWC<MoverC, False, C> for () {
    type Output = False;
}
impl<MoverC: ColorEn> RunSqIsCheckWC<MoverC, True, Empty> for () {
    type Output = False;
}
impl<P: PieceEn, MoverC: ColorEn> RunSqIsCheckWC<MoverC, True, Filled<ColoredPiece<P, MoverC>>>
    for ()
{
    type Output = False;
}
impl RunSqIsCheckWC<White, True, Filled<ColoredPiece<King, Black>>> for () {
    type Output = True;
}
impl RunSqIsCheckWC<Black, True, Filled<ColoredPiece<King, White>>> for () {
    type Output = True;
}
impl RunSqIsCheckWC<White, True, Filled<ColoredPiece<Pawn, Black>>> for () {
    type Output = False;
}
impl RunSqIsCheckWC<Black, True, Filled<ColoredPiece<Pawn, White>>> for () {
    type Output = False;
}
impl RunSqIsCheckWC<White, True, Filled<ColoredPiece<Knight, Black>>> for () {
    type Output = False;
}
impl RunSqIsCheckWC<Black, True, Filled<ColoredPiece<Knight, White>>> for () {
    type Output = False;
}
impl RunSqIsCheckWC<White, True, Filled<ColoredPiece<Bishop, Black>>> for () {
    type Output = False;
}
impl RunSqIsCheckWC<Black, True, Filled<ColoredPiece<Bishop, White>>> for () {
    type Output = False;
}
impl RunSqIsCheckWC<White, True, Filled<ColoredPiece<Rook, Black>>> for () {
    type Output = False;
}
impl RunSqIsCheckWC<Black, True, Filled<ColoredPiece<Rook, White>>> for () {
    type Output = False;
}
impl RunSqIsCheckWC<White, True, Filled<ColoredPiece<Queen, Black>>> for () {
    type Output = False;
}
impl RunSqIsCheckWC<Black, True, Filled<ColoredPiece<Queen, White>>> for () {
    type Output = False;
}
