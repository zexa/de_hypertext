#[cfg(test)]
mod option_string;

use pretty_assertions::assert_eq;
use proc_macro2::TokenStream;

pub fn formatted(tokens: TokenStream) -> String {
    prettyplease::unparse(&syn::parse2(tokens).expect("failed to parse tokens"))
}

pub fn assert_tokens_eq(left: TokenStream, right: TokenStream) {
    assert_eq!(formatted(left), formatted(right))
}
