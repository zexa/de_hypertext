use de_hypertext_core::derive::impl_derive_deserialize;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, DeriveInput};

#[test]
fn test_vec_t() {
    use de_hypertext::Deserializer;

    #[derive(Debug)]
    struct T {}

    impl de_hypertext::Deserializer<T> for T {
        fn from_document(
            document: &de_hypertext::scraper::ElementRef,
        ) -> Result<Self, de_hypertext::DeserializeError> {
            Ok(T {})
        }
    }

    #[derive(Debug)]
    struct VecT {
        field1: Vec<T>,
    }

    impl de_hypertext::Deserializer<Self> for VecT {
        fn from_document(
            document: &de_hypertext::scraper::ElementRef,
        ) -> Result<Self, de_hypertext::DeserializeError> {
            let field1 = {
                document
                    .select(
                        &de_hypertext::scraper::Selector::parse(".some-class").map_err(|_| {
                            de_hypertext::DeserializeError::BuildingSelectorFailed {
                                struct_name: std::any::type_name::<Self>().to_string(),
                                field: "field1".to_string(),
                                selector: ".some-class".to_string(),
                            }
                        })?,
                    )
                    .map(|document| T::from_document(&document))
                    .collect::<Result<Vec<T>, _>>()?
            };
            Ok(Self { field1 })
        }
    }

    let html = r#"
        <html>
            <body>
                <a class="some-class"></a>
                <a class="some-class"></a>
                <a class="some-class"></a>
            </body>
        </html>
    "#;

    let result = VecT::from_html(html).unwrap();

    assert_eq!(result.field1.len(), 3);
}

#[test]
fn test_vec_t_impl() {
    let input: DeriveInput = parse_quote! {
        struct VecT {
            #[de_hypertext(selector = ".some-class")]
            field1: Vec<T>,
        }
    };

    let actual = impl_derive_deserialize(input);

    let expected: TokenStream = parse_quote! {
        impl de_hypertext::Deserializer<Self> for VecT {
            fn from_document(
                document: &de_hypertext::scraper::ElementRef,
            ) -> Result<Self, de_hypertext::DeserializeError> {
                let field1 = {
                    document
                        .select(
                            &de_hypertext::scraper::Selector::parse(".some-class").map_err(|_| {
                                de_hypertext::DeserializeError::BuildingSelectorFailed {
                                    struct_name: std::any::type_name::<Self>().to_string(),
                                    field: "field1".to_string(),
                                    selector: ".some-class".to_string(),
                                }
                            })?,
                        )
                        .map(|document| T::from_document(&document))
                        .collect::<Result<Vec<T>, _>>()?
                };
                Ok(Self { field1 })
            }
        }
    };

    // crate::assert_tokens_ugly_eq(expected, actual);
    crate::assert_tokens_eq(expected, actual);
}
