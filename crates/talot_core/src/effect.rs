use rand_distr::{Bernoulli, Distribution};
use serde::Deserialize;

use crate::{QueryInfo, RespInfo, Stat, EF};

pub enum EffectResp {
    None,
    AttrAdd(u32),
    AttrRemove(u32),
    ER { ef: EF, v: f32 },
    Stat { stat: Stat, v: f32 },
}

#[derive(Clone, Debug, Deserialize)]
pub enum Effect {
    AttrAdd { p: f32, attr_id: u32 },
    AttrRemove { p: f32, attr_id: u32 },
    ER { p: f32, ef: EF, v: f32 },
    Stat { p: f32, stat: Stat, v: f32 },
}

impl Effect {
    pub fn take(&self) -> EffectResp {
        let b = Bernoulli::new(self.p() as f64).unwrap_or(Bernoulli::new(0.5).unwrap());

        if !b.sample(&mut rand::thread_rng()) {
            return EffectResp::None;
        }

        match self {
            Self::AttrAdd { attr_id, .. } => EffectResp::AttrAdd(*attr_id),
            Self::AttrRemove { attr_id, .. } => EffectResp::AttrRemove(*attr_id),
            Self::ER { ef, v, .. } => EffectResp::ER { ef: *ef, v: *v },
            Self::Stat { stat, v, .. } => EffectResp::Stat { stat: *stat, v: *v },
        }
    }

    fn p(&self) -> f32 {
        match self {
            Self::AttrAdd { p, .. } => *p,
            Self::AttrRemove { p, .. } => *p,
            Self::ER { p, .. } => *p,
            Self::Stat { p, .. } => *p,
        }
    }
}
