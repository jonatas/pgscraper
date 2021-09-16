use pgx::*;
use scraper::{Html, Selector};
pg_module_magic!();

#[pg_extern]
fn http(url: &str) -> String {
    let response = rttp_client::HttpClient::new()
        .url(url)
        .emit()
        .expect("valid response");
    response.to_string()
}

#[pg_extern]
fn html_select(expression: &str, url: &str) -> String {
    let document = Html::parse_document(&http(url));
    let selector = Selector::parse(expression).unwrap();
    document.select(&selector).next().unwrap().html()
}

#[pg_extern]
fn html_select_text(expression: &str, url: &str) -> String {
    let document = Html::parse_document(&http(&url));
    let selector = Selector::parse(expression).unwrap();
    let mut results = String::from("");
    for element in document.select(&selector) {
        results.push_str(&element.inner_html());
        results.push_str("\n");
    }
    results
}
