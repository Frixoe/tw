use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ColumnValue {
    pub text: Option<String>,

    #[serde(alias = "type")]
    pub c_type: Option<String>,
}
