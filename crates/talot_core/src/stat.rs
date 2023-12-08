use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum Stat {
    // Intuition
    Int,
    // Knowledge
    Kno,
    // Physical
    Phy,
    // Social
    Soc,
}

#[derive(Clone, Debug, Default)]
pub struct Stats {
    pub intuition: f32,
    pub knowledge: f32,
    pub physical: f32,
    pub social: f32,
}
