use de_hypertext_core::derive::impl_derive_deserialize;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, DeriveInput};

#[test]
fn test_option_string_impl() {
    let input: DeriveInput = parse_quote! {
        struct OptionStringSelector {
            #[de_hypertext(selector = "a")]
            field1: Option<String>,
        }
    };
    let expected: TokenStream = parse_quote! {
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
                        .map(|document| document.text().collect::<String>().to_string())
                };
                Ok(Self { field1, })
            }
        }
    };
    let actual = impl_derive_deserialize(input);
    crate::assert_tokens_eq(expected, actual);
}

#[test]
fn test_option_string_attribute_impl() {
    let input: DeriveInput = parse_quote! {
        struct OptionStringSelectorAttribute {
            #[de_hypertext(selector = "a", attribute = "href")]
            field1: Option<String>,
        }
    };
    let expected: TokenStream = quote! {
        impl de_hypertext::Deserializer<Self> for OptionStringSelectorAttribute {
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
                    match document.select(&selector).next() {
                        Some(document) => document
                            .value()
                            .attr("href")
                            .map(|attribute| attribute.trim().to_string()),
                        None => None,
                    }
                };
                Ok(Self { field1 })
            }
        }
    };
    let actual = impl_derive_deserialize(input);
    crate::assert_tokens_eq(expected, actual);
}
