use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub enum Effect {
    Attr,
    Stat,
}
