use crate::assert_tokens_eq;
use de_hypertext_core::derive::impl_derive_deserialize;
use quote::quote;
use syn::{parse_quote, DeriveInput};

#[test]
fn test_transform() {
    struct Transforming {
        field1: String,
    }

    impl de_hypertext::Deserializer for Transforming {
        fn from_document(
            document: &de_hypertext::scraper::ElementRef,
        ) -> Result<Self, de_hypertext::DeserializeError> {
            let field1 = {
                document
                    .select(&de_hypertext::scraper::Selector::parse("a").map_err(|_| {
                        de_hypertext::DeserializeError::BuildingSelectorFailed {
                            struct_name: std::any::type_name::<Self>().to_string(),
                            field: "field1".to_string(),
                            selector: "span".to_string(),
                        }
                    })?)
                    .next()
                    .map(|element| element.text().collect::<String>())
                    .map(|x| x.replace("€‎", "").trim().to_string())
                    .ok_or(de_hypertext::DeserializeError::ElementNotFoud {
                        struct_name: std::any::type_name::<Self>().to_string(),
                        field: "field1".to_string(),
                        selector: "span".to_string(),
                    })?
            };

            Ok(Self { field1 })
        }
    }
}

#[test]
fn test_transform_impl() {
    let input: DeriveInput = parse_quote! {
        struct Transforming {
            #[de_hypertext(
                selector = "span",
                transform = |x: String| x.replace("€‎", "").trim().to_string()
            )]
            field1: String,
        }
    };
    let actual = impl_derive_deserialize(input);
    let expected = quote! {
        impl de_hypertext::Deserializer for Transforming {
            fn from_document(
                document: &de_hypertext::scraper::ElementRef,
            ) -> Result<Self, de_hypertext::DeserializeError> {
                let field1 = {
                    let value = document
                        .select(&de_hypertext::scraper::Selector::parse("span").map_err(|_| {
                            de_hypertext::DeserializeError::BuildingSelectorFailed {
                                struct_name: std::any::type_name::<Self>().to_string(),
                                field: "field1".to_string(),
                                selector: "span".to_string(),
                            }
                        })?)
                        .next()
                        .ok_or(de_hypertext::DeserializeError::ElementNotFoud {
                            struct_name: std::any::type_name::<Self>().to_string(),
                            field: "field1".to_string(),
                            selector: "span".to_string(),
                        })?
                        .text()
                        .collect::<String>()
                        .to_string();
                    let transform = |x: String| x.replace("€‎", "").trim().to_string();
                    let value = transform(value);
                    value
                };

                Ok(Self { field1 })
            }
        }
    };
    assert_tokens_eq(expected, actual);
}

#[test]
fn test_transform_functionality() {
    #[derive(de_hypertext::Deserialize)]
    struct Transforming {
        #[de_hypertext(
            selector = "span",
            transform = |x: String| x.replace("€‎", "").trim().to_string()
        )]
        field1: String,
    }
    let html = r#"
        <html>
            <body><span>5.22€‎</span></body>
        </html>
    "#;
    let result = <Transforming as de_hypertext::Deserializer>::from_html(html).unwrap();
    assert_eq!(result.field1, "5.22");
}
