use core::fmt::Display;
use scraper::ElementRef;
use std::error::Error;

/// de_hypertext is a rust framework for deserializing html into structs.
///
/// ```rust
/// #[derive(Debug, de_hypertext::Deserialize)]
/// struct MyStruct {
///     #[de_hypertext(".title")]
///     title: String,
/// }
///
/// fn main() {
///     let html = r#"<div><h1 class="title">Example Domain</h1></div>"#;
///     let result = MyStruct::from_html(html).unwrap()
///     println!("{result:#?}");
/// }
/// ```
///
/// If an element is not found, Error will be returned. This functionality can
/// be overwritten by using Option<T>, which will return Some(T) when
/// the element is found or None if it is not.
///
/// usize may be used with `#[de_hypertext(count)]` to return the amount of
/// times an element was found.
///
/// The first match is used. Except when requesting Vec<T>.
///
///
/// `de-hypertext-finder` is an app that helps you find updated selectors
/// for structs that derive `de_hypertext::Deserialize`.
///
pub trait Deserializer<T> {
    fn from_document(document: &ElementRef) -> Result<T, Box<dyn Error>>;

    fn from_html(html: &str) -> Result<T, Box<dyn Error>> {
        let html = scraper::Html::parse_document(html);
        let document = html.root_element();
        Self::from_document(&document)
    }
}

#[derive(Debug)]
pub enum DeserializeError {
    ElementNotFoud {
        struct_name: String,
        field: String,
        selector: String,
    },
    AttributeNotFound {
        struct_name: String,
        field: String,
        selector: String,
        attribute: String,
    },
}

impl Display for DeserializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{self:?}"))
    }
}

impl Error for DeserializeError {}
