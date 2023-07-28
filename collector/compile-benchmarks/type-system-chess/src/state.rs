use crate::{
    board_rep::{
        board::{
            write::{MaybeWriteToBoard, RunMaybeWriteToBoard, RunWriteToBoard, WriteToBoard},
            BoardTy, Empty, Filled,
        },
        color::{Black, ColorEn, White},
        piece::{Bishop, ColoredPiece, ColoredPieceTy, King, Knight, Pawn, Queen, Rook},
        square::{
            file,
            offset::{
                MaybeSquare, Neg1, Neg2, NoSquare, Offset, OffsetSquare, OffsetTy, Pos1, Pos2,
                RunOffsetSquare, SomeSquare, Zero,
            },
            rank, RunSquareEq, Square, SquareEq, SquareTy,
        },
    },
    move_gen::{Move, MoveTy},
    util::{And, Bool, False, Not, Or, RunAnd, RunNot, RunOr, True},
    values,
};
use std::marker::PhantomData;

pub mod outcome;

pub(crate) trait StateTy {
    fn reify() -> values::State;
}
pub(crate) struct State<ToMove: ColorEn, Pieces: BoardTy, EPSquare: MaybeSquare, CA: CastleStateTy>(
    PhantomData<(ToMove, Pieces, EPSquare, CA)>,
);

pub(crate) trait CastleStateTy {
    fn reify() -> values::CastleState;
}
pub(crate) struct CastleState<WK: Bool, WQ: Bool, BK: Bool, BQ: Bool>(
    PhantomData<(WK, WQ, BK, BQ)>,
);

pub(crate) type FullCa = CastleState<True, True, True, True>;
pub(crate) type EmptyCa = CastleState<False, False, False, False>;

impl<WK: Bool, WQ: Bool, BK: Bool, BQ: Bool> CastleStateTy for CastleState<WK, WQ, BK, BQ> {
    fn reify() -> values::CastleState {
        values::CastleState {
            wk: WK::reify(),
            wq: WQ::reify(),
            bk: BK::reify(),
            bq: BQ::reify(),
        }
    }
}

impl<ToMove: ColorEn, Pieces: BoardTy, EPSquare: MaybeSquare, CA: CastleStateTy> StateTy
    for State<ToMove, Pieces, EPSquare, CA>
{
    fn reify() -> values::State {
        values::State {
            to_move: ToMove::reify(),
            pieces: Pieces::reify(),
            ep_square: EPSquare::reify(),
            castle_state: CA::reify(),
        }
    }
}

pub(crate) trait RunMakeMove<S: StateTy>: MoveTy {
    type Output: StateTy;
}
pub(crate) type MakeMove<M, S> = <M as RunMakeMove<S>>::Output;

impl<M: MoveTy, B: BoardTy, EPSquare: MaybeSquare, CA: CastleStateTy>
    RunMakeMove<State<White, B, EPSquare, CA>> for M
where
    M: RunMakeMoveForBoard<B, White>,
    M: RunMakeMoveForEp,
    M: RunMakeMoveForCa<CA>,
{
    type Output =
        State<Black, MakeMoveForBoard<M, B, White>, MakeMoveForEP<M>, MakeMoveForCa<M, CA>>;
}
impl<M: MoveTy, B: BoardTy, EPSquare: MaybeSquare, CA: CastleStateTy>
    RunMakeMove<State<Black, B, EPSquare, CA>> for M
where
    M: RunMakeMoveForBoard<B, Black>,
    M: RunMakeMoveForEp,
    M: RunMakeMoveForCa<CA>,
{
    type Output =
        State<White, MakeMoveForBoard<M, B, Black>, MakeMoveForEP<M>, MakeMoveForCa<M, CA>>;
}

pub(crate) trait RunMakeMoveForBoard<B: BoardTy, MoverC: ColorEn>: MoveTy {
    type Output: BoardTy;
}
pub(crate) type MakeMoveForBoard<M, B, MoverC> = <M as RunMakeMoveForBoard<B, MoverC>>::Output;

impl<
        B: BoardTy,
        MoverC: ColorEn,
        F: SquareTy,
        T: SquareTy,
        P: ColoredPieceTy,
        EP: MaybeSquare,
        RFrom: MaybeSquare,
        RTo: MaybeSquare,
    > RunMakeMoveForBoard<B, MoverC> for Move<F, T, P, EP, RFrom, RTo>
where
    B: RunWriteToBoard<T, Filled<P>>,
    WriteToBoard<B, T, Filled<P>>: RunWriteToBoard<F, Empty>,
    WriteToBoard<WriteToBoard<B, T, Filled<P>>, F, Empty>: RunMaybeWriteToBoard<EP, Empty>,
    MaybeWriteToBoard<WriteToBoard<WriteToBoard<B, T, Filled<P>>, F, Empty>, EP, Empty>:
        RunMaybeWriteToBoard<RFrom, Empty>,
    MaybeWriteToBoard<
        MaybeWriteToBoard<WriteToBoard<WriteToBoard<B, T, Filled<P>>, F, Empty>, EP, Empty>,
        RFrom,
        Empty,
    >: RunMaybeWriteToBoard<RTo, Filled<ColoredPiece<Rook, MoverC>>>,
{
    type Output = MaybeWriteToBoard<
        MaybeWriteToBoard<
            MaybeWriteToBoard<WriteToBoard<WriteToBoard<B, T, Filled<P>>, F, Empty>, EP, Empty>,
            RFrom,
            Empty,
        >,
        RTo,
        Filled<ColoredPiece<Rook, MoverC>>,
    >;
}

pub(crate) trait RunMakeMoveForEp: MoveTy {
    type Output: MaybeSquare;
}
pub(crate) type MakeMoveForEP<M> = <M as RunMakeMoveForEp>::Output;

impl<F: SquareTy, T: SquareTy, MoverC: ColorEn> RunMakeMoveForEp
    for Move<F, T, ColoredPiece<Bishop, MoverC>>
{
    type Output = NoSquare;
}
impl<F: SquareTy, T: SquareTy, MoverC: ColorEn> RunMakeMoveForEp
    for Move<F, T, ColoredPiece<Knight, MoverC>>
{
    type Output = NoSquare;
}
impl<F: SquareTy, T: SquareTy, MoverC: ColorEn> RunMakeMoveForEp
    for Move<F, T, ColoredPiece<Rook, MoverC>>
{
    type Output = NoSquare;
}
impl<F: SquareTy, T: SquareTy, MoverC: ColorEn> RunMakeMoveForEp
    for Move<F, T, ColoredPiece<Queen, MoverC>>
{
    type Output = NoSquare;
}
impl<F: SquareTy, T: SquareTy, MoverC: ColorEn, RF: MaybeSquare, RT: MaybeSquare> RunMakeMoveForEp
    for Move<F, T, ColoredPiece<King, MoverC>, NoSquare, RF, RT>
{
    type Output = NoSquare;
}
impl<F: SquareTy, T: SquareTy, MoverC: ColorEn, EP: MaybeSquare> RunMakeMoveForEp
    for Move<F, T, ColoredPiece<Pawn, MoverC>, EP>
where
    MoverC: RunForward1O + RunForward2O,
    F: RunOffsetSquare<Forward2O<MoverC>> + RunOffsetSquare<Forward1O<MoverC>>,
    T: RunMaybeSqEq<OffsetSquare<F, Forward2O<MoverC>>>,
    OffsetSquare<F, Forward1O<MoverC>>:
        RunSelectIf<MaybeSqEq<T, OffsetSquare<F, Forward2O<MoverC>>>>,
{
    type Output = <OffsetSquare<F, Forward1O<MoverC>> as RunSelectIf<
        MaybeSqEq<T, OffsetSquare<F, Forward2O<MoverC>>>,
    >>::Output;
}

pub(crate) trait RunMaybeSqEq<MS: MaybeSquare>: SquareTy {
    type Output: Bool;
}
pub(crate) type MaybeSqEq<S, MS> = <S as RunMaybeSqEq<MS>>::Output;

impl<S: SquareTy> RunMaybeSqEq<NoSquare> for S {
    type Output = False;
}
impl<S1: SquareTy, S2: SquareTy> RunMaybeSqEq<SomeSquare<S1>> for S2
where
    S1: RunSquareEq<S2>,
{
    type Output = SquareEq<S1, S2>;
}

pub(crate) trait RunForward2O: ColorEn {
    type Output: OffsetTy;
}
pub(crate) type Forward2O<C> = <C as RunForward2O>::Output;
impl RunForward2O for White {
    type Output = Offset<Pos2, Zero>;
}
impl RunForward2O for Black {
    type Output = Offset<Neg2, Zero>;
}

pub(crate) trait RunForward1O: ColorEn {
    type Output: OffsetTy;
}
pub(crate) type Forward1O<C> = <C as RunForward1O>::Output;
impl RunForward1O for White {
    type Output = Offset<Pos1, Zero>;
}
impl RunForward1O for Black {
    type Output = Offset<Neg1, Zero>;
}

pub(crate) trait RunSelectIf<B: Bool>: MaybeSquare {
    type Output: MaybeSquare;
}

impl<B: Bool> RunSelectIf<B> for NoSquare {
    type Output = NoSquare;
}
impl<S: SquareTy> RunSelectIf<False> for SomeSquare<S> {
    type Output = NoSquare;
}
impl<S: SquareTy> RunSelectIf<True> for SomeSquare<S> {
    type Output = Self;
}

pub(crate) trait RunMakeMoveForCa<CA: CastleStateTy>: MoveTy {
    type Output: CastleStateTy;
}
pub(crate) type MakeMoveForCa<M, CA> = <M as RunMakeMoveForCa<CA>>::Output;

type A1 = Square<rank::R1, file::FA>;
type E1 = Square<rank::R1, file::FE>;
type H1 = Square<rank::R1, file::FH>;
type A8 = Square<rank::R8, file::FA>;
type E8 = Square<rank::R8, file::FE>;
type H8 = Square<rank::R8, file::FH>;

impl<
        CA: CastleStateTy,
        F: SquareTy,
        T: SquareTy,
        CP: ColoredPieceTy,
        EP: MaybeSquare,
        RF: MaybeSquare,
        RT: MaybeSquare,
    > RunMakeMoveForCa<CA> for Move<F, T, CP, EP, RF, RT>
where
    T: RunSquareEq<A1> + RunSquareEq<H1> + RunSquareEq<A8> + RunSquareEq<H8>,
    F: RunSquareEq<A1>
        + RunSquareEq<H1>
        + RunSquareEq<E1>
        + RunSquareEq<A8>
        + RunSquareEq<H8>
        + RunSquareEq<E8>,
    SquareEq<F, A1>: RunOr<SquareEq<T, A1>>,
    SquareEq<F, E1>: RunOr<Or<SquareEq<F, A1>, SquareEq<T, A1>>>,
    SquareEq<F, H1>: RunOr<SquareEq<T, H1>>,
    SquareEq<F, E1>: RunOr<Or<SquareEq<F, H1>, SquareEq<T, H1>>>,
    SquareEq<F, A8>: RunOr<SquareEq<T, A8>>,
    SquareEq<F, E8>: RunOr<Or<SquareEq<F, A8>, SquareEq<T, A8>>>,
    SquareEq<F, H8>: RunOr<SquareEq<T, H8>>,
    SquareEq<F, E8>: RunOr<Or<SquareEq<F, H8>, SquareEq<T, H8>>>,
    CA: RunMakeMoveForCaWSqsTouched<
        Or<SquareEq<F, E1>, Or<SquareEq<F, H1>, SquareEq<T, H1>>>,
        Or<SquareEq<F, E1>, Or<SquareEq<F, A1>, SquareEq<T, A1>>>,
        Or<SquareEq<F, E8>, Or<SquareEq<F, H8>, SquareEq<T, H8>>>,
        Or<SquareEq<F, E8>, Or<SquareEq<F, A8>, SquareEq<T, A8>>>,
    >,
{
    type Output = MakeMoveForCaWSqsTouched<
        CA,
        Or<SquareEq<F, E1>, Or<SquareEq<F, H1>, SquareEq<T, H1>>>,
        Or<SquareEq<F, E1>, Or<SquareEq<F, A1>, SquareEq<T, A1>>>,
        Or<SquareEq<F, E8>, Or<SquareEq<F, H8>, SquareEq<T, H8>>>,
        Or<SquareEq<F, E8>, Or<SquareEq<F, A8>, SquareEq<T, A8>>>,
    >;
}

pub(crate) trait RunMakeMoveForCaWSqsTouched<WK: Bool, WQ: Bool, BK: Bool, BQ: Bool>:
    CastleStateTy
{
    type Output: CastleStateTy;
}
pub(crate) type MakeMoveForCaWSqsTouched<CA, WK, WQ, BK, BQ> =
    <CA as RunMakeMoveForCaWSqsTouched<WK, WQ, BK, BQ>>::Output;

impl<WK1: Bool, WQ1: Bool, BK1: Bool, BQ1: Bool, WK2: Bool, WQ2: Bool, BK2: Bool, BQ2: Bool>
    RunMakeMoveForCaWSqsTouched<WK2, WQ2, BK2, BQ2> for CastleState<WK1, WQ1, BK1, BQ1>
where
    WK2: RunNot,
    WQ2: RunNot,
    BK2: RunNot,
    BQ2: RunNot,
    WK1: RunAnd<Not<WK2>>,
    WQ1: RunAnd<Not<WQ2>>,
    BK1: RunAnd<Not<BK2>>,
    BQ1: RunAnd<Not<BQ2>>,
{
    type Output =
        CastleState<And<WK1, Not<WK2>>, And<WQ1, Not<WQ2>>, And<BK1, Not<BK2>>, And<BQ1, Not<BQ2>>>;
}
