use serde::Deserialize;

use crate::{effect::EffectResp, Effect, QueryInfo, RespInfo, Stat, Timing, EF};

#[derive(Clone, Debug, Deserialize)]
pub struct Lot {
    pub desc: String,
    pub p: f32,
    #[serde(default)]
    pub effects: Vec<Effect>,
    #[serde(default)]
    pub timings: Vec<Timing>,
}

impl Lot {
    pub fn apply(&self, query: &QueryInfo) -> RespInfo {
        let mut attrs = None;
        let mut er = None;
        let mut stats = None;

        for effect in &self.effects {
            let resp = effect.take();

            match resp {
                EffectResp::AttrAdd(id) => {
                    if attrs.is_none() {
                        attrs = Some(query.attrs.to_owned());
                    }

                    if let Some(ref mut attrs) = &mut attrs {
                        attrs.insert(id);
                    }
                }
                EffectResp::AttrRemove(id) => {
                    if attrs.is_none() {
                        attrs = Some(query.attrs.to_owned());
                    }

                    if let Some(ref mut attrs) = &mut attrs {
                        attrs.remove(&id);
                    }
                }
                EffectResp::ER { ef, v } => {
                    if er.is_none() {
                        er = Some(query.er.clone());
                    }

                    if let Some(ref mut er) = &mut er {
                        match ef {
                            EF::Lol => er.lol += v,
                            EF::Tot => er.tot += v,
                        }
                    }
                }
                EffectResp::Stat { stat, v } => {
                    if stats.is_none() {
                        stats = Some(query.stats.clone());
                    }

                    if let Some(ref mut stats) = &mut stats {
                        match stat {
                            Stat::Int => stats.intuition += v,
                            Stat::Kno => stats.knowledge += v,
                            Stat::Phy => stats.physical += v,
                            Stat::Soc => stats.social += v,
                        }
                    }
                }
                _ => {}
            }
        }

        RespInfo { attrs, er, stats }
    }
}
