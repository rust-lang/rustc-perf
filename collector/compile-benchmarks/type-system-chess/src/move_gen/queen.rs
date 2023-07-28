use super::{
    cast::{AddCastSqs, RunAddCastSqs},
    list::{SLNil, SquareListTy},
};
use crate::board_rep::{
    board::BoardTy,
    square::{
        offset::{Neg1, Offset, Pos1, Zero},
        SquareTy,
    },
};

pub(crate) trait RunQueenMoveSqs<B: BoardTy>: SquareTy {
    type Output: SquareListTy;
}
pub(crate) type QueenMoveSqs<S, B> = <S as RunQueenMoveSqs<B>>::Output;

type O1 = Offset<Neg1, Neg1>;
type O2 = Offset<Neg1, Zero>;
type O3 = Offset<Neg1, Pos1>;
type O4 = Offset<Zero, Neg1>;
type O5 = Offset<Zero, Pos1>;
type O6 = Offset<Pos1, Neg1>;
type O7 = Offset<Pos1, Zero>;
type O8 = Offset<Pos1, Pos1>;

type L1<B, S> = AddCastSqs<SLNil, B, S, O1>;
type L2<B, S> = AddCastSqs<L1<B, S>, B, S, O2>;
type L3<B, S> = AddCastSqs<L2<B, S>, B, S, O3>;
type L4<B, S> = AddCastSqs<L3<B, S>, B, S, O4>;
type L5<B, S> = AddCastSqs<L4<B, S>, B, S, O5>;
type L6<B, S> = AddCastSqs<L5<B, S>, B, S, O6>;
type L7<B, S> = AddCastSqs<L6<B, S>, B, S, O7>;
type L8<B, S> = AddCastSqs<L7<B, S>, B, S, O8>;

impl<S: SquareTy, B: BoardTy> RunQueenMoveSqs<B> for S
where
    SLNil: RunAddCastSqs<B, S, O1>,
    L1<B, S>: RunAddCastSqs<B, S, O2>,
    L2<B, S>: RunAddCastSqs<B, S, O3>,
    L3<B, S>: RunAddCastSqs<B, S, O4>,
    L4<B, S>: RunAddCastSqs<B, S, O5>,
    L5<B, S>: RunAddCastSqs<B, S, O6>,
    L6<B, S>: RunAddCastSqs<B, S, O7>,
    L7<B, S>: RunAddCastSqs<B, S, O8>,
{
    type Output = L8<B, S>;
}
