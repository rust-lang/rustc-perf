use super::*;
use delimited::Delimited;

ast_struct! {
    /// Things that can appear directly inside of a module.
    pub struct Item {
        pub attrs: Vec<u64>,
        pub node: ItemKind,
    }
}

ast_enum_of_structs! {
    pub enum ItemKind {
        /// An `extern crate` item, with optional original crate name.
        ///
        /// E.g. `extern crate foo` or `extern crate foo_bar as foo`
        pub ExternCrate(ItemExternCrate {
            pub vis: Visibility,
            pub extern_token: tokens::Extern,
            pub crate_token: tokens::Crate,
            pub ident: Ident,
            pub rename: Option<(tokens::As, Ident)>,
            pub semi_token: tokens::Semi,
        }),
        /// A use declaration (`use` or `pub use`) item.
        ///
        /// E.g. `use foo;`, `use foo::bar;` or `use foo::bar as FooBar;`
        pub Use(ItemUse {
            pub vis: Visibility,
            pub use_token: tokens::Use,
            pub path: Box<u64>,
            pub semi_token: tokens::Semi,
        }),
        /// A static item (`static` or `pub static`).
        ///
        /// E.g. `static FOO: i32 = 42;` or `static FOO: &'static str = "bar";`
        pub Static(ItemStatic {
            pub vis: Visibility,
            pub static_token: tokens::Static,
            pub mutbl: Mutability,
            pub ident: Ident,
            pub colon_token: tokens::Colon,
            pub ty: Box<u64>,
            pub eq_token: tokens::Eq,
            pub expr: Box<u64>,
            pub semi_token: tokens::Semi,
        }),
        /// A constant item (`const` or `pub const`).
        ///
        /// E.g. `const FOO: i32 = 42;`
        pub Const(ItemConst {
            pub vis: Visibility,
            pub const_token: tokens::Const,
            pub ident: Ident,
            pub colon_token: tokens::Colon,
            pub ty: Box<u64>,
            pub eq_token: tokens::Eq,
            pub expr: Box<u64>,
            pub semi_token: tokens::Semi,
        }),
        /// A function declaration (`fn` or `pub fn`).
        ///
        /// E.g. `fn foo(bar: usize) -> usize { .. }`
        pub Fn(ItemFn {
            pub vis: Visibility,
            pub constness: Constness,
            pub unsafety: Unsafety,
            pub abi: Option<Abi>,
            pub decl: Box<u64>,
            pub ident: Ident,
            pub block: Box<u64>,
        }),
        /// A module declaration (`mod` or `pub mod`).
        ///
        /// E.g. `mod foo;` or `mod foo { .. }`
        pub Mod(ItemMod {
            pub vis: Visibility,
            pub mod_token: tokens::Mod,
            pub ident: Ident,
            pub content: Option<(tokens::Brace, Vec<u64>)>,
            pub semi: Option<tokens::Semi>,
        }),
        /// An external module (`extern` or `pub extern`).
        ///
        /// E.g. `extern {}` or `extern "C" {}`
        pub ForeignMod(ItemForeignMod {
            pub abi: Abi,
            pub brace_token: tokens::Brace,
            pub items: Vec<u64>,
        }),
        /// A type alias (`type` or `pub type`).
        ///
        /// E.g. `type Foo = Bar<u8>;`
        pub Ty(ItemTy {
            pub vis: Visibility,
            pub type_token: tokens::Type,
            pub ident: Ident,
            pub generics: Generics,
            pub eq_token: tokens::Eq,
            pub ty: Box<u64>,
            pub semi_token: tokens::Semi,
        }),
        /// An enum definition (`enum` or `pub enum`).
        ///
        /// E.g. `enum Foo<A, B> { C<A>, D<B> }`
        pub Enum(ItemEnum {
            pub vis: Visibility,
            pub enum_token: tokens::Enum,
            pub ident: Ident,
            pub generics: Generics,
            pub brace_token: tokens::Brace,
            pub variants: Delimited,
        }),
        /// A struct definition (`struct` or `pub struct`).
        ///
        /// E.g. `struct Foo<A> { x: A }`
        pub Struct(ItemStruct {
            pub vis: Visibility,
            pub struct_token: tokens::Struct,
            pub ident: Ident,
            pub generics: Generics,
            pub data: VariantData,
            pub semi_token: Option<tokens::Semi>,
        }),
        /// A union definition (`union` or `pub union`).
        ///
        /// E.g. `union Foo<A, B> { x: A, y: B }`
        pub Union(ItemUnion {
            pub vis: Visibility,
            pub union_token: tokens::Union,
            pub ident: Ident,
            pub generics: Generics,
            pub data: VariantData,
        }),
        /// A Trait declaration (`trait` or `pub trait`).
        ///
        /// E.g. `trait Foo { .. }` or `trait Foo<T> { .. }`
        pub Trait(ItemTrait {
            pub vis: Visibility,
            pub unsafety: Unsafety,
            pub trait_token: tokens::Trait,
            pub ident: Ident,
            pub generics: Generics,
            pub colon_token: Option<tokens::Colon>,
            pub supertraits: Delimited,
            pub brace_token: tokens::Brace,
            pub items: Vec<u64>,
        }),
        /// Default trait implementation.
        ///
        /// E.g. `impl Trait for .. {}` or `impl<T> Trait<T> for .. {}`
        pub DefaultImpl(ItemDefaultImpl {
            pub unsafety: Unsafety,
            pub impl_token: tokens::Impl,
            pub path: Path,
            pub for_token: tokens::For,
            pub dot2_token: tokens::Dot2,
            pub brace_token: tokens::Brace,
        }),
        /// An implementation.
        ///
        /// E.g. `impl<A> Foo<A> { .. }` or `impl<A> Trait for Foo<A> { .. }`
        pub Impl(ItemImpl {
            pub defaultness: Defaultness,
            pub unsafety: Unsafety,
            pub impl_token: tokens::Impl,
            pub generics: Generics,
            /// Trait this impl implements.
            pub trait_: Option<(ImplPolarity, Path, tokens::For)>,
            /// The Self type of the impl.
            pub self_ty: Box<u64>,
            pub brace_token: tokens::Brace,
            pub items: Vec<u64>,
        }),
        /// A macro invocation (which includes macro definition).
        ///
        /// E.g. `macro_rules! foo { .. }` or `foo!(..)`
        pub Mac(Mac),
    }
}

ast_enum_of_structs! {
    pub enum ViewPath {
        /// `foo::bar::baz as quux`
        ///
        /// or just
        ///
        /// `foo::bar::baz` (with `as baz` implicitly on the right)
        pub Simple(PathSimple {
            pub path: Path,
            pub as_token: Option<tokens::As>,
            pub rename: Option<Ident>,
        }),

        /// `foo::bar::*`
        pub Glob(PathGlob {
            pub path: Path,
            pub colon2_token: Option<tokens::Colon2>,
            pub star_token: tokens::Star,
        }),

        /// `foo::bar::{a, b, c}`
        pub List(PathList {
            pub path: Path,
            pub colon2_token: tokens::Colon2,
            pub brace_token: tokens::Brace,
            pub items: Delimited,
        }),
    }
}

ast_struct! {
    pub struct PathListItem {
        pub name: Ident,
        /// renamed in list, e.g. `use foo::{bar as baz};`
        pub rename: Option<Ident>,
        pub as_token: Option<tokens::As>,
    }
}

ast_enum! {
    pub enum Constness {
        Const(tokens::Const),
        NotConst,
    }
}

ast_enum! {
    pub enum Defaultness {
        Default(tokens::Default_),
        Final,
    }
}

ast_struct! {
    pub struct ForeignItem {
        pub ident: Ident,
        pub attrs: Vec<u64>,
        pub node: ForeignItemKind,
        pub vis: Visibility,
        pub semi_token: tokens::Semi,
    }
}

ast_enum_of_structs! {
    /// An item within an `extern` block
    pub enum ForeignItemKind {
        /// A foreign function
        pub Fn(ForeignItemFn {
            pub decl: Box<u64>,
        }),
        /// A foreign static item (`static ext: u8`)
        pub Static(ForeignItemStatic {
            pub static_token: tokens::Static,
            pub ty: Box<u64>,
            pub colon_token: tokens::Colon,
            pub mutbl: Mutability,
        }),
    }
}

ast_struct! {
    /// Represents an item declaration within a trait declaration,
    /// possibly including a default implementation. A trait item is
    /// either required (meaning it doesn't have an implementation, just a
    /// signature) or provided (meaning it has a default implementation).
    pub struct TraitItem {
        pub attrs: Vec<u64>,
        pub node: TraitItemKind,
    }
}

ast_enum_of_structs! {
    pub enum TraitItemKind {
        pub Const(TraitItemConst {
            pub const_token: tokens::Const,
            pub ident: Ident,
            pub colon_token: tokens::Colon,
            pub ty: Ty,
            pub default: Option<(tokens::Eq, Expr)>,
            pub semi_token: tokens::Semi,
        }),
        pub Method(TraitItemMethod {
            pub sig: MethodSig,
            pub default: Option<Block>,
            pub semi_token: Option<tokens::Semi>,
        }),
        pub Type(TraitItemType {
            pub type_token: tokens::Type,
            pub ident: Ident,
            pub colon_token: Option<tokens::Colon>,
            pub bounds: Delimited,
            pub default: Option<(tokens::Eq, Ty)>,
            pub semi_token: tokens::Semi,
        }),
        pub Macro(Mac),
    }
}

ast_enum! {
    pub enum ImplPolarity {
        /// `impl Trait for Type`
        Positive,
        /// `impl !Trait for Type`
        Negative(tokens::Bang),
    }
}

ast_struct! {
    pub struct ImplItem {
        pub attrs: Vec<u64>,
        pub node: ImplItemKind,
    }
}

ast_enum_of_structs! {
    pub enum ImplItemKind {
        pub Const(ImplItemConst {
            pub vis: Visibility,
            pub defaultness: Defaultness,
            pub const_token: tokens::Const,
            pub ident: Ident,
            pub colon_token: tokens::Colon,
            pub ty: Ty,
            pub eq_token: tokens::Eq,
            pub expr: Expr,
            pub semi_token: tokens::Semi,
        }),
        pub Method(ImplItemMethod {
            pub vis: Visibility,
            pub defaultness: Defaultness,
            pub sig: MethodSig,
            pub block: Block,
        }),
        pub Type(ImplItemType {
            pub vis: Visibility,
            pub defaultness: Defaultness,
            pub type_token: tokens::Type,
            pub ident: Ident,
            pub eq_token: tokens::Eq,
            pub ty: Ty,
            pub semi_token: tokens::Semi,
        }),
        pub Macro(Mac),
    }
}

ast_struct! {
    /// Represents a method's signature in a trait declaration,
    /// or in an implementation.
    pub struct MethodSig {
        pub constness: Constness,
        pub unsafety: Unsafety,
        pub abi: Option<Abi>,
        pub ident: Ident,
        pub decl: FnDecl,
    }
}

ast_struct! {
    /// Header (not the body) of a function declaration.
    ///
    /// E.g. `fn foo(bar: baz)`
    pub struct FnDecl {
        pub fn_token: tokens::Fn_,
        pub paren_token: tokens::Paren,
        pub inputs: Delimited,
        pub output: FunctionRetTy,
        pub generics: Generics,
        pub variadic: bool,
        pub dot_tokens: Option<tokens::Dot3>,
    }
}

ast_enum_of_structs! {
    /// An argument in a function header.
    ///
    /// E.g. `bar: usize` as in `fn foo(bar: usize)`
    pub enum FnArg {
        pub SelfRef(ArgSelfRef {
            pub and_token: tokens::And,
            pub self_token: tokens::Self_,
            pub lifetime: Option<Lifetime>,
            pub mutbl: Mutability,
        }),
        pub SelfValue(ArgSelf {
            pub mutbl: Mutability,
            pub self_token: tokens::Self_,
        }),
        pub Captured(ArgCaptured {
            pub pat: Pat,
            pub colon_token: tokens::Colon,
            pub ty: Ty,
        }),
        pub Ignored(Ty),
    }
}
