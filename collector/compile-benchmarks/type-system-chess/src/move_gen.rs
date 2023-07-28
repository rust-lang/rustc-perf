use crate::{
    board_rep::{
        piece::ColoredPieceTy,
        square::{
            offset::{MaybeSquare, NoSquare},
            SquareTy,
        },
    },
    values,
};
use std::marker::PhantomData;

pub mod attacked;
pub mod bishop;
pub mod cast;
pub mod castle;
pub mod check;
pub mod king;
pub mod knight;
pub mod list;
pub mod ml_from_sl;
pub mod moves;
pub mod pawn;
pub mod pmoves;
pub mod queen;
pub mod rook;

pub(crate) trait MoveTy {
    fn reify() -> values::Move;
}
pub(crate) struct Move<
    From: SquareTy,
    To: SquareTy,
    P: ColoredPieceTy,
    EP: MaybeSquare = NoSquare,
    RFrom: MaybeSquare = NoSquare,
    RTo: MaybeSquare = NoSquare,
>(PhantomData<(From, To, P, EP, RFrom, RTo)>);

pub(crate) trait MaybeMove {
    fn reify() -> Option<values::Move>;
}
pub(crate) struct NoMove;
pub(crate) struct SomeMove<M: MoveTy>(PhantomData<M>);

impl MaybeMove for NoMove {
    fn reify() -> Option<values::Move> {
        None
    }
}
impl<M: MoveTy> MaybeMove for SomeMove<M> {
    fn reify() -> Option<values::Move> {
        Some(M::reify())
    }
}

impl<
        From: SquareTy,
        To: SquareTy,
        P: ColoredPieceTy,
        EP: MaybeSquare,
        RFrom: MaybeSquare,
        RTo: MaybeSquare,
    > MoveTy for Move<From, To, P, EP, RFrom, RTo>
{
    fn reify() -> values::Move {
        values::Move {
            from: From::reify(),
            to: To::reify(),
            piece: P::reify(),
            ep: EP::reify(),
            rook_from: RFrom::reify(),
            rook_to: RTo::reify(),
        }
    }
}
