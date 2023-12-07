use serde::Deserialize;

use crate::{Effect, Timing, TimingImpl};

#[derive(Clone, Debug, Deserialize)]
pub struct Lot {
    pub desc: String,
    pub p: f32,
    pub effect: Effect,
    #[serde(default)]
    pub timings: Vec<Timing>,
}

pub struct LotWrapper<'a, T: Copy> {
    pub id: T,
    pub timings: &'a [Timing],
}

impl<'a, T: Copy> TimingImpl<T> for LotWrapper<'a, T> {
    fn id(&self) -> T {
        self.id
    }

    fn timings(&self) -> &[Timing] {
        self.timings
    }
}
