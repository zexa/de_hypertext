use de_hypertext::Deserializer;
use std::error::Error;

#[allow(unused)]
#[derive(Debug, de_hypertext_macro::Deserialize)]
struct BooksPage {
    #[de_hypertext(selector = "title", transform = |x: String| x.trim().to_string())]
    title: String,
    #[de_hypertext(selector = ".pager > .current", transform = |x: String| x.trim().to_string())]
    pages: String,
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
