use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, DeriveInput};

fn format_code(code: TokenStream) -> String {
    let code = code.to_string();
    code.replace("{", "{\n")
        .replace("}", "\n}")
        .replace(";", ";\n")
}

#[test]
fn test_option_string_impl() {
    let input: DeriveInput = parse_quote! {
        struct OptionStringSelector {
            #[de_hypertext(selector = "a")]
            field1: Option<String>,
        }
    };
    let actual = de_hypertext_core::derive::impl_derive_deserialize(input);
    let expected: TokenStream = quote! {
        impl de_hypertext::Deserializer<Self> for OptionStringSelector {
            fn from_document(
                document: &de_hypertext::scraper::ElementRef,
            ) -> Result<Self, de_hypertext::DeserializeError> {
                let field1 = {
                    let selector = de_hypertext::scraper::Selector::parse("a").map_err(|_| {
                        de_hypertext::DeserializeError::BuildingSelectorFailed {
                            struct_name: std::any::type_name::<Self>().to_string(),
                            field: "field1".to_string(),
                            selector: "a".to_string(),
                        }
                    })?;
                    document
                        .select(&selector)
                        .next()
                        .map(|document| {document.text().collect::<String>().to_string()})
                };
                Ok(Self { field1, })
            }
        }
    };
    let actual = prettyplease::unparse(&syn::parse2(actual).unwrap());
    let expected = prettyplease::unparse(&syn::parse2(expected).unwrap());
    pretty_assertions::assert_eq!(actual, expected)
}
