# de_hypertext
Framework for parsing html into Rust structures.

> [!CAUTION]
> The library is currently an experiment for my personal use. It's not at all ready for production.

## Usage
```bash
cargo add de_hypertext
```

## Example
```rust
use de_hypertext::Deserializer;
use std::error::Error;

#[derive(Debug, de_hypertext_macro::Deserialize)]
struct BooksPage {
    #[de_hypertext(selector = "title", transform = |x: String| x.trim().to_string())]
    title: String,
    #[de_hypertext(selector = ".pager > .current", transform = |x: String| x.trim().to_string())]
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
```
