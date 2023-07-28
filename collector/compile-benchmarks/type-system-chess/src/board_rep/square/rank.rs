use crate::{
    util::{Bool, False, True},
    values,
};

pub(crate) trait RankEn {
    fn reify() -> values::Rank;
}
pub(crate) struct R1;
pub(crate) struct R2;
pub(crate) struct R3;
pub(crate) struct R4;
pub(crate) struct R5;
pub(crate) struct R6;
pub(crate) struct R7;
pub(crate) struct R8;

impl RankEn for R1 {
    fn reify() -> values::Rank {
        values::Rank::R1
    }
}
impl RankEn for R2 {
    fn reify() -> values::Rank {
        values::Rank::R2
    }
}
impl RankEn for R3 {
    fn reify() -> values::Rank {
        values::Rank::R3
    }
}
impl RankEn for R4 {
    fn reify() -> values::Rank {
        values::Rank::R4
    }
}
impl RankEn for R5 {
    fn reify() -> values::Rank {
        values::Rank::R5
    }
}
impl RankEn for R6 {
    fn reify() -> values::Rank {
        values::Rank::R6
    }
}
impl RankEn for R7 {
    fn reify() -> values::Rank {
        values::Rank::R7
    }
}
impl RankEn for R8 {
    fn reify() -> values::Rank {
        values::Rank::R8
    }
}

pub(crate) trait RunRankEq<A: RankEn>: RankEn {
    type Output: Bool;
}
pub(crate) type RankEq<A, B> = <A as RunRankEq<B>>::Output;

// Its disturbing that I can't think of a better way to do this
// It seems like we would need some sort of negative impl or
// specificity.
impl RunRankEq<R1> for R1 {
    type Output = True;
}
impl RunRankEq<R1> for R2 {
    type Output = False;
}
impl RunRankEq<R1> for R3 {
    type Output = False;
}
impl RunRankEq<R1> for R4 {
    type Output = False;
}
impl RunRankEq<R1> for R5 {
    type Output = False;
}
impl RunRankEq<R1> for R6 {
    type Output = False;
}
impl RunRankEq<R1> for R7 {
    type Output = False;
}
impl RunRankEq<R1> for R8 {
    type Output = False;
}
impl RunRankEq<R2> for R1 {
    type Output = False;
}
impl RunRankEq<R2> for R2 {
    type Output = True;
}
impl RunRankEq<R2> for R3 {
    type Output = False;
}
impl RunRankEq<R2> for R4 {
    type Output = False;
}
impl RunRankEq<R2> for R5 {
    type Output = False;
}
impl RunRankEq<R2> for R6 {
    type Output = False;
}
impl RunRankEq<R2> for R7 {
    type Output = False;
}
impl RunRankEq<R2> for R8 {
    type Output = False;
}
impl RunRankEq<R3> for R1 {
    type Output = False;
}
impl RunRankEq<R3> for R2 {
    type Output = False;
}
impl RunRankEq<R3> for R3 {
    type Output = True;
}
impl RunRankEq<R3> for R4 {
    type Output = False;
}
impl RunRankEq<R3> for R5 {
    type Output = False;
}
impl RunRankEq<R3> for R6 {
    type Output = False;
}
impl RunRankEq<R3> for R7 {
    type Output = False;
}
impl RunRankEq<R3> for R8 {
    type Output = False;
}
impl RunRankEq<R4> for R1 {
    type Output = False;
}
impl RunRankEq<R4> for R2 {
    type Output = False;
}
impl RunRankEq<R4> for R3 {
    type Output = False;
}
impl RunRankEq<R4> for R4 {
    type Output = True;
}
impl RunRankEq<R4> for R5 {
    type Output = False;
}
impl RunRankEq<R4> for R6 {
    type Output = False;
}
impl RunRankEq<R4> for R7 {
    type Output = False;
}
impl RunRankEq<R4> for R8 {
    type Output = False;
}
impl RunRankEq<R5> for R1 {
    type Output = False;
}
impl RunRankEq<R5> for R2 {
    type Output = False;
}
impl RunRankEq<R5> for R3 {
    type Output = False;
}
impl RunRankEq<R5> for R4 {
    type Output = False;
}
impl RunRankEq<R5> for R5 {
    type Output = True;
}
impl RunRankEq<R5> for R6 {
    type Output = False;
}
impl RunRankEq<R5> for R7 {
    type Output = False;
}
impl RunRankEq<R5> for R8 {
    type Output = False;
}
impl RunRankEq<R6> for R1 {
    type Output = False;
}
impl RunRankEq<R6> for R2 {
    type Output = False;
}
impl RunRankEq<R6> for R3 {
    type Output = False;
}
impl RunRankEq<R6> for R4 {
    type Output = False;
}
impl RunRankEq<R6> for R5 {
    type Output = False;
}
impl RunRankEq<R6> for R6 {
    type Output = True;
}
impl RunRankEq<R6> for R7 {
    type Output = False;
}
impl RunRankEq<R6> for R8 {
    type Output = False;
}
impl RunRankEq<R7> for R1 {
    type Output = False;
}
impl RunRankEq<R7> for R2 {
    type Output = False;
}
impl RunRankEq<R7> for R3 {
    type Output = False;
}
impl RunRankEq<R7> for R4 {
    type Output = False;
}
impl RunRankEq<R7> for R5 {
    type Output = False;
}
impl RunRankEq<R7> for R6 {
    type Output = False;
}
impl RunRankEq<R7> for R7 {
    type Output = True;
}
impl RunRankEq<R7> for R8 {
    type Output = False;
}
impl RunRankEq<R8> for R1 {
    type Output = False;
}
impl RunRankEq<R8> for R2 {
    type Output = False;
}
impl RunRankEq<R8> for R3 {
    type Output = False;
}
impl RunRankEq<R8> for R4 {
    type Output = False;
}
impl RunRankEq<R8> for R5 {
    type Output = False;
}
impl RunRankEq<R8> for R6 {
    type Output = False;
}
impl RunRankEq<R8> for R7 {
    type Output = False;
}
impl RunRankEq<R8> for R8 {
    type Output = True;
}
