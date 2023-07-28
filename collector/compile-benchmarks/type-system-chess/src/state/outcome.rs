use std::marker::PhantomData;

use crate::{
    board_rep::{
        board::BoardTy,
        color::{Black, ColorEn, White},
        square::offset::MaybeSquare,
    },
    move_gen::{
        check::{IsCheck, RunIsCheck},
        list::{MLCons, MLNil, MoveListTy},
        moves::{Moves, RunMoves},
        MoveTy,
    },
    util::{Bool, False, True},
    values,
};

use super::{CastleStateTy, State, StateTy};

pub(crate) trait OutcomeEn {
    fn reify() -> values::Outcome;
}
pub(crate) struct Ongoing;
pub(crate) struct Draw;
pub(crate) struct Checkmate<Winner: ColorEn>(PhantomData<Winner>);

impl OutcomeEn for Ongoing {
    fn reify() -> values::Outcome {
        values::Outcome::Ongoing
    }
}
impl OutcomeEn for Draw {
    fn reify() -> values::Outcome {
        values::Outcome::Draw
    }
}
impl<W: ColorEn> OutcomeEn for Checkmate<W> {
    fn reify() -> values::Outcome {
        values::Outcome::Checkmate(W::reify())
    }
}

pub(crate) trait RunOutcome: StateTy {
    type Output: OutcomeEn;
}
pub(crate) type Outcome<S> = <S as RunOutcome>::Output;

impl<B: BoardTy, EP: MaybeSquare, CA: CastleStateTy> RunOutcome for State<White, B, EP, CA>
where
    State<White, B, EP, CA>: RunMoves,
    State<White, B, EP, CA>: RunOutcomeWML<Moves<State<White, B, EP, CA>>, Black>,
{
    type Output =
        <State<White, B, EP, CA> as RunOutcomeWML<Moves<State<White, B, EP, CA>>, Black>>::Output;
}
impl<B: BoardTy, EP: MaybeSquare, CA: CastleStateTy> RunOutcome for State<Black, B, EP, CA>
where
    State<Black, B, EP, CA>: RunMoves,
    State<Black, B, EP, CA>: RunOutcomeWML<Moves<State<Black, B, EP, CA>>, White>,
{
    type Output =
        <State<Black, B, EP, CA> as RunOutcomeWML<Moves<State<Black, B, EP, CA>>, White>>::Output;
}

pub(crate) trait RunOutcomeWML<ML: MoveListTy, MoverC: ColorEn>: StateTy {
    type Output: OutcomeEn;
}

impl<M: MoveTy, L: MoveListTy, S: StateTy, MoverC: ColorEn> RunOutcomeWML<MLCons<M, L>, MoverC>
    for S
{
    type Output = Ongoing;
}
impl<S: StateTy, MoverC: ColorEn> RunOutcomeWML<MLNil, MoverC> for S
where
    S: RunIsCheck<MoverC>,
    S: RunOutcomeWIsCheck<IsCheck<S, MoverC>>,
{
    type Output = <S as RunOutcomeWIsCheck<IsCheck<S, MoverC>>>::Output;
}

pub(crate) trait RunOutcomeWIsCheck<C: Bool>: StateTy {
    type Output: OutcomeEn;
}

impl<S: StateTy> RunOutcomeWIsCheck<False> for S {
    type Output = Draw;
}
impl<B: BoardTy, EP: MaybeSquare, CA: CastleStateTy> RunOutcomeWIsCheck<True>
    for State<White, B, EP, CA>
{
    type Output = Checkmate<Black>;
}
impl<B: BoardTy, EP: MaybeSquare, CA: CastleStateTy> RunOutcomeWIsCheck<True>
    for State<Black, B, EP, CA>
{
    type Output = Checkmate<White>;
}
