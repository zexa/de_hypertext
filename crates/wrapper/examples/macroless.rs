#![allow(dead_code)]

use de_hypertext::DeserializeError;
use de_hypertext::Deserializer;
use std::error::Error;

#[derive(Debug)]
struct BooksPage {
    title: String,
    items: Vec<BookItem>,
}

#[derive(Debug)]
struct BookItem {
    url: String,
    name: String,
    price: String,
    stars: String,
}

impl Deserializer<Self> for BooksPage {
    fn from_document(document: &scraper::ElementRef) -> Result<Self, DeserializeError> {
        let title = {
            let selector = scraper::Selector::parse("title").map_err(|_| {
                DeserializeError::BuildingSelectorFailed {
                    struct_name: std::any::type_name::<Self>().to_string(),
                    field: "title".to_string(),
                    selector: "title".to_string(),
                }
            })?;
            document
                .select(&selector)
                .next()
                .ok_or(DeserializeError::ElementNotFoud {
                    struct_name: std::any::type_name::<Self>().to_string(),
                    field: "title".to_string(),
                    selector: "title".to_string(),
                })?
                .text()
                .collect::<String>()
                .trim()
                .to_string()
        };
        let items = {
            let selector = scraper::Selector::parse("riw > li").map_err(|_| {
                DeserializeError::BuildingSelectorFailed {
                    struct_name: std::any::type_name::<Self>().to_string(),
                    field: "items".to_string(),
                    selector: "row > li".to_string(),
                }
            })?;
            document
                .select(&selector)
                .into_iter()
                .map(|document| BookItem::from_document(&document))
                .collect::<Result<Vec<BookItem>, _>>()?
        };
        Ok(BooksPage { title, items })
    }
}

impl Deserializer<Self> for BookItem {
    fn from_document(document: &scraper::ElementRef) -> Result<Self, DeserializeError> {
        let url = {
            let selector = scraper::Selector::parse("h3 > a").map_err(|_| {
                DeserializeError::BuildingSelectorFailed {
                    struct_name: std::any::type_name::<Self>().to_string(),
                    field: "url".to_string(),
                    selector: "h3 > a".to_string(),
                }
            })?;
            document
                .select(&selector)
                .next()
                .ok_or(DeserializeError::ElementNotFoud {
                    struct_name: std::any::type_name::<Self>().to_string(),
                    field: "name".to_string(),
                    selector: "h3 > a".to_string(),
                })?
                .value()
                .attr("href")
                .ok_or(DeserializeError::AttributeNotFound {
                    struct_name: std::any::type_name::<Self>().to_string(),
                    field: "url".to_string(),
                    selector: Some("h3 > a".to_string()),
                    attribute: "href".to_string(),
                })?
                .to_string()
        };
        let name = {
            let selector = scraper::Selector::parse("h3 > a").map_err(|_| {
                DeserializeError::BuildingSelectorFailed {
                    struct_name: std::any::type_name::<Self>().to_string(),
                    field: "name".to_string(),
                    selector: "h3 > a".to_string(),
                }
            })?;
            document
                .select(&selector)
                .next()
                .ok_or(DeserializeError::ElementNotFoud {
                    struct_name: std::any::type_name::<Self>().to_string(),
                    field: "name".to_string(),
                    selector: "h3 > a".to_string(),
                })?
                .text()
                .collect()
        };
        let price = {
            let selector = scraper::Selector::parse(".price_color").map_err(|_| {
                DeserializeError::BuildingSelectorFailed {
                    struct_name: std::any::type_name::<Self>().to_string(),
                    field: "price".to_string(),
                    selector: ".price_color".to_string(),
                }
            })?;
            document
                .select(&selector)
                .next()
                .ok_or(DeserializeError::ElementNotFoud {
                    struct_name: std::any::type_name::<Self>().to_string(),
                    field: "price".to_string(),
                    selector: ".price_color".to_string(),
                })?
                .text()
                .collect()
        };
        let stars = {
            let selector = scraper::Selector::parse(".star-rating").map_err(|_| {
                DeserializeError::BuildingSelectorFailed {
                    struct_name: std::any::type_name::<Self>().to_string(),
                    field: "stars".to_string(),
                    selector: ".star-rating".to_string(),
                }
            })?;
            document
                .select(&selector)
                .next()
                .ok_or(DeserializeError::ElementNotFoud {
                    struct_name: std::any::type_name::<Self>().to_string(),
                    field: "stars".to_string(),
                    selector: ".star-rating".to_string(),
                })?
                .value()
                .attr("class")
                .ok_or(DeserializeError::AttributeNotFound {
                    struct_name: std::any::type_name::<Self>().to_string(),
                    field: "stars".to_string(),
                    selector: Some(".star-rating".to_string()),
                    attribute: "class".to_string(),
                })?
                .to_string()
        };
        Ok(BookItem {
            url,
            name,
            price,
            stars,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let html = reqwest::get("https://books.toscrape.com/")
        .await?
        .text()
        .await?;
    let result = BooksPage::from_html(&html)?;
    println!("{result:#?}");
    Ok(())
}
