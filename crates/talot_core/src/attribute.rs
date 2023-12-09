use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Attribute {
    pub id: u32,
    pub name: String,
    #[serde(default)]
    pub hidden: bool,
}
