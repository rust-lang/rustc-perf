use super::*;
use delimited::Delimited;

ast_enum! {
    /// Distinguishes between Attributes that decorate items and Attributes that
    /// are contained as statements within items. These two cases need to be
    /// distinguished for pretty-printing.
    pub enum AttrStyle {
        /// Attribute of the form `#[...]`.
        Outer,

        /// Attribute of the form `#![...]`.
        Inner(tokens::Bang),
    }
}

ast_enum_of_structs! {
    /// A compile-time attribute item.
    ///
    /// E.g. `#[test]`, `#[derive(..)]` or `#[feature = "foo"]`
    pub enum MetaItem {
        /// Term meta item.
        ///
        /// E.g. `test` as in `#[test]`
        pub Term(Ident),

        /// List meta item.
        ///
        /// E.g. `derive(..)` as in `#[derive(..)]`
        pub List(MetaItemList {
            /// Name of this attribute.
            ///
            /// E.g. `derive` in `#[derive(..)]`
            pub ident: Ident,

            pub paren_token: tokens::Paren,

            /// Arguments to this attribute
            ///
            /// E.g. `..` in `#[derive(..)]`
            pub nested: Delimited,
        }),

        /// Name-value meta item.
        ///
        /// E.g. `feature = "foo"` as in `#[feature = "foo"]`
        pub NameValue(MetaNameValue {
            /// Name of this attribute.
            ///
            /// E.g. `feature` in `#[feature = "foo"]`
            pub ident: Ident,

            pub eq_token: tokens::Eq,

            /// Arguments to this attribute
            ///
            /// E.g. `"foo"` in `#[feature = "foo"]`
            pub lit: Lit,
        }),
    }
}

ast_enum_of_structs! {
    /// Possible values inside of compile-time attribute lists.
    ///
    /// E.g. the '..' in `#[name(..)]`.
    pub enum NestedMetaItem {
        /// A full `MetaItem`.
        ///
        /// E.g. `Copy` in `#[derive(Copy)]` would be a `MetaItem::Term(Ident::from("Copy"))`.
        pub MetaItem(MetaItem),

        /// A Rust literal.
        ///
        /// E.g. `"name"` in `#[rename("name")]`.
        pub Literal(Lit),
    }
}
