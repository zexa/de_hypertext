#![allow(dead_code)]

use de_hypertext_core::DeserializeError;
use de_hypertext_core::Deserializer;
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
    fn from_document(document: &scraper::ElementRef) -> Result<Self, Box<dyn Error>> {
        let title = {
            let selector = scraper::Selector::parse("title")?;
            document
                .select(&selector)
                .next()
                .ok_or(DeserializeError::ElementNotFoud {
                    struct_name: "BookPage".to_string(),
                    field: "title".to_string(),
                    selector: "title".to_string(),
                })?
                .text()
                .collect::<String>()
                .trim()
                .to_string()
        };
        let items = BookItem::from_document(document)?;
        Ok(BooksPage { title, items })
    }
}

impl Deserializer<Vec<Self>> for BookItem {
    fn from_document(document: &scraper::ElementRef) -> Result<Vec<BookItem>, Box<dyn Error>> {
        let selector = scraper::Selector::parse(".row > li")?;
        document
            .select(&selector)
            .into_iter()
            .map(|document| BookItem::from_document(&document))
            .collect()
    }
}

impl Deserializer<Self> for BookItem {
    fn from_document(document: &scraper::ElementRef) -> Result<BookItem, Box<dyn Error>> {
        let url = {
            let selector = scraper::Selector::parse("h3 > a")?;
            document
                .select(&selector)
                .next()
                .ok_or(DeserializeError::ElementNotFoud {
                    struct_name: "BookItem".to_string(),
                    field: "name".to_string(),
                    selector: "h3 > a".to_string(),
                })?
                .value()
                .attr("href")
                .ok_or(DeserializeError::AttributeNotFound {
                    struct_name: "BookItem".to_string(),
                    field: "url".to_string(),
                    selector: "h3 > a".to_string(),
                    attribute: "href".to_string(),
                })?
                .to_string()
        };
        let name = {
            let selector = scraper::Selector::parse("h3 > a")?;
            document
                .select(&selector)
                .next()
                .ok_or(DeserializeError::ElementNotFoud {
                    struct_name: "BookItem".to_string(),
                    field: "name".to_string(),
                    selector: "h3 > a".to_string(),
                })?
                .text()
                .collect()
        };
        let price = {
            let selector = scraper::Selector::parse(".price_color")?;
            document
                .select(&selector)
                .next()
                .ok_or(DeserializeError::ElementNotFoud {
                    struct_name: "BookItem".to_string(),
                    field: "price".to_string(),
                    selector: ".price_color".to_string(),
                })?
                .text()
                .collect()
        };
        let stars = {
            let selector = scraper::Selector::parse(".star-rating")?;
            document
                .select(&selector)
                .next()
                .ok_or(DeserializeError::ElementNotFoud {
                    struct_name: "BookItem".to_string(),
                    field: "stars".to_string(),
                    selector: ".star-rating".to_string(),
                })?
                .value()
                .attr("class")
                .ok_or(DeserializeError::AttributeNotFound {
                    struct_name: "BookItem".to_string(),
                    field: "stars".to_string(),
                    selector: ".star-rating".to_string(),
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
