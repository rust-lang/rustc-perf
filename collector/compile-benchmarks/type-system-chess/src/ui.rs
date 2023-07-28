use crate::{
    board_rep::{
        color::ColorEn,
        piece::{ColoredPiece, ColoredPieceTy, PieceEn, PieceEq, RunPieceEq},
        square::{offset::MaybeSquare, RunSquareEq, SquareEq, SquareTy},
    },
    move_gen::{
        list::{MLCons, MLNil, MoveListTy},
        moves::{Moves, RunMoves},
        MaybeMove, Move, MoveTy, NoMove, SomeMove,
    },
    state::{
        outcome::{Checkmate, Draw, Ongoing, Outcome, OutcomeEn, RunOutcome},
        MakeMove, RunMakeMove, StateTy,
    },
    util::{And, Bool, False, RunAnd, True},
    values,
};
use std::marker::PhantomData;

pub(crate) mod prelude {
    #![allow(unused)]

    pub(crate) use super::{MakeEM, MakeEMResultEn};
    pub(crate) use crate::board_rep::board::{Board, BoardRank};
    pub(crate) use crate::board_rep::piece::{Bishop, Knight, Queen, Rook};
    pub(crate) use crate::board_rep::square::{all_sqs::*};
    pub(crate) use crate::board_rep::square::offset::{SomeSquare, NoSquare};
    use crate::state::FullCa;
    pub(crate) use crate::state::State;
    pub(crate) use crate::util::board_creator::*;
    pub(crate) use crate::board_rep::color::{White, Black};

    type StartStateBoard = Board<
        //        AA  BB  CC  DD  EE  FF  GG  HH
        BoardRank<WR, WN, WB, WQ, WK, WB, WN, WR>, // 1
        BoardRank<WP, WP, WP, WP, WP, WP, WP, WP>, // 2
        BoardRank<__, __, __, __, __, __, __, __>, // 3
        BoardRank<__, __, __, __, __, __, __, __>, // 4
        BoardRank<__, __, __, __, __, __, __, __>, // 5
        BoardRank<__, __, __, __, __, __, __, __>, // 6
        BoardRank<BP, BP, BP, BP, BP, BP, BP, BP>, // 7
        BoardRank<BP, BN, BB, BQ, BK, BB, BN, BR>, // 8
    >;
    pub(crate) type StartState = State<White, StartStateBoard, NoSquare, FullCa>;
}

pub(crate) trait MaybePiece {
    fn reify() -> Option<values::Piece>;
}
pub(crate) struct NoPiece;

impl MaybePiece for NoPiece {
    fn reify() -> Option<values::Piece> {
        None
    }
}
impl<P: PieceEn> MaybePiece for P {
    fn reify() -> Option<values::Piece> {
        Some(P::reify())
    }
}

pub(crate) trait EasyMoveTy {}
pub(crate) struct EasyMove<From: SquareTy, To: SquareTy, Piece: MaybePiece = NoPiece>(
    PhantomData<(From, To, Piece)>,
);

impl<From: SquareTy, To: SquareTy, P: MaybePiece> EasyMoveTy for EasyMove<From, To, P> {}

pub(crate) trait RunFindMove<ML: MoveListTy>: EasyMoveTy {
    type Output: MaybeMove;
}
pub(crate) type FindMove<EM, ML> = <EM as RunFindMove<ML>>::Output;

impl<EM: EasyMoveTy> RunFindMove<MLNil> for EM {
    type Output = NoMove;
}
impl<
        F: SquareTy,
        T: SquareTy,
        CP: ColoredPieceTy,
        EP: MaybeSquare,
        RF: MaybeSquare,
        RT: MaybeSquare,
        Next: MoveListTy,
        EMF: SquareTy,
        EMT: SquareTy,
    > RunFindMove<MLCons<Move<F, T, CP, EP, RF, RT>, Next>> for EasyMove<EMF, EMT, NoPiece>
where
    F: RunSquareEq<EMF>,
    T: RunSquareEq<EMT>,
    SquareEq<F, EMF>: RunAnd<SquareEq<T, EMT>>,
    EasyMove<EMF, EMT>: RunFindMoveWMatches<
        Move<F, T, CP, EP, RF, RT>,
        Next,
        And<SquareEq<F, EMF>, SquareEq<T, EMT>>,
    >,
{
    type Output = <EasyMove<EMF, EMT> as RunFindMoveWMatches<
        Move<F, T, CP, EP, RF, RT>,
        Next,
        And<SquareEq<F, EMF>, SquareEq<T, EMT>>,
    >>::Output;
}
impl<
        F: SquareTy,
        T: SquareTy,
        MoverC: ColorEn,
        P: PieceEn,
        EP: MaybeSquare,
        RF: MaybeSquare,
        RT: MaybeSquare,
        Next: MoveListTy,
        EMF: SquareTy,
        EMT: SquareTy,
        EMP: PieceEn + MaybePiece,
    > RunFindMove<MLCons<Move<F, T, ColoredPiece<P, MoverC>, EP, RF, RT>, Next>>
    for EasyMove<EMF, EMT, EMP>
where
    F: RunSquareEq<EMF>,
    T: RunSquareEq<EMT>,
    SquareEq<F, EMF>: RunAnd<SquareEq<T, EMT>>,
    P: RunPieceEq<EMP>,
    PieceEq<P, EMP>: RunAnd<And<SquareEq<F, EMF>, SquareEq<T, EMT>>>,
    EasyMove<EMF, EMT, EMP>: RunFindMoveWMatches<
        Move<F, T, ColoredPiece<P, MoverC>, EP, RF, RT>,
        Next,
        And<PieceEq<P, EMP>, And<SquareEq<F, EMF>, SquareEq<T, EMT>>>,
    >,
{
    type Output = <EasyMove<EMF, EMT, EMP> as RunFindMoveWMatches<
        Move<F, T, ColoredPiece<P, MoverC>, EP, RF, RT>,
        Next,
        And<PieceEq<P, EMP>, And<SquareEq<F, EMF>, SquareEq<T, EMT>>>,
    >>::Output;
}

pub(crate) trait RunFindMoveWMatches<M: MoveTy, Next: MoveListTy, Matches: Bool>:
    EasyMoveTy
{
    type Output: MaybeMove;
}

impl<EM: EasyMoveTy, M: MoveTy, Next: MoveListTy> RunFindMoveWMatches<M, Next, True> for EM {
    type Output = SomeMove<M>;
}
impl<EM: EasyMoveTy, M: MoveTy, Next: MoveListTy> RunFindMoveWMatches<M, Next, False> for EM
where
    EM: RunFindMove<Next>,
{
    type Output = FindMove<EM, Next>;
}

pub(crate) trait MakeEMResultEn {
    fn reify() -> values::EMResult;
}
pub(crate) struct InvalidMove;

impl<S: StateTy> MakeEMResultEn for S {
    fn reify() -> values::EMResult {
        values::EMResult::Ongoing(S::reify())
    }
}
impl MakeEMResultEn for InvalidMove {
    fn reify() -> values::EMResult {
        values::EMResult::InvalidMove
    }
}
impl MakeEMResultEn for Draw {
    fn reify() -> values::EMResult {
        values::EMResult::Draw
    }
}
impl<C: ColorEn> MakeEMResultEn for Checkmate<C> {
    fn reify() -> values::EMResult {
        values::EMResult::Checkmate(C::reify())
    }
}

pub(crate) trait RunMakeEM<F: SquareTy, T: SquareTy, P: MaybePiece>: StateTy {
    type Output: MakeEMResultEn;
}
pub(crate) type MakeEM<S, F, T, P = NoPiece> = <S as RunMakeEM<F, T, P>>::Output;

impl<S: StateTy, F: SquareTy, T: SquareTy, P: MaybePiece> RunMakeEM<F, T, P> for S
where
    S: RunMoves,
    EasyMove<F, T, P>: RunFindMove<Moves<S>>,
    S: RunMakeEMwM<FindMove<EasyMove<F, T, P>, Moves<S>>>,
{
    type Output = <S as RunMakeEMwM<FindMove<EasyMove<F, T, P>, Moves<S>>>>::Output;
}

pub(crate) trait RunMakeEMwM<M: MaybeMove>: StateTy {
    type Output: MakeEMResultEn;
}

impl<S: StateTy> RunMakeEMwM<NoMove> for S {
    type Output = InvalidMove;
}
impl<S: StateTy, M: MoveTy> RunMakeEMwM<SomeMove<M>> for S
where
    M: RunMakeMove<S>,
    MakeMove<M, S>: RunOutcome,
    MakeMove<M, S>: RunMakeEMwO<Outcome<MakeMove<M, S>>>,
{
    type Output = <MakeMove<M, S> as RunMakeEMwO<Outcome<MakeMove<M, S>>>>::Output;
}

pub(crate) trait RunMakeEMwO<O: OutcomeEn>: StateTy {
    type Output: MakeEMResultEn;
}

impl<S: StateTy> RunMakeEMwO<Ongoing> for S {
    type Output = S;
}
impl<S: StateTy> RunMakeEMwO<Draw> for S {
    type Output = Draw;
}
impl<S: StateTy, C: ColorEn> RunMakeEMwO<Checkmate<C>> for S {
    type Output = Checkmate<C>;
}
