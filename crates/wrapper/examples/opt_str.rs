#[allow(dead_code)]
#[derive(de_hypertext::Deserialize)]
struct HasOption {
    #[de_hypertext(selector = "h3")]
    s: Option<String>,
    #[de_hypertext(selector = "a", attribute = "href")]
    v: Option<String>,
}

fn main() {
    println!("Hello world");
}
