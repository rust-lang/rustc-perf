use super::{
    attacked::{Attacked, RunAttacked},
    MaybeMove, Move, MoveTy, NoMove, SomeMove,
};
use crate::{
    board_rep::{
        board::{
            idx::{IdxBoard, RunIdxBoard},
            BoardTy, CellEn, Empty, Filled,
        },
        color::{Black, ColorEn, White},
        piece::{ColoredPiece, ColoredPieceTy, King},
        square::{
            file,
            offset::{NoSquare, SomeSquare},
            rank::{self, RankEn},
            set::{IsOccupied, RunIsOccupied, SquareSetTy},
            Square, SquareTy,
        },
    },
    util::{And, Bool, False, Not, Or, RunAnd, RunNot, RunOr, True},
};

pub(crate) trait RunIsEmpty<S: SquareTy>: BoardTy {
    type Output: Bool;
}
pub(crate) type IsEmpty<B, S> = <B as RunIsEmpty<S>>::Output;

impl<B: BoardTy, S: SquareTy> RunIsEmpty<S> for B
where
    B: RunIdxBoard<S>,
    IdxBoard<B, S>: RunCellIsEmpty,
{
    type Output = <IdxBoard<B, S> as RunCellIsEmpty>::Output;
}

pub(crate) trait RunCellIsEmpty: CellEn {
    type Output: Bool;
}
impl RunCellIsEmpty for Empty {
    type Output = True;
}
impl<CP: ColoredPieceTy> RunCellIsEmpty for Filled<CP> {
    type Output = False;
}

pub(crate) type SA<R> = Square<R, file::FA>;
pub(crate) type SB<R> = Square<R, file::FB>;
pub(crate) type SC<R> = Square<R, file::FC>;
pub(crate) type SD<R> = Square<R, file::FD>;
pub(crate) type SE<R> = Square<R, file::FE>;
pub(crate) type SF<R> = Square<R, file::FF>;
pub(crate) type SG<R> = Square<R, file::FG>;
pub(crate) type SH<R> = Square<R, file::FH>;

pub(crate) trait RunMoveIfPasses<B: Bool>: MoveTy {
    type Output: MaybeMove;
}
pub(crate) type MoveIfPasses<M, B> = <M as RunMoveIfPasses<B>>::Output;

impl<M: MoveTy> RunMoveIfPasses<True> for M {
    type Output = SomeMove<M>;
}
impl<M: MoveTy> RunMoveIfPasses<False> for M {
    type Output = NoMove;
}

pub(crate) trait RunKingsideCastle<MoverC: ColorEn, WK: Bool, BK: Bool>: BoardTy {
    type Output: MaybeMove;
}
pub(crate) type KingsideCastle<B, MoverC, WK, BK> =
    <B as RunKingsideCastle<MoverC, WK, BK>>::Output;

impl<B: BoardTy, BK: Bool> RunKingsideCastle<White, False, BK> for B {
    type Output = NoMove;
}
impl<B: BoardTy, WK: Bool> RunKingsideCastle<Black, WK, False> for B {
    type Output = NoMove;
}
impl<B: BoardTy, BK: Bool> RunKingsideCastle<White, True, BK> for B
where
    B: RunAttacked<Black>,
    B: RunKingsideCastleWX<White, rank::R1, Attacked<B, Black>>,
{
    type Output = KingsideCastleWX<B, White, rank::R1, Attacked<B, Black>>;
}
impl<B: BoardTy, WK: Bool> RunKingsideCastle<Black, WK, True> for B
where
    B: RunAttacked<White>,
    B: RunKingsideCastleWX<Black, rank::R8, Attacked<B, White>>,
{
    type Output = KingsideCastleWX<B, Black, rank::R8, Attacked<B, White>>;
}

pub(crate) trait RunKingsideCastleWX<MoverC: ColorEn, R: RankEn, A: SquareSetTy>:
    BoardTy
{
    type Output: MaybeMove;
}
pub(crate) type KingsideCastleWX<B, MoverC, R, A> =
    <B as RunKingsideCastleWX<MoverC, R, A>>::Output;

impl<B: BoardTy, MoverC: ColorEn, R: RankEn, A: SquareSetTy> RunKingsideCastleWX<MoverC, R, A> for B
where
    B: RunIsEmpty<SF<R>> + RunIsEmpty<SG<R>>,
    A: RunIsOccupied<SE<R>> + RunIsOccupied<SF<R>> + RunIsOccupied<SG<R>>,
    IsEmpty<B, SF<R>>: RunAnd<IsEmpty<B, SG<R>>>,
    IsOccupied<A, SE<R>>: RunOr<IsOccupied<A, SF<R>>>,
    Or<IsOccupied<A, SE<R>>, IsOccupied<A, SF<R>>>: RunOr<IsOccupied<A, SG<R>>>,
    Or<Or<IsOccupied<A, SE<R>>, IsOccupied<A, SF<R>>>, IsOccupied<A, SG<R>>>: RunNot,
    And<IsEmpty<B, SF<R>>, IsEmpty<B, SG<R>>>:
        RunAnd<Not<Or<Or<IsOccupied<A, SE<R>>, IsOccupied<A, SF<R>>>, IsOccupied<A, SG<R>>>>>,
    Move<SE<R>, SG<R>, ColoredPiece<King, MoverC>, NoSquare, SomeSquare<SH<R>>, SomeSquare<SF<R>>>:
        RunMoveIfPasses<
            And<
                And<IsEmpty<B, SF<R>>, IsEmpty<B, SG<R>>>,
                Not<Or<Or<IsOccupied<A, SE<R>>, IsOccupied<A, SF<R>>>, IsOccupied<A, SG<R>>>>,
            >,
        >,
{
    type Output = MoveIfPasses<
        Move<
            SE<R>,
            SG<R>,
            ColoredPiece<King, MoverC>,
            NoSquare,
            SomeSquare<SH<R>>,
            SomeSquare<SF<R>>,
        >,
        And<
            And<IsEmpty<B, SF<R>>, IsEmpty<B, SG<R>>>,
            Not<Or<Or<IsOccupied<A, SE<R>>, IsOccupied<A, SF<R>>>, IsOccupied<A, SG<R>>>>,
        >,
    >;
}

pub(crate) trait RunQueensideCastle<MoverC: ColorEn, WQ: Bool, BQ: Bool>: BoardTy {
    type Output: MaybeMove;
}
pub(crate) type QueensideCastle<B, MoverC, WQ, BQ> =
    <B as RunQueensideCastle<MoverC, WQ, BQ>>::Output;

impl<B: BoardTy, BQ: Bool> RunQueensideCastle<White, False, BQ> for B {
    type Output = NoMove;
}
impl<B: BoardTy, WQ: Bool> RunQueensideCastle<Black, WQ, False> for B {
    type Output = NoMove;
}
impl<B: BoardTy, BQ: Bool> RunQueensideCastle<White, True, BQ> for B
where
    B: RunAttacked<Black>,
    B: RunQueensideCastleWX<White, rank::R1, Attacked<B, Black>>,
{
    type Output = QueensideCastleWX<B, White, rank::R1, Attacked<B, Black>>;
}
impl<B: BoardTy, WQ: Bool> RunQueensideCastle<Black, WQ, True> for B
where
    B: RunAttacked<White>,
    B: RunQueensideCastleWX<Black, rank::R8, Attacked<B, White>>,
{
    type Output = QueensideCastleWX<B, Black, rank::R8, Attacked<B, White>>;
}

pub(crate) trait RunQueensideCastleWX<MoverC: ColorEn, R: RankEn, A: SquareSetTy>:
    BoardTy
{
    type Output: MaybeMove;
}
pub(crate) type QueensideCastleWX<B, MoverC, R, A> =
    <B as RunQueensideCastleWX<MoverC, R, A>>::Output;

impl<B: BoardTy, MoverC: ColorEn, R: RankEn, A: SquareSetTy> RunQueensideCastleWX<MoverC, R, A>
    for B
where
    B: RunIsEmpty<SB<R>> + RunIsEmpty<SC<R>> + RunIsEmpty<SD<R>>,
    A: RunIsOccupied<SE<R>> + RunIsOccupied<SD<R>> + RunIsOccupied<SC<R>>,
    IsEmpty<B, SB<R>>: RunAnd<IsEmpty<B, SC<R>>>,
    And<IsEmpty<B, SB<R>>, IsEmpty<B, SC<R>>>: RunAnd<IsEmpty<B, SD<R>>>,
    IsOccupied<A, SE<R>>: RunOr<IsOccupied<A, SD<R>>>,
    Or<IsOccupied<A, SE<R>>, IsOccupied<A, SD<R>>>: RunOr<IsOccupied<A, SC<R>>>,
    Or<Or<IsOccupied<A, SE<R>>, IsOccupied<A, SD<R>>>, IsOccupied<A, SC<R>>>: RunNot,
    And<And<IsEmpty<B, SB<R>>, IsEmpty<B, SC<R>>>, IsEmpty<B, SD<R>>>:
        RunAnd<Not<Or<Or<IsOccupied<A, SE<R>>, IsOccupied<A, SD<R>>>, IsOccupied<A, SC<R>>>>>,
    Move<SE<R>, SC<R>, ColoredPiece<King, MoverC>, NoSquare, SomeSquare<SA<R>>, SomeSquare<SD<R>>>:
        RunMoveIfPasses<
            And<
                And<And<IsEmpty<B, SB<R>>, IsEmpty<B, SC<R>>>, IsEmpty<B, SD<R>>>,
                Not<Or<Or<IsOccupied<A, SE<R>>, IsOccupied<A, SD<R>>>, IsOccupied<A, SC<R>>>>,
            >,
        >,
{
    type Output = MoveIfPasses<
        Move<
            SE<R>,
            SC<R>,
            ColoredPiece<King, MoverC>,
            NoSquare,
            SomeSquare<SA<R>>,
            SomeSquare<SD<R>>,
        >,
        And<
            And<And<IsEmpty<B, SB<R>>, IsEmpty<B, SC<R>>>, IsEmpty<B, SD<R>>>,
            Not<Or<Or<IsOccupied<A, SE<R>>, IsOccupied<A, SD<R>>>, IsOccupied<A, SC<R>>>>,
        >,
    >;
}
