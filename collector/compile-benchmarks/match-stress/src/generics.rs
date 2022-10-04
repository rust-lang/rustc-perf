use super::*;
use delimited::Delimited;

ast_struct! {
    /// Represents lifetimes and type parameters attached to a declaration
    /// of a function, enum, trait, etc.
    pub struct Generics {
        pub lt_token: Option<tokens::Lt>,
        pub gt_token: Option<tokens::Gt>,
        pub lifetimes: Delimited,
        pub ty_params: Delimited,
        pub where_clause: WhereClause,
    }
}

ast_struct! {
    /// A set of bound lifetimes, e.g. `for<'a, 'b, 'c>`
    pub struct BoundLifetimes {
        pub for_token: tokens::For,
        pub lt_token: tokens::Lt,
        pub lifetimes: Delimited,
        pub gt_token: tokens::Gt,
    }
}

ast_enum! {
    /// The AST represents all type param bounds as types.
    /// `typeck::collect::compute_bounds` matches these against
    /// the "special" built-in traits (see `middle::lang_items`) and
    /// detects Copy, Send and Sync.
    pub enum TyParamBound {
        Trait(PolyTraitRef, TraitBoundModifier),
        Region(Lifetime),
    }
}

ast_enum! {
    /// A modifier on a bound, currently this is only used for `?Sized`, where the
    /// modifier is `Maybe`. Negative bounds should also be handled here.
    pub enum TraitBoundModifier {
        None,
        Maybe(tokens::Question),
    }
}

ast_struct! {
    /// A `where` clause in a definition
    pub struct WhereClause {
        pub where_token: Option<tokens::Where>,
        pub predicates: Delimited,
    }
}

ast_enum_of_structs! {
    /// A single predicate in a `where` clause
    pub enum WherePredicate {
        /// A type binding, e.g. `for<'c> Foo: Send+Clone+'c`
        pub BoundPredicate(WhereBoundPredicate {
            /// Any lifetimes from a `for` binding
            pub bound_lifetimes: Option<BoundLifetimes>,
            /// The type being bounded
            pub bounded_ty: Ty,
            pub colon_token: tokens::Colon,
            /// Trait and lifetime bounds (`Clone+Send+'static`)
            pub bounds: Delimited,
        }),

        /// A lifetime predicate, e.g. `'a: 'b+'c`
        pub RegionPredicate(WhereRegionPredicate {
            pub lifetime: Lifetime,
            pub colon_token: Option<tokens::Colon>,
            pub bounds: Delimited,
        }),

        /// An equality predicate (unsupported)
        pub EqPredicate(WhereEqPredicate {
            pub lhs_ty: Ty,
            pub eq_token: tokens::Eq,
            pub rhs_ty: Ty,
        }),
    }
}
