use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct SheetModel {
    pub range: String,
    pub values: Vec<HashMap<String, String>>,
}
