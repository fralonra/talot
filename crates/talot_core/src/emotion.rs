use serde::Deserialize;

// Emotional Factor
#[derive(Clone, Copy, Debug, Deserialize)]
pub enum EF {
    Lol,
    Tot,
}

// Emotional Rating
#[derive(Clone, Debug, Default)]
pub struct ER {
    pub lol: f32,
    pub tot: f32,
}
