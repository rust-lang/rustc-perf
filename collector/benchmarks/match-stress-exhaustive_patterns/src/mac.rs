use super::*;

ast_struct! {
    /// Represents a macro invocation. The Path indicates which macro
    /// is being invoked, and the vector of token-trees contains the source
    /// of the macro invocation.
    pub struct Mac {
        pub path: Path,
        pub bang_token: tokens::Bang,
        /// The `example` in `macro_rules! example { ... }`.
        pub ident: Option<Ident>,
        pub tokens: Vec<TokenTree>,
    }
}

pub struct TokenTree;

impl PartialEq for TokenTree {
    fn eq(&self, _other: &TokenTree) -> bool {
        todo!()
    }
}
