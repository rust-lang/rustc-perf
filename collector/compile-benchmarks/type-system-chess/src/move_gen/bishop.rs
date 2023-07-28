use super::{
    cast::{AddCastSqs, RunAddCastSqs},
    list::{SLNil, SquareListTy},
};
use crate::board_rep::{
    board::BoardTy,
    square::{
        offset::{Neg1, Offset, Pos1},
        SquareTy,
    },
};

pub(crate) trait RunBishopMoveSqs<B: BoardTy>: SquareTy {
    type Output: SquareListTy;
}
pub(crate) type BishopMoveSqs<S, B> = <S as RunBishopMoveSqs<B>>::Output;

type O1 = Offset<Pos1, Pos1>;
type O2 = Offset<Neg1, Pos1>;
type O3 = Offset<Pos1, Neg1>;
type O4 = Offset<Neg1, Neg1>;

type L1<B, S> = AddCastSqs<SLNil, B, S, O1>;
type L2<B, S> = AddCastSqs<L1<B, S>, B, S, O2>;
type L3<B, S> = AddCastSqs<L2<B, S>, B, S, O3>;
type L4<B, S> = AddCastSqs<L3<B, S>, B, S, O4>;

impl<S: SquareTy, B: BoardTy> RunBishopMoveSqs<B> for S
where
    SLNil: RunAddCastSqs<B, S, O1>,
    L1<B, S>: RunAddCastSqs<B, S, O2>,
    L2<B, S>: RunAddCastSqs<B, S, O3>,
    L3<B, S>: RunAddCastSqs<B, S, O4>,
{
    type Output = L4<B, S>;
}
