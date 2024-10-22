use de_hypertext::Deserializer;
use std::error::Error;

#[allow(unused)]
#[derive(Debug, serde::Serialize, de_hypertext_macro::Deserialize)]
struct BooksPage {
    #[de_hypertext(selector = "title", transform = |x: String| x.trim().to_string())]
    title: String,
    #[de_hypertext(selector = ".pager > .current", transform = |x: String| x.trim().to_string())]
    pages: String,
    #[de_hypertext(selector = ".row > li")]
    items: Vec<BookItem>,
}

#[derive(Debug, serde::Serialize, de_hypertext_macro::Deserialize)]
struct BookItem {
    #[de_hypertext(selector = "h3 > a", attribute = "href")]
    url: String,
    #[de_hypertext(selector = "h3 > a")]
    name: String,
    #[de_hypertext(selector = ".price_color")]
    price: String,
    #[de_hypertext(selector = ".star-rating", attribute = "class", transform = |x: String| x.trim().to_string())]
    stars: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let html = reqwest::get("https://books.toscrape.com/")
        .await?
        .text()
        .await?;
    let page = BooksPage::from_html(&html)?;
    let serialized = serde_json::to_string_pretty(&page)?;
    println!("{serialized}");
    Ok(())
}
