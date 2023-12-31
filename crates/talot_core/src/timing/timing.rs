use serde::Deserialize;

use crate::Condition;

pub trait TimingImpl<T> {
    fn id(&self) -> T;

    fn timings(&self) -> &[Timing];
}

#[derive(Clone, Debug, Deserialize)]
pub struct Timing {
    #[serde(default)]
    pub p: f32,
    // Timing condition. Needs to meet them all.
    #[serde(default)]
    pub conditions: Vec<Condition>,
}

impl Default for Timing {
    fn default() -> Self {
        Self {
            p: 0.5,
            conditions: vec![],
        }
    }
}

impl Timing {
    pub fn default_timings() -> Vec<Self> {
        vec![Self::default()]
    }
}
