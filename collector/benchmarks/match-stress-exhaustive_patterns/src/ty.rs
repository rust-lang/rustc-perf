use super::*;
use delimited::Delimited;

ast_enum_of_structs! {
    /// The different kinds of types recognized by the compiler
    pub enum Ty {
        /// A variable-length array (`[T]`)
        pub Slice(TySlice {
            pub ty: Box<u64>,
            pub bracket_token: tokens::Bracket,
        }),
        /// A fixed length array (`[T; n]`)
        pub Array(TyArray {
            pub bracket_token: tokens::Bracket,
            pub ty: Box<u64>,
            pub semi_token: tokens::Semi,
            pub amt0: Expr,
            pub amt1: Expr,
        }),
        /// A raw pointer (`*const T` or `*mut T`)
        pub Ptr(TyPtr {
            pub star_token: tokens::Star,
            pub const_token: Option<tokens::Const>,
            pub ty: Box<u64>,
        }),
        /// A reference (`&'a T` or `&'a mut T`)
        pub Rptr(TyRptr {
            pub and_token: tokens::And,
            pub lifetime: Option<Lifetime>,
            pub ty: Box<u64>,
        }),
        /// A bare function (e.g. `fn(usize) -> bool`)
        pub BareFn(TyBareFn {
            pub ty: Box<u64>,
        }),
        /// The never type (`!`)
        pub Never(TyNever {
            pub bang_token: tokens::Bang,
        }),
        /// A tuple (`(A, B, C, D, ...)`)
        pub Tup(TyTup {
            pub paren_token: tokens::Paren,
            pub tys: Delimited,
            pub lone_comma: Option<tokens::Comma>,
        }),
        /// A path (`module::module::...::Type`), optionally
        /// "qualified", e.g. `<Vec<T> as SomeTrait>::SomeType`.
        ///
        /// Type parameters are stored in the Path itself
        pub Path(TyPath {
            pub qself: Option<QSelf>,
            pub path: Path,
        }),
        /// A trait object type `Bound1 + Bound2 + Bound3`
        /// where `Bound` is a trait or a lifetime.
        pub TraitObject(TyTraitObject {
            pub bounds: Delimited,
        }),
        /// An `impl Bound1 + Bound2 + Bound3` type
        /// where `Bound` is a trait or a lifetime.
        pub ImplTrait(TyImplTrait {
            pub impl_token: tokens::Impl,
            pub bounds: Delimited,
        }),
        /// No-op; kept solely so that we can pretty-print faithfully
        pub Paren(TyParen {
            pub paren_token: tokens::Paren,
            pub ty: Box<u64>,
        }),
        /// No-op: kept solely so that we can pretty-print faithfully
        pub Group(TyGroup {
            pub group_token: tokens::Group,
            pub ty: Box<u64>,
        }),
        /// TyKind::Infer means the type should be inferred instead of it having been
        /// specified. This can appear anywhere in a type.
        pub Infer(TyInfer {
            pub underscore_token: tokens::Underscore
        }),
        /// A macro in the type position.
        pub Mac(Mac),
    }
}

ast_enum! {
    pub enum Mutability {
        Mutable(tokens::Mut),
        Immutable,
    }
}

ast_struct! {
    /// A "Path" is essentially Rust's notion of a name.
    ///
    /// It's represented as a sequence of identifiers,
    /// along with a bunch of supporting information.
    ///
    /// E.g. `std::cmp::PartialEq`
    pub struct Path {
        /// A `::foo` path, is relative to the crate root rather than current
        /// module (like paths in an import).
        pub leading_colon: Option<tokens::Colon2>,
        /// The segments in the path: the things separated by `::`.
        pub segments: Delimited,
    }
}

ast_struct! {
    /// A segment of a path: an identifier, an optional lifetime, and a set of types.
    ///
    /// E.g. `std`, `String` or `Box<T>`
    pub struct PathSegment {
        /// The identifier portion of this path segment.
        pub ident: Ident,
        /// Type/lifetime parameters attached to this path. They come in
        /// two flavors: `Path<A,B,C>` and `Path(A,B) -> C`. Note that
        /// this is more than just simple syntactic sugar; the use of
        /// parens affects the region binding rules, so we preserve the
        /// distinction.
        pub parameters: PathParameters,
    }
}

ast_enum! {
    /// Parameters of a path segment.
    ///
    /// E.g. `<A, B>` as in `Foo<A, B>` or `(A, B)` as in `Foo(A, B)`
    pub enum PathParameters {
        None,
        /// The `<'a, A, B, C>` in `foo::bar::baz::<'a, A, B, C>`
        AngleBracketed(AngleBracketedParameterData),
        /// The `(A, B)` and `C` in `Foo(A, B) -> C`
        Parenthesized(ParenthesizedParameterData),
    }
}

ast_struct! {
    /// A path like `Foo<'a, T>`
    pub struct AngleBracketedParameterData {
        pub turbofish: Option<tokens::Colon2>,
        pub lt_token: tokens::Lt,
        /// The lifetime parameters for this path segment.
        pub lifetimes: Delimited,
        /// The type parameters for this path segment, if present.
        pub types: Delimited,
        /// Bindings (equality constraints) on associated types, if present.
        ///
        /// E.g., `Foo<A=Bar>`.
        pub bindings: Delimited,
        pub gt_token: tokens::Gt,
    }
}

ast_struct! {
    /// Bind a type to an associated type: `A=Foo`.
    pub struct TypeBinding {
        pub ident: Ident,
        pub eq_token: tokens::Eq,
        pub ty: Ty,
    }
}

ast_struct! {
    /// A path like `Foo(A,B) -> C`
    pub struct ParenthesizedParameterData {
        pub paren_token: tokens::Paren,
        /// `(A, B)`
        pub inputs: Delimited,
        /// `C`
        pub output: FunctionRetTy,
    }
}

ast_struct! {
    pub struct PolyTraitRef {
        /// The `for<'a>` in `for<'a> Foo<&'a T>`
        pub bound_lifetimes: Option<BoundLifetimes>,
        /// The `Foo<&'a T>` in `<'a> Foo<&'a T>`
        pub trait_ref: Path,
    }
}

ast_struct! {
    /// The explicit Self type in a "qualified path". The actual
    /// path, including the trait and the associated item, is stored
    /// separately. `position` represents the index of the associated
    /// item qualified with this Self type.
    ///
    /// ```rust,ignore
    /// <Vec<T> as a::b::Trait>::AssociatedItem
    ///  ^~~~~     ~~~~~~~~~~~~~~^
    ///  ty        position = 3
    ///
    /// <Vec<T>>::AssociatedItem
    ///  ^~~~~    ^
    ///  ty       position = 0
    /// ```
    pub struct QSelf {
        pub lt_token: tokens::Lt,
        pub ty: Box<u64>,
        pub position: usize,
        pub as_token: Option<tokens::As>,
        pub gt_token: tokens::Gt,
    }
}

ast_struct! {
    pub struct BareFnTy {
        pub lifetimes: Option<BoundLifetimes>,
        pub unsafety: Unsafety,
        pub abi: Option<Abi>,
        pub fn_token: tokens::Fn_,
        pub paren_token: tokens::Paren,
        pub inputs: Delimited,
        pub variadic: Option<tokens::Dot3>,
        pub output: FunctionRetTy,
    }
}

ast_enum! {
    pub enum Unsafety {
        Unsafe(tokens::Unsafe),
        Normal,
    }
}

ast_struct! {
    pub struct Abi {
        pub extern_token: tokens::Extern,
        pub kind: AbiKind,
    }
}

ast_enum! {
    pub enum AbiKind {
        Named(Lit),
        Default,
    }
}

ast_struct! {
    /// An argument in a function type.
    ///
    /// E.g. `bar: usize` as in `fn foo(bar: usize)`
    pub struct BareFnArg {
        pub name: Option<(BareFnArgName, tokens::Colon)>,
        pub ty: Ty,
    }
}

ast_enum! {
    /// Names of arguments in the `BareFnArg` structure
    pub enum BareFnArgName {
        /// Argument with the provided name
        Named(Ident),
        /// Argument matched with `_`
        Wild(tokens::Underscore),
    }
}

ast_enum! {
    pub enum FunctionRetTy {
        /// Return type is not specified.
        ///
        /// Functions default to `()` and
        /// closures default to inference. Span points to where return
        /// type would be inserted.
        Default,
        /// Everything else
        Ty(Ty, tokens::RArrow),
    }
}
