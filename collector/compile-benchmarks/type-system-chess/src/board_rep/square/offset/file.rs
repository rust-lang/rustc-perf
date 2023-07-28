use super::{Neg1, Neg2, Offset1DEn, Pos1, Pos2, Zero};
use crate::{
    board_rep::square::file::{self, FileEn},
    values,
};
use std::marker::PhantomData;

pub(crate) trait MaybeFile {
    fn reify() -> Option<values::File>;
}

pub(crate) struct NoFile;
pub(crate) struct SomeFile<R: FileEn>(PhantomData<R>);

impl MaybeFile for NoFile {
    fn reify() -> Option<values::File> {
        None
    }
}
impl<R: FileEn> MaybeFile for SomeFile<R> {
    fn reify() -> Option<values::File> {
        Some(R::reify())
    }
}

pub(crate) trait RunOffsetFile<O: Offset1DEn>: FileEn {
    type Output: MaybeFile;
}
pub(crate) type OffsetFile<R, O> = <R as RunOffsetFile<O>>::Output;

impl RunOffsetFile<Neg2> for file::FA {
    type Output = NoFile;
}
impl RunOffsetFile<Neg2> for file::FB {
    type Output = NoFile;
}
impl RunOffsetFile<Neg2> for file::FC {
    type Output = SomeFile<file::FA>;
}
impl RunOffsetFile<Neg2> for file::FD {
    type Output = SomeFile<file::FB>;
}
impl RunOffsetFile<Neg2> for file::FE {
    type Output = SomeFile<file::FC>;
}
impl RunOffsetFile<Neg2> for file::FF {
    type Output = SomeFile<file::FD>;
}
impl RunOffsetFile<Neg2> for file::FG {
    type Output = SomeFile<file::FE>;
}
impl RunOffsetFile<Neg2> for file::FH {
    type Output = SomeFile<file::FF>;
}
impl RunOffsetFile<Neg1> for file::FA {
    type Output = NoFile;
}
impl RunOffsetFile<Neg1> for file::FB {
    type Output = SomeFile<file::FA>;
}
impl RunOffsetFile<Neg1> for file::FC {
    type Output = SomeFile<file::FB>;
}
impl RunOffsetFile<Neg1> for file::FD {
    type Output = SomeFile<file::FC>;
}
impl RunOffsetFile<Neg1> for file::FE {
    type Output = SomeFile<file::FD>;
}
impl RunOffsetFile<Neg1> for file::FF {
    type Output = SomeFile<file::FE>;
}
impl RunOffsetFile<Neg1> for file::FG {
    type Output = SomeFile<file::FF>;
}
impl RunOffsetFile<Neg1> for file::FH {
    type Output = SomeFile<file::FG>;
}
impl<F: FileEn> RunOffsetFile<Zero> for F {
    type Output = SomeFile<F>;
}
impl RunOffsetFile<Pos1> for file::FA {
    type Output = SomeFile<file::FB>;
}
impl RunOffsetFile<Pos1> for file::FB {
    type Output = SomeFile<file::FC>;
}
impl RunOffsetFile<Pos1> for file::FC {
    type Output = SomeFile<file::FD>;
}
impl RunOffsetFile<Pos1> for file::FD {
    type Output = SomeFile<file::FE>;
}
impl RunOffsetFile<Pos1> for file::FE {
    type Output = SomeFile<file::FF>;
}
impl RunOffsetFile<Pos1> for file::FF {
    type Output = SomeFile<file::FG>;
}
impl RunOffsetFile<Pos1> for file::FG {
    type Output = SomeFile<file::FH>;
}
impl RunOffsetFile<Pos1> for file::FH {
    type Output = NoFile;
}
impl RunOffsetFile<Pos2> for file::FA {
    type Output = SomeFile<file::FC>;
}
impl RunOffsetFile<Pos2> for file::FB {
    type Output = SomeFile<file::FD>;
}
impl RunOffsetFile<Pos2> for file::FC {
    type Output = SomeFile<file::FE>;
}
impl RunOffsetFile<Pos2> for file::FD {
    type Output = SomeFile<file::FF>;
}
impl RunOffsetFile<Pos2> for file::FE {
    type Output = SomeFile<file::FG>;
}
impl RunOffsetFile<Pos2> for file::FF {
    type Output = SomeFile<file::FH>;
}
impl RunOffsetFile<Pos2> for file::FG {
    type Output = NoFile;
}
impl RunOffsetFile<Pos2> for file::FH {
    type Output = NoFile;
}
