#![allow(dead_code)]

use de_hypertext_core::Deserializer;
use std::error::Error;

#[allow(unused)]
#[derive(Debug, de_hypertext_macro::Deserialize)]
struct BooksPage {
    #[de_hypertext(selector = "title", trim)]
    title: String,
    #[de_hypertext(selector = ".pager > .current", trim)]
    pages: String,
    #[de_hypertext(selector = ".row > li")]
    items: Vec<BookItem>,
}

#[derive(Debug, de_hypertext_macro::Deserialize)]
struct BookItem {
    #[de_hypertext(selector = "h3 > a", attribute = "href")]
    url: String,
    #[de_hypertext(selector = "h3 > a")]
    name: String,
    #[de_hypertext(selector = ".price_color")]
    price: String,
    #[de_hypertext(selector = ".star-rating", attribute = "class")]
    stars: String,
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
