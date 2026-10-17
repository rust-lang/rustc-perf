use super::*;
use delimited::Delimited;

ast_struct! {
    /// An enum variant.
    pub struct Variant {
        /// Name of the variant.
        pub ident: Ident,

        /// Attributes tagged on the variant.
        pub attrs: Vec<u64>,

        /// Type of variant.
        pub data: VariantData,

        /// Explicit discriminant, e.g. `Foo = 1`
        pub discriminant: Option<Expr>,

        pub eq_token: Option<tokens::Eq>,
    }
}

ast_enum! {
    /// Data stored within an enum variant or struct.
    pub enum VariantData {
        /// Struct variant, e.g. `Point { x: f64, y: f64 }`.
        Struct(Delimited, tokens::Brace),

        /// Tuple variant, e.g. `Some(T)`.
        Tuple(Delimited, tokens::Paren),

        /// Unit variant, e.g. `None`.
        Unit,
    }
}

ast_struct! {
    /// A field of a struct or enum variant.
    pub struct Field {
        /// Name of the field, if any.
        ///
        /// Fields of tuple structs have no names.
        pub ident: Option<Ident>,

        /// Visibility of the field.
        pub vis: Visibility,

        /// Attributes tagged on the field.
        pub attrs: Vec<u64>,

        /// Type of the field.
        pub ty: Ty,

        pub colon_token: Option<tokens::Colon>,
    }
}

ast_enum_of_structs! {
    /// Visibility level of an item.
    pub enum Visibility {
        /// Public, i.e. `pub`.
        pub Public(VisPublic {
            pub pub_token: tokens::Pub,
        }),

        /// Crate-visible, i.e. `pub(crate)`.
        pub Crate(VisCrate {
            pub pub_token: tokens::Pub,
            pub paren_token: tokens::Paren,
            pub crate_token: tokens::Crate,
        }),

        /// Restricted, e.g. `pub(self)` or `pub(super)` or `pub(in some::module)`.
        pub Restricted(VisRestricted {
            pub pub_token: tokens::Pub,
            pub paren_token: tokens::Paren,
            pub in_token: Option<tokens::In>,
            pub path: Box<u64>,
        }),

        /// Inherited, i.e. private.
        pub Inherited(VisInherited {}),
    }
}
