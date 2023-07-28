use super::{MaybeMove, MoveTy, NoMove, SomeMove};
use crate::{
    board_rep::square::{
        offset::{MaybeSquare, NoSquare, SomeSquare},
        SquareTy,
    },
    values,
};
use std::marker::PhantomData;

pub(crate) trait MoveListTy {
    fn reify() -> values::MoveList;
}
pub(crate) struct MLNil;
pub(crate) struct MLCons<M: MoveTy, L: MoveListTy>(PhantomData<(M, L)>);

impl MoveListTy for MLNil {
    fn reify() -> values::MoveList {
        values::MoveList(Vec::new())
    }
}
impl<M: MoveTy, L: MoveListTy> MoveListTy for MLCons<M, L> {
    fn reify() -> values::MoveList {
        let values::MoveList(mut tail) = L::reify();
        tail.insert(0, M::reify());
        values::MoveList(tail)
    }
}

pub(crate) trait RunAppendMaybeMove<M: MaybeMove>: MoveListTy {
    type Output: MoveListTy;
}
pub(crate) type AppendMaybeMove<L, M> = <L as RunAppendMaybeMove<M>>::Output;

impl<L: MoveListTy> RunAppendMaybeMove<NoMove> for L {
    type Output = L;
}
impl<M: MoveTy, L: MoveListTy> RunAppendMaybeMove<SomeMove<M>> for L {
    type Output = MLCons<M, L>;
}

pub(crate) trait SquareListTy {
    fn reify() -> values::SquareList;
}
pub(crate) struct SLNil;
pub(crate) struct SLCons<S: SquareTy, L: SquareListTy>(PhantomData<(S, L)>);

impl SquareListTy for SLNil {
    fn reify() -> values::SquareList {
        values::SquareList(Vec::new())
    }
}
impl<S: SquareTy, L: SquareListTy> SquareListTy for SLCons<S, L> {
    fn reify() -> values::SquareList {
        let values::SquareList(mut tail) = L::reify();
        tail.insert(0, S::reify());
        values::SquareList(tail)
    }
}

pub(crate) trait RunAppendMaybeSquare<S: MaybeSquare>: SquareListTy {
    type Output: SquareListTy;
}
pub(crate) type AppendMaybeSquare<L, S> = <L as RunAppendMaybeSquare<S>>::Output;

impl<L: SquareListTy> RunAppendMaybeSquare<NoSquare> for L {
    type Output = L;
}
impl<S: SquareTy, L: SquareListTy> RunAppendMaybeSquare<SomeSquare<S>> for L {
    type Output = SLCons<S, L>;
}
