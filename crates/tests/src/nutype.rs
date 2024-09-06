use de_hypertext::Deserialize;

#[test]
fn test_nutype() {
    use nutype::nutype;

    #[nutype(
        sanitize(trim, lowercase),
        validate(not_empty, len_char_max = 20),
        derive(Debug, PartialEq, Clone)
    )]
    struct NuType(String);

    #[derive(Debug)]
    // #[derive(Deserialize)]
    struct NutypeUsing {
        // #[de_hypertext(selector = "", nutype)]
        field1: NuType,
    }

    impl de_hypertext::Deserializer<Self> for NutypeUsing {
        fn from_document(
            document: &de_hypertext::scraper::ElementRef,
        ) -> Result<Self, de_hypertext::DeserializeError> {
            let field1 = document
                .select(
                    &de_hypertext::scraper::Selector::parse(".some-class").map_err(|_| {
                        de_hypertext::DeserializeError::BuildingSelectorFailed {
                            struct_name: std::any::type_name::<Self>().to_string(),
                            field: "field1".to_string(),
                            selector: ".some-class".to_string(),
                        }
                    })?,
                )
                .next()
                .map(|document| document.text().collect::<String>())
                .ok_or_else(|| de_hypertext::DeserializeError::ElementNotFoud {
                    struct_name: std::any::type_name::<Self>().to_string(),
                    field: "field1".to_string(),
                    selector: ".some-class".to_string(),
                })
                .map(|raw_value| {
                    NuType::try_new(raw_value)
                        .map_err(|e| de_hypertext::DeserializeError::NuType(Box::new(e)))
                })
                .flatten()?;

            Ok(Self { field1 })
        }
    }

    let html = r#"
        <html>
            <body>
                <a class="some-class"></a>
            </body>
        </html>
    "#;

    let result = <NutypeUsing as de_hypertext::Deserializer>::from_html(html).unwrap_err();
    assert_eq!(result.to_string(), "NuType(NotEmptyViolated)");
}
