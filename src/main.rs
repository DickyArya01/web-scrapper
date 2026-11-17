mod helper;
mod item;
mod web_scraper;

use std::error::Error;

use crate::helper::get_project_path;
use crate::{
    item::Items,
    web_scraper::{fetch, get_paged_url, title_and_price_scrapper},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut items: Items = Items::new();

    let base_url = "https://www.scrapingcourse.com/ecommerce";

    for i in 1..12 {
        let url = get_paged_url(base_url, i, Option::Some("orderby=price-desc"));

        let html = fetch(&url).await?;

        for i in title_and_price_scrapper(&html)? {
            items.push(i);
        }
    }

    let project_path =  get_project_path().ok_or("Cant get project path")?.join("webscraper.json");

    items.write_to_json(&project_path)?;

    Ok(())
}
