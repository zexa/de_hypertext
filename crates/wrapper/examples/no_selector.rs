use de_hypertext::Deserializer;

#[derive(de_hypertext::Deserialize)]
struct Parent {
    #[de_hypertext(selector = "a")]
    pub child: NoSelectorButWithAttribute,
}

#[derive(de_hypertext::Deserialize)]
struct NoSelectorButWithAttribute {
    #[de_hypertext(attribute = "href")]
    pub field1: String,
}

fn main() {
    let html = "<html><body><a href=\"#\">hello world</a></body></html>";
    let p = Parent::from_html(html).unwrap();
    assert_eq!(p.child.field1, "#");
}
