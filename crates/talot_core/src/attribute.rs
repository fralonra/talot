use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Attribute {
    pub id: u32,
    pub name: String,
}
