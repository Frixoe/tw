use crate::types::ColumnValue;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub name: String,
    pub column_values: Vec<ColumnValue>,
}

impl Item {
    pub fn items_from_response(res: serde_json::Value) -> anyhow::Result<Vec<Item>> {
        let Some(response) = res.get("data") else {
            return Err(anyhow::anyhow!("Failed to get data"));
        };

        let Some(boards) = response.get("boards") else {
            return Err(anyhow::anyhow!("Failed to get boards"));
        };

        let Some(items_page) = boards[0].get("items_page") else {
            return Err(anyhow::anyhow!("Failed to get items_page"));
        };

        let Some(items) = items_page.get("items") else {
            return Err(anyhow::anyhow!("Failed to get items"));
        };

        Ok(serde_json::from_value::<Vec<Item>>(items.clone())?)
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "- {}", self.name)
    }
}
