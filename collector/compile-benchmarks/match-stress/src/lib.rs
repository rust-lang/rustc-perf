//! This benchmark was originally extracted from the `syn` benchmark to provide
//! a realistic benchmark for match checking. It mostly defines a ton of
//! structs and enums, and the matches come from the `PartialEq` derives. This
//! resulted in the `match-stress-exhaustive_patterns` benchmark.
//!
//! Later the `match-stress-enum` benchmark was merged in (as the `huge`
//! module). The combination forms the `match-stress` benchmark. This was done
//! because having two different match stress tests cluttered up the results. A
//! single stress test should be enough to identify any regressions.

#![feature(exhaustive_patterns)]

#[macro_use]
mod macros;

mod delimited {
    #[derive(PartialEq)]
    pub struct Delimited {
        inner: Box<u64>,
    }
}

mod span {
    pub struct Span(u32, u32);

    impl PartialEq for Span {
        fn eq(&self, _other: &Span) -> bool {
            true
        }
    }
}

mod tokens {
    use super::span::Span;

    macro_rules! tokens {
        ( $($token:ident,)*) => (
            $(token! { $token })*
        )
    }

    macro_rules! token {
        ($name:ident) => {
            #[derive(PartialEq)]
            pub struct $name(Span, Span);
        };
    }

    tokens! {
        Add,
        AddEq,
        And,
        AndAnd,
        AndEq,
        As,
        At,
        Bang,
        Box_,
        Brace,
        Bracket,
        Break,
        CapSelf,
        Caret,
        CaretEq,
        Catch,
        Colon,
        Colon2,
        Comma,
        Const,
        Continue,
        Crate,
        Default_,
        Div,
        DivEq,
        Do,
        Dot,
        Dot2,
        Dot3,
        Else,
        Enum,
        Eq,
        EqEq,
        Extern,
        Fn_,
        For,
        Ge,
        Group,
        Gt,
        If,
        Impl,
        In,
        LArrow,
        Le,
        Let,
        Loop,
        Lt,
        Match,
        Mod,
        Move,
        MulEq,
        Mut,
        Ne,
        Or,
        OrEq,
        OrOr,
        Paren,
        Pound,
        Pub,
        Question,
        RArrow,
        Ref,
        Rem,
        RemEq,
        Return,
        Rocket,
        Self_,
        Semi,
        Shl,
        ShlEq,
        Shr,
        ShrEq,
        Star,
        Static,
        Struct,
        Sub,
        SubEq,
        Super,
        Trait,
        Type,
        Underscore,
        Union,
        Unsafe,
        Use,
        Where,
        While,
        Yield,
    }
}

mod ident {
    use Span;

    pub struct Ident {
        pub span0: Span,
        pub span1: Span,
        pub span2: Span,
        pub span3: Span,
        pub span4: Span,
        pub span5: Span,
        pub span6: Span,
        pub span7: Span,
    }

    impl PartialEq<Ident> for Ident {
        fn eq(&self, _other: &Ident) -> bool {
            todo!()
        }
    }
}

mod lifetime {
    use Ident;

    #[derive(PartialEq)]
    pub struct Lifetime {
        pub ident: Ident,
    }
}

mod lit {
    use Ident;

    #[derive(PartialEq)]
    pub struct Lit {
        pub ident: Ident,
    }
}

mod attr;
mod data;
mod expr;
mod generics;
mod huge;
mod item;
mod mac;
mod op;
mod ty;
pub mod visit;
pub use attr::*;
pub use data::*;
pub use expr::*;
pub use generics::*;
pub use ident::*;
pub use item::*;
pub use lifetime::*;
pub use lit::*;
pub use mac::*;
pub use op::*;
pub use span::Span;
pub use ty::*;
