use serde::Deserialize;

use crate::{Lot, Timing};

#[derive(Clone, Debug, Deserialize)]
pub struct Category {
    pub id: u32,
    pub name: String,
    #[serde(default)]
    pub timings: Vec<Timing>,
    #[serde(default)]
    pub lots: Vec<Lot>,
}
