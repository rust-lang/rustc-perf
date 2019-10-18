extern crate proc_macro;

use proc_macro::{Ident, Punct, Spacing, Span, TokenStream, TokenTree};
use std::iter::once;

const N: u32 = 2000;

#[proc_macro]
pub fn bench(_input: TokenStream) -> TokenStream {
    let span = Span::call_site();
    let mut tokens = TokenStream::new();
    for _ in 0..N {
        // Similar to what is emitted by `quote`.
        tokens.extend(once(TokenTree::Ident(Ident::new("core", span))));
        tokens.extend(once(TokenTree::Punct(Punct::new(':', Spacing::Joint))));
        tokens.extend(once(TokenTree::Punct(Punct::new(':', Spacing::Alone))));
        tokens.extend(once(TokenTree::Ident(Ident::new("option", span))));
        tokens.extend(once(TokenTree::Punct(Punct::new(':', Spacing::Joint))));
        tokens.extend(once(TokenTree::Punct(Punct::new(':', Spacing::Alone))));
        tokens.extend(once(TokenTree::Ident(Ident::new("Option", span))));
        tokens.extend(once(TokenTree::Punct(Punct::new(':', Spacing::Joint))));
        tokens.extend(once(TokenTree::Punct(Punct::new(':', Spacing::Alone))));
        tokens.extend(once(TokenTree::Ident(Ident::new("None", span))));
        tokens.extend(once(TokenTree::Punct(Punct::new(',', Spacing::Joint))));
    }

    TokenStream::new()
}
