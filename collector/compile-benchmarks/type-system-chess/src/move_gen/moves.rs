use crate::{
    board_rep::{
        board::BoardTy,
        color::{Black, ColorEn, White},
        square::offset::MaybeSquare,
    },
    state::{CastleState, MakeMove, RunMakeMove, State, StateTy},
    util::{Bool, False, True},
};

use super::{
    check::{IsCheck, RunIsCheck},
    list::{AppendMaybeMove, MLCons, MLNil, MoveListTy, RunAppendMaybeMove},
    pmoves::{PMoves, RunPMoves},
    MaybeMove, MoveTy, NoMove, SomeMove,
};

pub(crate) trait RunMoves: StateTy {
    type Output: MoveListTy;
}
pub(crate) type Moves<S> = <S as RunMoves>::Output;

impl<B: BoardTy, MoverC: ColorEn, EP: MaybeSquare, WK: Bool, WQ: Bool, BK: Bool, BQ: Bool> RunMoves
    for State<MoverC, B, EP, CastleState<WK, WQ, BK, BQ>>
where
    B: RunPMoves<MoverC, EP, WK, WQ, BK, BQ>,
    PMoves<B, MoverC, EP, WK, WQ, BK, BQ>:
        RunMsFromPMs<MLNil, State<MoverC, B, EP, CastleState<WK, WQ, BK, BQ>>, MoverC>,
{
    type Output = MsFromPMs<
        PMoves<B, MoverC, EP, WK, WQ, BK, BQ>,
        MLNil,
        State<MoverC, B, EP, CastleState<WK, WQ, BK, BQ>>,
        MoverC,
    >;
}

pub(crate) trait RunMsFromPMs<ML: MoveListTy, S: StateTy, MoverC: ColorEn>:
    MoveListTy
{
    type Output: MoveListTy;
}
pub(crate) type MsFromPMs<PML, ML, S, MoverC> = <PML as RunMsFromPMs<ML, S, MoverC>>::Output;

impl<ML: MoveListTy, S: StateTy, MoverC: ColorEn> RunMsFromPMs<ML, S, MoverC> for MLNil {
    type Output = ML;
}
impl<ML: MoveListTy, S: StateTy, M: MoveTy, Next: MoveListTy> RunMsFromPMs<ML, S, White>
    for MLCons<M, Next>
where
    M: RunMakeMove<S>,
    MakeMove<M, S>: RunIsCheck<Black>,
    M: RunMMFromIsCheck<IsCheck<MakeMove<M, S>, Black>>,
    Next: RunMsFromPMs<ML, S, White>,
    MsFromPMs<Next, ML, S, White>:
        RunAppendMaybeMove<MMFromIsCheck<M, IsCheck<MakeMove<M, S>, Black>>>,
{
    type Output = AppendMaybeMove<
        MsFromPMs<Next, ML, S, White>,
        MMFromIsCheck<M, IsCheck<MakeMove<M, S>, Black>>,
    >;
}
impl<ML: MoveListTy, S: StateTy, M: MoveTy, Next: MoveListTy> RunMsFromPMs<ML, S, Black>
    for MLCons<M, Next>
where
    M: RunMakeMove<S>,
    MakeMove<M, S>: RunIsCheck<White>,
    M: RunMMFromIsCheck<IsCheck<MakeMove<M, S>, White>>,
    Next: RunMsFromPMs<ML, S, Black>,
    MsFromPMs<Next, ML, S, Black>:
        RunAppendMaybeMove<MMFromIsCheck<M, IsCheck<MakeMove<M, S>, White>>>,
{
    type Output = AppendMaybeMove<
        MsFromPMs<Next, ML, S, Black>,
        MMFromIsCheck<M, IsCheck<MakeMove<M, S>, White>>,
    >;
}

pub(crate) trait RunMMFromIsCheck<C: Bool>: MoveTy {
    type Output: MaybeMove;
}
pub(crate) type MMFromIsCheck<M, C> = <M as RunMMFromIsCheck<C>>::Output;

impl<M: MoveTy> RunMMFromIsCheck<True> for M {
    type Output = NoMove;
}
impl<M: MoveTy> RunMMFromIsCheck<False> for M {
    type Output = SomeMove<M>;
}
