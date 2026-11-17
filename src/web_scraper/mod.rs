use std::error::Error;

use scraper::{Html, Selector};

use crate::item::Item;

pub async fn fetch(url: &str) -> Result<String, reqwest::Error> {
    reqwest::get(url).await?.error_for_status()?.text().await
}

pub fn get_paged_url(base: &str, page: u32, query: Option<&str>) -> String {
    let mut url: String = base.to_string();

    if page > 1 {
        url = format!("{}/page/{}", url, page);
    }

    if let Some(val) = query {
        url = format!("{}?{}", url, val);
    }

    url
}

pub fn title_and_price_scrapper(html: &str) -> Result<Vec<Item>, Box<dyn Error>> {
    let document = Html::parse_document(html);

    let product_li = Selector::parse("li.product")?;
    let title_selector = Selector::parse("h2.product-name")?;
    let price_selector = Selector::parse("span.price")?;

    let data: Vec<Item> = document
        .select(&product_li)
        .filter_map(|e| {
            let title = e.select(&title_selector).next()?.text().collect::<String>();
            let raw_price = e.select(&price_selector).next()?.text().collect::<String>();

            Some(Item::create(title, raw_price))
        })
        .collect();

    Ok(data)
}
