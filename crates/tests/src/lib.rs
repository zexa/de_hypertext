#[cfg(test)]
mod option_string;
#[cfg(test)]
mod string;

use pretty_assertions::assert_eq;
use proc_macro2::TokenStream;

pub fn formatted(tokens: TokenStream) -> String {
    prettyplease::unparse(&syn::parse2(tokens).expect("failed to parse tokens"))
}

pub fn assert_tokens_eq(left: TokenStream, right: TokenStream) {
    assert_eq!(formatted(left), formatted(right))
}

pub fn assert_tokens_ugly_eq(left: TokenStream, right: TokenStream) {
    assert_eq!(left.to_string(), right.to_string())
}
