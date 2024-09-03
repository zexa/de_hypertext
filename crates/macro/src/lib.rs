use syn::parse_macro_input;
use syn::DeriveInput;

#[proc_macro_derive(Deserialize, attributes(de_hypertext))]
pub fn derive_deserialize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    de_hypertext_core::derive::impl_derive_deserialize(input).into()
}
