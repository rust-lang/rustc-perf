use crate::{
    util::{Bool, False, True},
    values,
};

pub(crate) trait FileEn {
    fn reify() -> values::File;
}
pub(crate) struct FA;
pub(crate) struct FB;
pub(crate) struct FC;
pub(crate) struct FD;
pub(crate) struct FE;
pub(crate) struct FF;
pub(crate) struct FG;
pub(crate) struct FH;

impl FileEn for FA {
    fn reify() -> values::File {
        values::File::A
    }
}
impl FileEn for FB {
    fn reify() -> values::File {
        values::File::B
    }
}
impl FileEn for FC {
    fn reify() -> values::File {
        values::File::C
    }
}
impl FileEn for FD {
    fn reify() -> values::File {
        values::File::D
    }
}
impl FileEn for FE {
    fn reify() -> values::File {
        values::File::E
    }
}
impl FileEn for FF {
    fn reify() -> values::File {
        values::File::F
    }
}
impl FileEn for FG {
    fn reify() -> values::File {
        values::File::G
    }
}
impl FileEn for FH {
    fn reify() -> values::File {
        values::File::H
    }
}

pub(crate) trait RunFileEq<A: FileEn>: FileEn {
    type Output: Bool;
}
pub(crate) type FileEq<A, B> = <A as RunFileEq<B>>::Output;

// Its disturbing that I can't think of a better way to do this
// It seems like we would need some sort of negative impl or
// specificity.
impl RunFileEq<FA> for FA {
    type Output = True;
}
impl RunFileEq<FA> for FB {
    type Output = False;
}
impl RunFileEq<FA> for FC {
    type Output = False;
}
impl RunFileEq<FA> for FD {
    type Output = False;
}
impl RunFileEq<FA> for FE {
    type Output = False;
}
impl RunFileEq<FA> for FF {
    type Output = False;
}
impl RunFileEq<FA> for FG {
    type Output = False;
}
impl RunFileEq<FA> for FH {
    type Output = False;
}
impl RunFileEq<FB> for FA {
    type Output = False;
}
impl RunFileEq<FB> for FB {
    type Output = True;
}
impl RunFileEq<FB> for FC {
    type Output = False;
}
impl RunFileEq<FB> for FD {
    type Output = False;
}
impl RunFileEq<FB> for FE {
    type Output = False;
}
impl RunFileEq<FB> for FF {
    type Output = False;
}
impl RunFileEq<FB> for FG {
    type Output = False;
}
impl RunFileEq<FB> for FH {
    type Output = False;
}
impl RunFileEq<FC> for FA {
    type Output = False;
}
impl RunFileEq<FC> for FB {
    type Output = False;
}
impl RunFileEq<FC> for FC {
    type Output = True;
}
impl RunFileEq<FC> for FD {
    type Output = False;
}
impl RunFileEq<FC> for FE {
    type Output = False;
}
impl RunFileEq<FC> for FF {
    type Output = False;
}
impl RunFileEq<FC> for FG {
    type Output = False;
}
impl RunFileEq<FC> for FH {
    type Output = False;
}
impl RunFileEq<FD> for FA {
    type Output = False;
}
impl RunFileEq<FD> for FB {
    type Output = False;
}
impl RunFileEq<FD> for FC {
    type Output = False;
}
impl RunFileEq<FD> for FD {
    type Output = True;
}
impl RunFileEq<FD> for FE {
    type Output = False;
}
impl RunFileEq<FD> for FF {
    type Output = False;
}
impl RunFileEq<FD> for FG {
    type Output = False;
}
impl RunFileEq<FD> for FH {
    type Output = False;
}
impl RunFileEq<FE> for FA {
    type Output = False;
}
impl RunFileEq<FE> for FB {
    type Output = False;
}
impl RunFileEq<FE> for FC {
    type Output = False;
}
impl RunFileEq<FE> for FD {
    type Output = False;
}
impl RunFileEq<FE> for FE {
    type Output = True;
}
impl RunFileEq<FE> for FF {
    type Output = False;
}
impl RunFileEq<FE> for FG {
    type Output = False;
}
impl RunFileEq<FE> for FH {
    type Output = False;
}
impl RunFileEq<FF> for FA {
    type Output = False;
}
impl RunFileEq<FF> for FB {
    type Output = False;
}
impl RunFileEq<FF> for FC {
    type Output = False;
}
impl RunFileEq<FF> for FD {
    type Output = False;
}
impl RunFileEq<FF> for FE {
    type Output = False;
}
impl RunFileEq<FF> for FF {
    type Output = True;
}
impl RunFileEq<FF> for FG {
    type Output = False;
}
impl RunFileEq<FF> for FH {
    type Output = False;
}
impl RunFileEq<FG> for FA {
    type Output = False;
}
impl RunFileEq<FG> for FB {
    type Output = False;
}
impl RunFileEq<FG> for FC {
    type Output = False;
}
impl RunFileEq<FG> for FD {
    type Output = False;
}
impl RunFileEq<FG> for FE {
    type Output = False;
}
impl RunFileEq<FG> for FF {
    type Output = False;
}
impl RunFileEq<FG> for FG {
    type Output = True;
}
impl RunFileEq<FG> for FH {
    type Output = False;
}
impl RunFileEq<FH> for FA {
    type Output = False;
}
impl RunFileEq<FH> for FB {
    type Output = False;
}
impl RunFileEq<FH> for FC {
    type Output = False;
}
impl RunFileEq<FH> for FD {
    type Output = False;
}
impl RunFileEq<FH> for FE {
    type Output = False;
}
impl RunFileEq<FH> for FF {
    type Output = False;
}
impl RunFileEq<FH> for FG {
    type Output = False;
}
impl RunFileEq<FH> for FH {
    type Output = True;
}
