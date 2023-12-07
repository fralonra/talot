use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub enum Stat {
    Int,
    Kno,
    Phy,
    Soc,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct Stats {
    pub intuition: f32,
    pub knowledge: f32,
    pub physical: f32,
    pub social: f32,
}
