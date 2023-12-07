use serde::Deserialize;

use crate::{Timing, TimingImpl};

#[derive(Debug, Deserialize)]
pub struct Category {
    pub id: u32,
    pub name: String,
    #[serde(default)]
    pub timings: Vec<Timing>,
}

impl TimingImpl<u32> for Category {
    fn id(&self) -> u32 {
        self.id
    }

    fn timings(&self) -> &[Timing] {
        &self.timings
    }
}
