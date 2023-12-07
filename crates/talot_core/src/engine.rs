use std::collections::HashMap;

use rand_distr::{Distribution, WeightedIndex};
use serde::{de, Deserialize, Deserializer};

use crate::{lot::LotWrapper, Category, Lot, Stats, TimingImpl};

pub struct QueryInfo<'a> {
    pub age: f32,
    pub attrs: &'a [u32],
    pub stats: &'a Stats,
}

#[derive(Debug, Deserialize)]
pub struct Engine {
    pub categories: Vec<Category>,
    #[serde(deserialize_with = "deserialize_lots_map")]
    pub lots: HashMap<u32, Vec<Lot>>,
}

impl Engine {
    pub fn get_lot(&self, query: &QueryInfo) -> Option<Lot> {
        let category = self.get_category(query);

        category.and_then(|id| self.get_lot_of_category(id, query))
    }

    fn get_category(&self, query: &QueryInfo) -> Option<u32> {
        let candidates = self.get_candidates_id_and_weight(&self.categories, query);

        get_weighted_random(candidates)
    }

    fn get_candidates_id_and_weight<T>(
        &self,
        with_timings: &[impl TimingImpl<T>],
        query: &QueryInfo,
    ) -> Vec<(T, f32)> {
        let mut res = vec![];

        for obj in with_timings {
            let mut max_p = 0.0;

            for timing in obj.timings() {
                if timing
                    .conditions
                    .iter()
                    .all(|condition| condition.is_satisfied(&query))
                {
                    if max_p < timing.p {
                        max_p = timing.p;
                    }
                }
            }

            if max_p > 0.0 {
                res.push((obj.id(), max_p));
            }
        }

        res
    }

    fn get_lot_of_category(&self, category_id: u32, query: &QueryInfo) -> Option<Lot> {
        let lot_list = self
            .lots
            .get(&category_id)
            .expect(&format!("Couldn't find lots for category {}", category_id));

        let lot_wrapper_list = lot_list
            .iter()
            .enumerate()
            .map(|(id, lot)| LotWrapper {
                id,
                timings: &lot.timings,
            })
            .collect::<Vec<LotWrapper<usize>>>();

        let candidates = self.get_candidates_id_and_weight(&lot_wrapper_list, query);

        let index = get_weighted_random(candidates);

        index.and_then(|index| lot_list.iter().nth(index)).cloned()
    }
}

fn deserialize_lots_map<'de, D>(deserializer: D) -> Result<HashMap<u32, Vec<Lot>>, D::Error>
where
    D: Deserializer<'de>,
{
    let original_map = HashMap::<String, Vec<Lot>>::deserialize(deserializer)?;

    let original_len = original_map.len();

    let data = original_map
        .into_iter()
        .map(|(original_key, value)| match original_key.parse() {
            Ok(key) => Ok((key, value)),
            Err(_) => Err({
                de::Error::invalid_value(
                    de::Unexpected::Str(&original_key),
                    &"a non-negative integer",
                )
            }),
        })
        .collect::<Result<HashMap<_, _>, _>>()?;

    if data.len() < original_len {
        return Err(de::Error::custom("detected duplicate integer key"));
    }

    Ok(data)
}

fn get_weighted_random<T: Copy>(items: Vec<(T, f32)>) -> Option<T> {
    if items.is_empty() {
        return None;
    }

    let (values, weights): (Vec<T>, Vec<f32>) = items.into_iter().unzip();

    let dist = WeightedIndex::new(&weights).unwrap();

    let mut rng = rand::thread_rng();

    Some(values[dist.sample(&mut rng)])
}
