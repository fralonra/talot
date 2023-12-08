use serde::Deserialize;

use crate::{QueryInfo, Stat};

use super::op::{InclusionOp, OpImpl, RelationalOp};

#[derive(Clone, Debug, Deserialize)]
pub enum Condition {
    Age(RelationalOp<f32>),
    Attr(InclusionOp<u32>),
    Stat((RelationalOp<f32>, Stat)),
}

impl Condition {
    pub fn is_satisfied(&self, query: &QueryInfo) -> bool {
        match self {
            Self::Age(op) => op.is_matched(&query.age),
            Self::Attr(op) => op.is_matched(&query.attrs),
            Self::Stat((op, stat)) => {
                let stat = match stat {
                    Stat::Int => query.stats.intuition,
                    Stat::Kno => query.stats.knowledge,
                    Stat::Phy => query.stats.physical,
                    Stat::Soc => query.stats.social,
                };

                op.is_matched(&stat)
            }
        }
    }
}
