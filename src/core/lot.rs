use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Lot {
    pub desc: String,
    pub p: f32,
}
