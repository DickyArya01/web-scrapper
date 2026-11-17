use std::{error::Error, fs, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Item {
    pub title: String,
    pub price: f32,
}


impl Item {
    pub fn create(title: String, raw_price: String) -> Item {
        Item {
            title,
            price: parse_price(&raw_price),
        }
    }
}


#[derive(Deserialize, Serialize)]
pub struct Items {
    pub items: Vec<Item>,
}

impl Items {
    pub fn write_to_json(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        let json_string = serde_json::to_string_pretty(self)?;
        fs::write(path, json_string)?;
        Ok(())
    }
    
    pub(crate) fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn push(&mut self, val: Item) {
        self.items.push(val);
    }
}


fn parse_price(raw: &str) -> f32 {
    let clean_price: String = raw
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '.')
        .collect();

    clean_price.parse::<f32>().unwrap_or(0.0)
}
