use de_hypertext::{DeserializeError, Deserializer};
use de_hypertext_core::derive::impl_derive_deserialize;
use proc_macro2::TokenStream;
use syn::{parse_quote, DeriveInput};

use crate::{assert_tokens_eq, assert_tokens_ugly_eq};

#[test]
fn test_no_selector() {
    struct NoSelector {
        field1: String,
    }

    impl de_hypertext::Deserializer for NoSelector {
        fn from_document(
            document: &de_hypertext::scraper::ElementRef,
        ) -> Result<Self, de_hypertext::DeserializeError> {
            let field1 = { document.text().collect::<String>() };
            Ok(Self { field1 })
        }
    }

    let output = NoSelector::from_html("<html><body>hello world</body></html>").unwrap();
    assert_eq!("hello world", output.field1);
}

#[test]
fn test_no_selector_impl() {
    let input: DeriveInput = parse_quote! {
        struct NoSelector {
            field1: String,
        }
    };
    let expected: TokenStream = parse_quote! {
        impl de_hypertext::Deserializer for NoSelector {
            fn from_document(
                document: &de_hypertext::scraper::ElementRef,
            ) -> Result<Self, de_hypertext::DeserializeError> {
                let field1 = {
                    let value = document.text().collect::<String>().to_string();
                    value
                };
                Ok(Self { field1, })
            }
        }
    };
    let actual = impl_derive_deserialize(input);
    assert_tokens_eq(expected, actual);
}

// #[test]
// fn test_no_selector_but_with_attribute() {
//     struct NoSelectorButWithAttribute {
//         // #[de_hypertext(attribute = "href")]
//         field1: String,
//     }

//     impl de_hypertext::Deserializer for NoSelectorButWithAttribute {
//         fn from_document(
//             document: &de_hypertext::scraper::ElementRef,
//         ) -> Result<Self, de_hypertext::DeserializeError> {
//             let field1 = {
//                 document
//                     .value()
//                     .attr("href")
//                     .ok_or(DeserializeError::AttributeNotFound {
//                         struct_name: std::any::type_name::<Self>().to_string(),
//                         field: "title".to_string(),
//                         selector: None,
//                         attribute: "href".to_string(),
//                     })?
//                     .to_string()
//             };
//             Ok(Self { field1 })
//         }
//     }

//     let html = "<a href=\"#\">hello world</a>";
// }

#[test]
fn test_no_selector_but_with_attribute_impl() {
    let input: DeriveInput = parse_quote! {
        struct NoSelectorButWithAttribute {
            #[de_hypertext(attribute = "href")]
            field1: String,
        }
    };
    let expected: TokenStream = parse_quote! {
        impl de_hypertext::Deserializer for NoSelectorButWithAttribute {
            fn from_document(
                document: &de_hypertext::scraper::ElementRef,
            ) -> Result<Self, de_hypertext::DeserializeError> {
                let field1 = {
                    let value = document
                        .value()
                        .attr("href")
                        .ok_or(de_hypertext::DeserializeError::AttributeNotFound {
                            struct_name: std::any::type_name::<Self>().to_string(),
                            field: "field1".to_string(),
                            selector: None,
                            attribute: "href".to_string(),
                        })?
                        .to_string();
                    value
                };
                Ok(Self { field1 })
            }
        }
    };
    let actual = impl_derive_deserialize(input);
    assert_tokens_eq(expected, actual);
}
