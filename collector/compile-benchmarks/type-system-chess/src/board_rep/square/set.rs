use super::{
    file::{self, FileEn},
    rank::{self, RankEn},
    Square, SquareTy,
};
use crate::{
    move_gen::list::{SLCons, SLNil, SquareListTy},
    util::{Bool, False, True},
    values,
};
use std::marker::PhantomData;

pub(crate) trait SquareSetTy {
    fn reify() -> values::SquareSet;
}
pub(crate) struct SquareSet<
    R1: SquareSetRankTy,
    R2: SquareSetRankTy,
    R3: SquareSetRankTy,
    R4: SquareSetRankTy,
    R5: SquareSetRankTy,
    R6: SquareSetRankTy,
    R7: SquareSetRankTy,
    R8: SquareSetRankTy,
>(PhantomData<(R1, R2, R3, R4, R5, R6, R7, R8)>);

impl<
        R1: SquareSetRankTy,
        R2: SquareSetRankTy,
        R3: SquareSetRankTy,
        R4: SquareSetRankTy,
        R5: SquareSetRankTy,
        R6: SquareSetRankTy,
        R7: SquareSetRankTy,
        R8: SquareSetRankTy,
    > SquareSetTy for SquareSet<R1, R2, R3, R4, R5, R6, R7, R8>
{
    fn reify() -> values::SquareSet {
        values::SquareSet {
            r1: R1::reify(),
            r2: R2::reify(),
            r3: R3::reify(),
            r4: R4::reify(),
            r5: R5::reify(),
            r6: R6::reify(),
            r7: R7::reify(),
            r8: R8::reify(),
        }
    }
}

pub(crate) trait SquareSetRankTy {
    fn reify() -> values::SquareSetRank;
}
pub(crate) struct SquareSetRank<
    A: Bool,
    B: Bool,
    C: Bool,
    D: Bool,
    E: Bool,
    F: Bool,
    G: Bool,
    H: Bool,
>(PhantomData<(A, B, C, D, E, F, G, H)>);

impl<A: Bool, B: Bool, C: Bool, D: Bool, E: Bool, F: Bool, G: Bool, H: Bool> SquareSetRankTy
    for SquareSetRank<A, B, C, D, E, F, G, H>
{
    fn reify() -> values::SquareSetRank {
        values::SquareSetRank {
            a: A::reify(),
            b: B::reify(),
            c: C::reify(),
            d: D::reify(),
            e: E::reify(),
            f: F::reify(),
            g: G::reify(),
            h: H::reify(),
        }
    }
}

pub(crate) trait RunIdxSSRank<F: FileEn>: SquareSetRankTy {
    type Output: Bool;
}
pub(crate) type IdxSSRank<SS, F> = <SS as RunIdxSSRank<F>>::Output;
impl<A: Bool, B: Bool, C: Bool, D: Bool, E: Bool, F: Bool, G: Bool, H: Bool> RunIdxSSRank<file::FA>
    for SquareSetRank<A, B, C, D, E, F, G, H>
{
    type Output = A;
}
impl<A: Bool, B: Bool, C: Bool, D: Bool, E: Bool, F: Bool, G: Bool, H: Bool> RunIdxSSRank<file::FB>
    for SquareSetRank<A, B, C, D, E, F, G, H>
{
    type Output = B;
}
impl<A: Bool, B: Bool, C: Bool, D: Bool, E: Bool, F: Bool, G: Bool, H: Bool> RunIdxSSRank<file::FC>
    for SquareSetRank<A, B, C, D, E, F, G, H>
{
    type Output = C;
}
impl<A: Bool, B: Bool, C: Bool, D: Bool, E: Bool, F: Bool, G: Bool, H: Bool> RunIdxSSRank<file::FD>
    for SquareSetRank<A, B, C, D, E, F, G, H>
{
    type Output = D;
}
impl<A: Bool, B: Bool, C: Bool, D: Bool, E: Bool, F: Bool, G: Bool, H: Bool> RunIdxSSRank<file::FE>
    for SquareSetRank<A, B, C, D, E, F, G, H>
{
    type Output = E;
}
impl<A: Bool, B: Bool, C: Bool, D: Bool, E: Bool, F: Bool, G: Bool, H: Bool> RunIdxSSRank<file::FF>
    for SquareSetRank<A, B, C, D, E, F, G, H>
{
    type Output = F;
}
impl<A: Bool, B: Bool, C: Bool, D: Bool, E: Bool, F: Bool, G: Bool, H: Bool> RunIdxSSRank<file::FG>
    for SquareSetRank<A, B, C, D, E, F, G, H>
{
    type Output = G;
}
impl<A: Bool, B: Bool, C: Bool, D: Bool, E: Bool, F: Bool, G: Bool, H: Bool> RunIdxSSRank<file::FH>
    for SquareSetRank<A, B, C, D, E, F, G, H>
{
    type Output = H;
}

pub(crate) trait RunIdxSSForRank<R: RankEn>: SquareSetTy {
    type Output: SquareSetRankTy;
}
pub(crate) type IdxSSforRank<SS, R> = <SS as RunIdxSSForRank<R>>::Output;
impl<
        R1: SquareSetRankTy,
        R2: SquareSetRankTy,
        R3: SquareSetRankTy,
        R4: SquareSetRankTy,
        R5: SquareSetRankTy,
        R6: SquareSetRankTy,
        R7: SquareSetRankTy,
        R8: SquareSetRankTy,
    > RunIdxSSForRank<rank::R1> for SquareSet<R1, R2, R3, R4, R5, R6, R7, R8>
{
    type Output = R1;
}
impl<
        R1: SquareSetRankTy,
        R2: SquareSetRankTy,
        R3: SquareSetRankTy,
        R4: SquareSetRankTy,
        R5: SquareSetRankTy,
        R6: SquareSetRankTy,
        R7: SquareSetRankTy,
        R8: SquareSetRankTy,
    > RunIdxSSForRank<rank::R2> for SquareSet<R1, R2, R3, R4, R5, R6, R7, R8>
{
    type Output = R2;
}
impl<
        R1: SquareSetRankTy,
        R2: SquareSetRankTy,
        R3: SquareSetRankTy,
        R4: SquareSetRankTy,
        R5: SquareSetRankTy,
        R6: SquareSetRankTy,
        R7: SquareSetRankTy,
        R8: SquareSetRankTy,
    > RunIdxSSForRank<rank::R3> for SquareSet<R1, R2, R3, R4, R5, R6, R7, R8>
{
    type Output = R3;
}
impl<
        R1: SquareSetRankTy,
        R2: SquareSetRankTy,
        R3: SquareSetRankTy,
        R4: SquareSetRankTy,
        R5: SquareSetRankTy,
        R6: SquareSetRankTy,
        R7: SquareSetRankTy,
        R8: SquareSetRankTy,
    > RunIdxSSForRank<rank::R4> for SquareSet<R1, R2, R3, R4, R5, R6, R7, R8>
{
    type Output = R4;
}
impl<
        R1: SquareSetRankTy,
        R2: SquareSetRankTy,
        R3: SquareSetRankTy,
        R4: SquareSetRankTy,
        R5: SquareSetRankTy,
        R6: SquareSetRankTy,
        R7: SquareSetRankTy,
        R8: SquareSetRankTy,
    > RunIdxSSForRank<rank::R5> for SquareSet<R1, R2, R3, R4, R5, R6, R7, R8>
{
    type Output = R5;
}
impl<
        R1: SquareSetRankTy,
        R2: SquareSetRankTy,
        R3: SquareSetRankTy,
        R4: SquareSetRankTy,
        R5: SquareSetRankTy,
        R6: SquareSetRankTy,
        R7: SquareSetRankTy,
        R8: SquareSetRankTy,
    > RunIdxSSForRank<rank::R6> for SquareSet<R1, R2, R3, R4, R5, R6, R7, R8>
{
    type Output = R6;
}
impl<
        R1: SquareSetRankTy,
        R2: SquareSetRankTy,
        R3: SquareSetRankTy,
        R4: SquareSetRankTy,
        R5: SquareSetRankTy,
        R6: SquareSetRankTy,
        R7: SquareSetRankTy,
        R8: SquareSetRankTy,
    > RunIdxSSForRank<rank::R7> for SquareSet<R1, R2, R3, R4, R5, R6, R7, R8>
{
    type Output = R7;
}
impl<
        R1: SquareSetRankTy,
        R2: SquareSetRankTy,
        R3: SquareSetRankTy,
        R4: SquareSetRankTy,
        R5: SquareSetRankTy,
        R6: SquareSetRankTy,
        R7: SquareSetRankTy,
        R8: SquareSetRankTy,
    > RunIdxSSForRank<rank::R8> for SquareSet<R1, R2, R3, R4, R5, R6, R7, R8>
{
    type Output = R8;
}

pub(crate) trait RunIsOccupied<S: SquareTy>: SquareSetTy {
    type Output: Bool;
}
pub(crate) type IsOccupied<SS, S> = <SS as RunIsOccupied<S>>::Output;

impl<R: RankEn, F: FileEn, SS: SquareSetTy> RunIsOccupied<Square<R, F>> for SS
where
    SS: RunIdxSSForRank<R>,
    IdxSSforRank<SS, R>: RunIdxSSRank<F>,
{
    type Output = IdxSSRank<IdxSSforRank<SS, R>, F>;
}

pub(crate) type EmptySSRank = SquareSetRank<False, False, False, False, False, False, False, False>;
pub(crate) type EmptySS = SquareSet<
    EmptySSRank,
    EmptySSRank,
    EmptySSRank,
    EmptySSRank,
    EmptySSRank,
    EmptySSRank,
    EmptySSRank,
    EmptySSRank,
>;

pub(crate) trait RunAddToSSRank<F: FileEn>: SquareSetRankTy {
    type Output: SquareSetRankTy;
}
pub(crate) type AddToSSRank<SSR, F> = <SSR as RunAddToSSRank<F>>::Output;
impl<A: Bool, B: Bool, C: Bool, D: Bool, E: Bool, F: Bool, G: Bool, H: Bool>
    RunAddToSSRank<file::FA> for SquareSetRank<A, B, C, D, E, F, G, H>
{
    type Output = SquareSetRank<True, B, C, D, E, F, G, H>;
}
impl<A: Bool, B: Bool, C: Bool, D: Bool, E: Bool, F: Bool, G: Bool, H: Bool>
    RunAddToSSRank<file::FB> for SquareSetRank<A, B, C, D, E, F, G, H>
{
    type Output = SquareSetRank<A, True, C, D, E, F, G, H>;
}
impl<A: Bool, B: Bool, C: Bool, D: Bool, E: Bool, F: Bool, G: Bool, H: Bool>
    RunAddToSSRank<file::FC> for SquareSetRank<A, B, C, D, E, F, G, H>
{
    type Output = SquareSetRank<A, B, True, D, E, F, G, H>;
}
impl<A: Bool, B: Bool, C: Bool, D: Bool, E: Bool, F: Bool, G: Bool, H: Bool>
    RunAddToSSRank<file::FD> for SquareSetRank<A, B, C, D, E, F, G, H>
{
    type Output = SquareSetRank<A, B, C, True, E, F, G, H>;
}
impl<A: Bool, B: Bool, C: Bool, D: Bool, E: Bool, F: Bool, G: Bool, H: Bool>
    RunAddToSSRank<file::FE> for SquareSetRank<A, B, C, D, E, F, G, H>
{
    type Output = SquareSetRank<A, B, C, D, True, F, G, H>;
}
impl<A: Bool, B: Bool, C: Bool, D: Bool, E: Bool, F: Bool, G: Bool, H: Bool>
    RunAddToSSRank<file::FF> for SquareSetRank<A, B, C, D, E, F, G, H>
{
    type Output = SquareSetRank<A, B, C, D, E, True, G, H>;
}
impl<A: Bool, B: Bool, C: Bool, D: Bool, E: Bool, F: Bool, G: Bool, H: Bool>
    RunAddToSSRank<file::FG> for SquareSetRank<A, B, C, D, E, F, G, H>
{
    type Output = SquareSetRank<A, B, C, D, E, F, True, H>;
}
impl<A: Bool, B: Bool, C: Bool, D: Bool, E: Bool, F: Bool, G: Bool, H: Bool>
    RunAddToSSRank<file::FH> for SquareSetRank<A, B, C, D, E, F, G, H>
{
    type Output = SquareSetRank<A, B, C, D, E, F, G, True>;
}

pub(crate) trait RunAddToSS<S: SquareTy>: SquareSetTy {
    type Output: SquareSetTy;
}
pub(crate) type AddToSS<SS, S> = <SS as RunAddToSS<S>>::Output;
impl<
        R1: SquareSetRankTy,
        R2: SquareSetRankTy,
        R3: SquareSetRankTy,
        R4: SquareSetRankTy,
        R5: SquareSetRankTy,
        R6: SquareSetRankTy,
        R7: SquareSetRankTy,
        R8: SquareSetRankTy,
        F: FileEn,
    > RunAddToSS<Square<rank::R1, F>> for SquareSet<R1, R2, R3, R4, R5, R6, R7, R8>
where
    R1: RunAddToSSRank<F>,
{
    type Output = SquareSet<AddToSSRank<R1, F>, R2, R3, R4, R5, R6, R7, R8>;
}
impl<
        R1: SquareSetRankTy,
        R2: SquareSetRankTy,
        R3: SquareSetRankTy,
        R4: SquareSetRankTy,
        R5: SquareSetRankTy,
        R6: SquareSetRankTy,
        R7: SquareSetRankTy,
        R8: SquareSetRankTy,
        F: FileEn,
    > RunAddToSS<Square<rank::R2, F>> for SquareSet<R1, R2, R3, R4, R5, R6, R7, R8>
where
    R2: RunAddToSSRank<F>,
{
    type Output = SquareSet<R1, AddToSSRank<R2, F>, R3, R4, R5, R6, R7, R8>;
}
impl<
        R1: SquareSetRankTy,
        R2: SquareSetRankTy,
        R3: SquareSetRankTy,
        R4: SquareSetRankTy,
        R5: SquareSetRankTy,
        R6: SquareSetRankTy,
        R7: SquareSetRankTy,
        R8: SquareSetRankTy,
        F: FileEn,
    > RunAddToSS<Square<rank::R3, F>> for SquareSet<R1, R2, R3, R4, R5, R6, R7, R8>
where
    R3: RunAddToSSRank<F>,
{
    type Output = SquareSet<R1, R2, AddToSSRank<R3, F>, R4, R5, R6, R7, R8>;
}
impl<
        R1: SquareSetRankTy,
        R2: SquareSetRankTy,
        R3: SquareSetRankTy,
        R4: SquareSetRankTy,
        R5: SquareSetRankTy,
        R6: SquareSetRankTy,
        R7: SquareSetRankTy,
        R8: SquareSetRankTy,
        F: FileEn,
    > RunAddToSS<Square<rank::R4, F>> for SquareSet<R1, R2, R3, R4, R5, R6, R7, R8>
where
    R4: RunAddToSSRank<F>,
{
    type Output = SquareSet<R1, R2, R3, AddToSSRank<R4, F>, R5, R6, R7, R8>;
}
impl<
        R1: SquareSetRankTy,
        R2: SquareSetRankTy,
        R3: SquareSetRankTy,
        R4: SquareSetRankTy,
        R5: SquareSetRankTy,
        R6: SquareSetRankTy,
        R7: SquareSetRankTy,
        R8: SquareSetRankTy,
        F: FileEn,
    > RunAddToSS<Square<rank::R5, F>> for SquareSet<R1, R2, R3, R4, R5, R6, R7, R8>
where
    R5: RunAddToSSRank<F>,
{
    type Output = SquareSet<R1, R2, R3, R4, AddToSSRank<R5, F>, R6, R7, R8>;
}
impl<
        R1: SquareSetRankTy,
        R2: SquareSetRankTy,
        R3: SquareSetRankTy,
        R4: SquareSetRankTy,
        R5: SquareSetRankTy,
        R6: SquareSetRankTy,
        R7: SquareSetRankTy,
        R8: SquareSetRankTy,
        F: FileEn,
    > RunAddToSS<Square<rank::R6, F>> for SquareSet<R1, R2, R3, R4, R5, R6, R7, R8>
where
    R6: RunAddToSSRank<F>,
{
    type Output = SquareSet<R1, R2, R3, R4, R5, AddToSSRank<R6, F>, R7, R8>;
}
impl<
        R1: SquareSetRankTy,
        R2: SquareSetRankTy,
        R3: SquareSetRankTy,
        R4: SquareSetRankTy,
        R5: SquareSetRankTy,
        R6: SquareSetRankTy,
        R7: SquareSetRankTy,
        R8: SquareSetRankTy,
        F: FileEn,
    > RunAddToSS<Square<rank::R7, F>> for SquareSet<R1, R2, R3, R4, R5, R6, R7, R8>
where
    R7: RunAddToSSRank<F>,
{
    type Output = SquareSet<R1, R2, R3, R4, R5, R6, AddToSSRank<R7, F>, R8>;
}
impl<
        R1: SquareSetRankTy,
        R2: SquareSetRankTy,
        R3: SquareSetRankTy,
        R4: SquareSetRankTy,
        R5: SquareSetRankTy,
        R6: SquareSetRankTy,
        R7: SquareSetRankTy,
        R8: SquareSetRankTy,
        F: FileEn,
    > RunAddToSS<Square<rank::R8, F>> for SquareSet<R1, R2, R3, R4, R5, R6, R7, R8>
where
    R8: RunAddToSSRank<F>,
{
    type Output = SquareSet<R1, R2, R3, R4, R5, R6, R7, AddToSSRank<R8, F>>;
}

pub(crate) trait RunAddToSSFromL<L: SquareListTy>: SquareSetTy {
    type Output: SquareSetTy;
}
pub(crate) type AddToSSFromL<SS, L> = <SS as RunAddToSSFromL<L>>::Output;

impl<SS: SquareSetTy> RunAddToSSFromL<SLNil> for SS {
    type Output = SS;
}
impl<S: SquareTy, Next: SquareListTy, SS: SquareSetTy> RunAddToSSFromL<SLCons<S, Next>> for SS
where
    SS: RunAddToSS<S>,
    AddToSS<SS, S>: RunAddToSSFromL<Next>,
{
    type Output = AddToSSFromL<AddToSS<SS, S>, Next>;
}
