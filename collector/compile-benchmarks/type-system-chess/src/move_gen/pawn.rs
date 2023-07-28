use super::{
    list::{
        AppendMaybeMove, AppendMaybeSquare, MLCons, MoveListTy, RunAppendMaybeMove,
        RunAppendMaybeSquare, SLCons, SLNil, SquareListTy,
    },
    MaybeMove, Move, NoMove, SomeMove,
};
use crate::{
    board_rep::{
        board::{
            idx::{IdxBoard, RunIdxBoard},
            BoardTy, CellEn, Empty, Filled,
        },
        color::{Black, ColorEn, White},
        piece::{Bishop, ColoredPiece, ColoredPieceTy, Knight, Pawn, PieceEn, Queen, Rook},
        square::{
            file::FileEn,
            offset::{
                MaybeSquare, Neg1, NoSquare, Offset, Offset1DEn, OffsetSquare, OffsetTy, Pos1,
                RunOffsetSquare, SomeSquare, Zero,
            },
            rank::{self, RankEn},
            RunSquareEq, Square, SquareEq, SquareTy,
        },
    },
    util::{Bool, False, True},
};

pub(crate) trait RunForwardO<F: Offset1DEn>: ColorEn {
    type Output: OffsetTy;
}
pub(crate) type ForwardO<C, F> = <C as RunForwardO<F>>::Output;
impl<F: Offset1DEn> RunForwardO<F> for White {
    type Output = Offset<Pos1, F>;
}
impl<F: Offset1DEn> RunForwardO<F> for Black {
    type Output = Offset<Neg1, F>;
}

pub(crate) trait RunPawnMoves<B: BoardTy, MoverC: ColorEn, EP: MaybeSquare, ML: MoveListTy>:
    SquareTy
{
    type Output: MoveListTy;
}
pub(crate) type PawnMoves<S, B, MoverC, EP, ML> = <S as RunPawnMoves<B, MoverC, EP, ML>>::Output;

impl<S: SquareTy, B: BoardTy, MoverC: ColorEn, EP: MaybeSquare, ML: MoveListTy>
    RunPawnMoves<B, MoverC, EP, ML> for S
where
    S: RunPawnMoveSqs<B, MoverC>,
    S: RunMakeEPMove<MoverC, Pos1, EP> + RunMakeEPMove<MoverC, Neg1, EP>,
    PawnMoveSqs<S, B, MoverC>: RunPawnMlFromSL<B, S, MoverC, ML>,
    PawnMLFromSl<PawnMoveSqs<S, B, MoverC>, B, S, MoverC, ML>:
        RunAppendMaybeMove<MakeEPMove<S, MoverC, Pos1, EP>>,
    AppendMaybeMove<
        PawnMLFromSl<PawnMoveSqs<S, B, MoverC>, B, S, MoverC, ML>,
        MakeEPMove<S, MoverC, Pos1, EP>,
    >: RunAppendMaybeMove<MakeEPMove<S, MoverC, Neg1, EP>>,
{
    type Output = AppendMaybeMove<
        AppendMaybeMove<
            PawnMLFromSl<PawnMoveSqs<S, B, MoverC>, B, S, MoverC, ML>,
            MakeEPMove<S, MoverC, Pos1, EP>,
        >,
        MakeEPMove<S, MoverC, Neg1, EP>,
    >;
}

pub(crate) trait RunPawnMoveSqs<B: BoardTy, MoverC: ColorEn>: SquareTy {
    type Output: SquareListTy;
}
pub(crate) type PawnMoveSqs<S, B, MoverC> = <S as RunPawnMoveSqs<B, MoverC>>::Output;

type S1<B, MoverC, S> = Cap<OffsetSquare<S, ForwardO<MoverC, Neg1>>, B, MoverC>;
type S2<B, MoverC, S> = Cap<OffsetSquare<S, ForwardO<MoverC, Pos1>>, B, MoverC>;
type S3<B, MoverC, S> = Forward1<OffsetSquare<S, ForwardO<MoverC, Zero>>, B, MoverC>;
type S4<B, MoverC, S> =
    Forward2<YesIfDoubleSq<S, OffsetSquare<S, ForwardO<MoverC, Zero>>>, B, MoverC>;

type L1<B, MoverC, S> = AppendMaybeSquare<SLNil, S1<B, MoverC, S>>;
type L2<B, MoverC, S> = AppendMaybeSquare<L1<B, MoverC, S>, S2<B, MoverC, S>>;
type L3<B, MoverC, S> = AppendMaybeSquare<L2<B, MoverC, S>, S3<B, MoverC, S>>;
type L4<B, MoverC, S> = AppendMaybeSquare<L3<B, MoverC, S>, S4<B, MoverC, S>>;

impl<B: BoardTy, MoverC: ColorEn, S: SquareTy> RunPawnMoveSqs<B, MoverC> for S
where
    MoverC: RunForwardO<Neg1>,
    MoverC: RunForwardO<Zero>,
    MoverC: RunForwardO<Pos1>,
    S: RunOffsetSquare<ForwardO<MoverC, Neg1>>,
    S: RunOffsetSquare<ForwardO<MoverC, Zero>>,
    S: RunOffsetSquare<ForwardO<MoverC, Pos1>>,
    OffsetSquare<S, ForwardO<MoverC, Neg1>>: RunCap<B, MoverC>,
    OffsetSquare<S, ForwardO<MoverC, Zero>>: RunForward1<B, MoverC>,
    S: RunYesIfDoubleSq<OffsetSquare<S, ForwardO<MoverC, Zero>>>,
    YesIfDoubleSq<S, OffsetSquare<S, ForwardO<MoverC, Zero>>>: RunForward2<B, MoverC>,
    OffsetSquare<S, ForwardO<MoverC, Pos1>>: RunCap<B, MoverC>,
    SLNil: RunAppendMaybeSquare<S1<B, MoverC, S>>,
    L1<B, MoverC, S>: RunAppendMaybeSquare<S2<B, MoverC, S>>,
    L2<B, MoverC, S>: RunAppendMaybeSquare<S3<B, MoverC, S>>,
    L3<B, MoverC, S>: RunAppendMaybeSquare<S4<B, MoverC, S>>,
{
    type Output = L4<B, MoverC, S>;
}

pub(crate) trait RunCap<B: BoardTy, MoverC: ColorEn>: MaybeSquare {
    type Output: MaybeSquare;
}
pub(crate) type Cap<S, B, MoverC> = <S as RunCap<B, MoverC>>::Output;

impl<B: BoardTy, MoverC: ColorEn> RunCap<B, MoverC> for NoSquare {
    type Output = NoSquare;
}
impl<B: BoardTy, MoverC: ColorEn, S: SquareTy> RunCap<B, MoverC> for SomeSquare<S>
where
    B: RunIdxBoard<S>,
    IdxBoard<B, S>: RunYesIfOpp<S, MoverC>,
{
    type Output = YesIfOpp<IdxBoard<B, S>, S, MoverC>;
}

pub(crate) trait RunForward1<B: BoardTy, MoverC: ColorEn>: MaybeSquare {
    type Output: MaybeSquare;
}
pub(crate) type Forward1<S, B, MoverC> = <S as RunForward1<B, MoverC>>::Output;

impl<B: BoardTy, MoverC: ColorEn> RunForward1<B, MoverC> for NoSquare {
    type Output = NoSquare;
}
impl<B: BoardTy, MoverC: ColorEn, S: SquareTy> RunForward1<B, MoverC> for SomeSquare<S>
where
    B: RunIdxBoard<S>,
    IdxBoard<B, S>: RunYesIfEmpty<SomeSquare<S>>,
{
    type Output = YesIfEmpty<IdxBoard<B, S>, SomeSquare<S>>;
}

pub(crate) trait RunForward2<B: BoardTy, MoverC: ColorEn>: MaybeSquare {
    type Output: MaybeSquare;
}
pub(crate) type Forward2<S, B, MoverC> = <S as RunForward2<B, MoverC>>::Output;

impl<B: BoardTy, MoverC: ColorEn> RunForward2<B, MoverC> for NoSquare {
    type Output = NoSquare;
}
impl<B: BoardTy, MoverC: ColorEn, S: SquareTy> RunForward2<B, MoverC> for SomeSquare<S>
where
    MoverC: RunForwardO<Zero>,
    S: RunOffsetSquare<ForwardO<MoverC, Zero>>,
    OffsetSquare<S, ForwardO<MoverC, Zero>>: RunForward1<B, MoverC>,
    B: RunIdxBoard<S>,
    IdxBoard<B, S>: RunYesIfEmpty<Forward1<OffsetSquare<S, ForwardO<MoverC, Zero>>, B, MoverC>>,
{
    type Output =
        YesIfEmpty<IdxBoard<B, S>, Forward1<OffsetSquare<S, ForwardO<MoverC, Zero>>, B, MoverC>>;
}

pub(crate) trait RunMakeEPMove<MoverC: ColorEn, FOff: Offset1DEn, EP: MaybeSquare>:
    SquareTy
{
    type Output: MaybeMove;
}
pub(crate) type MakeEPMove<S, MoverC, FOff, EP> = <S as RunMakeEPMove<MoverC, FOff, EP>>::Output;

impl<MoverC: ColorEn, FOff: Offset1DEn, S: SquareTy> RunMakeEPMove<MoverC, FOff, NoSquare> for S {
    type Output = NoMove;
}
impl<MoverC: ColorEn, FOff: Offset1DEn, EPS: SquareTy, S: SquareTy>
    RunMakeEPMove<MoverC, FOff, SomeSquare<EPS>> for S
where
    MoverC: RunForwardO<FOff>,
    S: RunOffsetSquare<ForwardO<MoverC, FOff>>,
    EPS: RunEp<OffsetSquare<S, ForwardO<MoverC, FOff>>>,
    S: RunEpMove<MoverC, FOff, Ep<EPS, OffsetSquare<S, ForwardO<MoverC, FOff>>>>,
{
    type Output =
        <S as RunEpMove<MoverC, FOff, Ep<EPS, OffsetSquare<S, ForwardO<MoverC, FOff>>>>>::Output;
}

pub(crate) trait RunEpMove<MoverC: ColorEn, FOff: Offset1DEn, End: MaybeSquare>:
    SquareTy
{
    type Output: MaybeMove;
}

impl<MoverC: ColorEn, FOff: Offset1DEn, Start: SquareTy> RunEpMove<MoverC, FOff, NoSquare>
    for Start
{
    type Output = NoMove;
}
impl<MoverC: ColorEn, FOff: Offset1DEn, Start: SquareTy, End: SquareTy>
    RunEpMove<MoverC, FOff, SomeSquare<End>> for Start
where
    Start: RunOffsetSquare<Offset<Zero, FOff>>,
{
    type Output = SomeMove<
        Move<Start, End, ColoredPiece<Pawn, MoverC>, OffsetSquare<Start, Offset<Zero, FOff>>>,
    >;
}

pub(crate) trait RunEp<EP: MaybeSquare>: SquareTy {
    type Output: MaybeSquare;
}
pub(crate) type Ep<S, EP> = <S as RunEp<EP>>::Output;

impl<S: SquareTy> RunEp<NoSquare> for S {
    type Output = NoSquare;
}
impl<S1: SquareTy, S2: SquareTy> RunEp<SomeSquare<S1>> for S2
where
    S1: RunSquareEq<S2>,
    S1: RunEpWMatches<SquareEq<S1, S2>>,
{
    type Output = <S1 as RunEpWMatches<SquareEq<S1, S2>>>::Output;
}

pub(crate) trait RunEpWMatches<Matches: Bool>: SquareTy {
    type Output: MaybeSquare;
}
impl<S: SquareTy> RunEpWMatches<True> for S {
    type Output = SomeSquare<S>;
}
impl<S: SquareTy> RunEpWMatches<False> for S {
    type Output = NoSquare;
}

pub(crate) trait RunYesIfDoubleSq<S: MaybeSquare>: SquareTy {
    type Output: MaybeSquare;
}
pub(crate) type YesIfDoubleSq<TS, US> = <TS as RunYesIfDoubleSq<US>>::Output;

impl<F: FileEn, US: MaybeSquare> RunYesIfDoubleSq<US> for Square<rank::R1, F> {
    type Output = NoSquare;
}
impl<F: FileEn, US: MaybeSquare> RunYesIfDoubleSq<US> for Square<rank::R2, F> {
    type Output = US;
}
impl<F: FileEn, US: MaybeSquare> RunYesIfDoubleSq<US> for Square<rank::R3, F> {
    type Output = NoSquare;
}
impl<F: FileEn, US: MaybeSquare> RunYesIfDoubleSq<US> for Square<rank::R4, F> {
    type Output = NoSquare;
}
impl<F: FileEn, US: MaybeSquare> RunYesIfDoubleSq<US> for Square<rank::R5, F> {
    type Output = NoSquare;
}
impl<F: FileEn, US: MaybeSquare> RunYesIfDoubleSq<US> for Square<rank::R6, F> {
    type Output = NoSquare;
}
impl<F: FileEn, US: MaybeSquare> RunYesIfDoubleSq<US> for Square<rank::R7, F> {
    type Output = US;
}
impl<F: FileEn, US: MaybeSquare> RunYesIfDoubleSq<US> for Square<rank::R8, F> {
    type Output = NoSquare;
}

pub(crate) trait RunYesIfOpp<End: SquareTy, MoverC: ColorEn>: CellEn {
    type Output: MaybeSquare;
}
pub(crate) type YesIfOpp<C, E, MoverC> = <C as RunYesIfOpp<E, MoverC>>::Output;

impl<E: SquareTy, MoverC: ColorEn> RunYesIfOpp<E, MoverC> for Empty {
    type Output = NoSquare;
}
impl<E: SquareTy, P: PieceEn> RunYesIfOpp<E, White> for Filled<ColoredPiece<P, White>> {
    type Output = NoSquare;
}
impl<E: SquareTy, P: PieceEn> RunYesIfOpp<E, Black> for Filled<ColoredPiece<P, Black>> {
    type Output = NoSquare;
}
impl<E: SquareTy, P: PieceEn> RunYesIfOpp<E, White> for Filled<ColoredPiece<P, Black>> {
    type Output = SomeSquare<E>;
}
impl<E: SquareTy, P: PieceEn> RunYesIfOpp<E, Black> for Filled<ColoredPiece<P, White>> {
    type Output = SomeSquare<E>;
}

pub(crate) trait RunYesIfEmpty<End: MaybeSquare>: CellEn {
    type Output: MaybeSquare;
}
pub(crate) type YesIfEmpty<C, E> = <C as RunYesIfEmpty<E>>::Output;

impl<E: SquareTy> RunYesIfEmpty<SomeSquare<E>> for Empty {
    type Output = SomeSquare<E>;
}
impl RunYesIfEmpty<NoSquare> for Empty {
    type Output = NoSquare;
}
impl<E: MaybeSquare, CP: ColoredPieceTy> RunYesIfEmpty<E> for Filled<CP> {
    type Output = NoSquare;
}

// Square list is already only the pseudo legal moves. This is just for adding promotions.
pub(crate) trait RunPawnMlFromSL<B: BoardTy, Start: SquareTy, MoverC: ColorEn, ML: MoveListTy>:
    SquareListTy
{
    type Output: MoveListTy;
}
pub(crate) type PawnMLFromSl<SL, B, Start, MoverC, ML> =
    <SL as RunPawnMlFromSL<B, Start, MoverC, ML>>::Output;

impl<B: BoardTy, Start: SquareTy, MoverC: ColorEn, ML: MoveListTy>
    RunPawnMlFromSL<B, Start, MoverC, ML> for SLNil
{
    type Output = ML;
}
impl<
        B: BoardTy,
        Start: SquareTy,
        MoverC: ColorEn,
        ML: MoveListTy,
        S: SquareTy,
        Next: SquareListTy,
    > RunPawnMlFromSL<B, Start, MoverC, ML> for SLCons<S, Next>
where
    Next: RunPawnMlFromSL<B, Start, MoverC, ML>,
    (): RunPawnMFromS<B, Start, MoverC, S, PawnMLFromSl<Next, B, Start, MoverC, ML>>,
{
    type Output = PawnMFromS<S, B, Start, MoverC, PawnMLFromSl<Next, B, Start, MoverC, ML>>;
}

pub(crate) trait RunPawnMFromS<
    B: BoardTy,
    Start: SquareTy,
    MoverC: ColorEn,
    End: SquareTy,
    ML: MoveListTy,
>
{
    type Output: MoveListTy;
}
pub(crate) type PawnMFromS<End, B, Start, MoverC, ML> =
    <() as RunPawnMFromS<B, Start, MoverC, End, ML>>::Output;

impl<B: BoardTy, Start: SquareTy, MoverC: ColorEn, R: RankEn, F: FileEn, ML: MoveListTy>
    RunPawnMFromS<B, Start, MoverC, Square<R, F>, ML> for ()
where
    (): RunPawnMFromRF<B, Start, MoverC, R, F, ML>,
{
    type Output = <() as RunPawnMFromRF<B, Start, MoverC, R, F, ML>>::Output;
}

pub(crate) trait RunPawnMFromRF<
    B: BoardTy,
    Start: SquareTy,
    MoverC: ColorEn,
    EndR: RankEn,
    EndF: FileEn,
    ML: MoveListTy,
>
{
    type Output: MoveListTy;
}

// Promotions
impl<B: BoardTy, Start: SquareTy, MoverC: ColorEn, F: FileEn, ML: MoveListTy>
    RunPawnMFromRF<B, Start, MoverC, rank::R1, F, ML> for ()
{
    type Output = MLCons<
        Move<Start, Square<rank::R1, F>, ColoredPiece<Bishop, MoverC>>,
        MLCons<
            Move<Start, Square<rank::R1, F>, ColoredPiece<Knight, MoverC>>,
            MLCons<
                Move<Start, Square<rank::R1, F>, ColoredPiece<Rook, MoverC>>,
                MLCons<Move<Start, Square<rank::R1, F>, ColoredPiece<Queen, MoverC>>, ML>,
            >,
        >,
    >;
}
impl<B: BoardTy, Start: SquareTy, MoverC: ColorEn, F: FileEn, ML: MoveListTy>
    RunPawnMFromRF<B, Start, MoverC, rank::R8, F, ML> for ()
{
    type Output = MLCons<
        Move<Start, Square<rank::R8, F>, ColoredPiece<Bishop, MoverC>>,
        MLCons<
            Move<Start, Square<rank::R8, F>, ColoredPiece<Knight, MoverC>>,
            MLCons<
                Move<Start, Square<rank::R8, F>, ColoredPiece<Rook, MoverC>>,
                MLCons<Move<Start, Square<rank::R8, F>, ColoredPiece<Queen, MoverC>>, ML>,
            >,
        >,
    >;
}

// Non Promotions
impl<B: BoardTy, Start: SquareTy, MoverC: ColorEn, F: FileEn, ML: MoveListTy>
    RunPawnMFromRF<B, Start, MoverC, rank::R2, F, ML> for ()
{
    type Output = MLCons<Move<Start, Square<rank::R2, F>, ColoredPiece<Pawn, MoverC>>, ML>;
}
impl<B: BoardTy, Start: SquareTy, MoverC: ColorEn, F: FileEn, ML: MoveListTy>
    RunPawnMFromRF<B, Start, MoverC, rank::R3, F, ML> for ()
{
    type Output = MLCons<Move<Start, Square<rank::R3, F>, ColoredPiece<Pawn, MoverC>>, ML>;
}
impl<B: BoardTy, Start: SquareTy, MoverC: ColorEn, F: FileEn, ML: MoveListTy>
    RunPawnMFromRF<B, Start, MoverC, rank::R4, F, ML> for ()
{
    type Output = MLCons<Move<Start, Square<rank::R4, F>, ColoredPiece<Pawn, MoverC>>, ML>;
}
impl<B: BoardTy, Start: SquareTy, MoverC: ColorEn, F: FileEn, ML: MoveListTy>
    RunPawnMFromRF<B, Start, MoverC, rank::R5, F, ML> for ()
{
    type Output = MLCons<Move<Start, Square<rank::R5, F>, ColoredPiece<Pawn, MoverC>>, ML>;
}
impl<B: BoardTy, Start: SquareTy, MoverC: ColorEn, F: FileEn, ML: MoveListTy>
    RunPawnMFromRF<B, Start, MoverC, rank::R6, F, ML> for ()
{
    type Output = MLCons<Move<Start, Square<rank::R6, F>, ColoredPiece<Pawn, MoverC>>, ML>;
}
impl<B: BoardTy, Start: SquareTy, MoverC: ColorEn, F: FileEn, ML: MoveListTy>
    RunPawnMFromRF<B, Start, MoverC, rank::R7, F, ML> for ()
{
    type Output = MLCons<Move<Start, Square<rank::R7, F>, ColoredPiece<Pawn, MoverC>>, ML>;
}

pub(crate) trait RunPawnAttackSqs<MoverC: ColorEn>: SquareTy {
    type Output: SquareListTy;
}
pub(crate) type PawnAttackSqs<S, C> = <S as RunPawnAttackSqs<C>>::Output;

type AL1<S, C> = AppendMaybeSquare<SLNil, OffsetSquare<S, ForwardO<C, Neg1>>>;
type AL2<S, C> = AppendMaybeSquare<AL1<S, C>, OffsetSquare<S, ForwardO<C, Pos1>>>;

impl<S: SquareTy, MoverC: ColorEn> RunPawnAttackSqs<MoverC> for S
where
    MoverC: RunForwardO<Neg1> + RunForwardO<Pos1>,
    S: RunOffsetSquare<ForwardO<MoverC, Neg1>> + RunOffsetSquare<ForwardO<MoverC, Pos1>>,
    SLNil: RunAppendMaybeSquare<OffsetSquare<S, ForwardO<MoverC, Neg1>>>,
    AL1<S, MoverC>: RunAppendMaybeSquare<OffsetSquare<S, ForwardO<MoverC, Pos1>>>,
{
    type Output = AL2<S, MoverC>;
}
