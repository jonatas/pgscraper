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
fn html_select(expression: &str, url: &str) -> impl std::iter::Iterator<Item = String> {
    let document = Html::parse_document(&http(url));
    let selector = Selector::parse(expression).unwrap();
    document
        .select(&selector)
        .into_iter()
        .map(|i| i.html())
        .into_iter()
        .collect::<Vec<String>>()
        .into_iter()
}

#[pg_extern]
fn html_select_text(expression: &str, url: &str) -> impl std::iter::Iterator<Item = String> {
    let document = Html::parse_document(&http(url));
    let selector = Selector::parse(expression).unwrap();
    document
        .select(&selector)
        .into_iter()
        .map(|i| i.text().collect::<String>())
        .into_iter()
        .collect::<Vec<String>>()
        .into_iter()
}
