use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Category {
    pub id: u32,
    pub name: String,
}
