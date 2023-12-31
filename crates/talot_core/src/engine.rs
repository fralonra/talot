use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, RwLock},
};

use rand_distr::{Distribution, WeightedIndex};
use serde::{de, Deserialize, Deserializer};

use crate::{Attribute, Category, Lot, Stats, Timing, TimingImpl, ER};

pub struct TimingWrapper<'a, T: Copy> {
    pub id: T,
    pub timings: &'a [Timing],
}

impl<'a, T: Copy> TimingImpl<T> for TimingWrapper<'a, T> {
    fn id(&self) -> T {
        self.id
    }

    fn timings(&self) -> &[Timing] {
        self.timings
    }
}

pub struct QueryInfo<'a> {
    pub age: f32,
    pub attrs: &'a HashSet<u32>,
    pub er: &'a ER,
    pub stats: &'a Stats,
}

pub struct RespInfo {
    pub attrs: Option<HashSet<u32>>,
    pub er: Option<ER>,
    pub stats: Option<Stats>,
}

#[derive(Debug, Deserialize)]
pub struct Engine {
    #[serde(deserialize_with = "deserialize_attrs")]
    pub attributes: HashMap<u32, Attribute>,
    pub categories: Vec<Category>,

    #[serde(skip)]
    pub executed_set: Arc<RwLock<HashSet<u32>>>,
}

impl Engine {
    pub fn apply_lot(&self, lot: &Lot, query: &QueryInfo) -> RespInfo {
        lot.apply(query)
    }

    pub fn get_attr(&self, id: u32) -> Option<&Attribute> {
        self.attributes.get(&id)
    }

    pub fn query_category(&self, query: &QueryInfo) -> Option<&Category> {
        self.query_random_item(
            &self.categories,
            query,
            |_| true,
            |(idx, category)| TimingWrapper {
                id: idx,
                timings: &category.timings,
            },
        )
    }

    pub fn query_category_and_lot(&self, query: &QueryInfo) -> Option<(&Category, &Lot)> {
        let category = self.query_category(query);

        let lot = category.and_then(|id| self.query_lot_of_category(id, query));

        if category.is_none() || lot.is_none() {
            return None;
        }

        Some((category.unwrap(), lot.unwrap()))
    }

    pub fn query_lot(&self, query: &QueryInfo) -> Option<&Lot> {
        let category = self.query_category(query);

        category.and_then(|id| self.query_lot_of_category(id, query))
    }

    pub fn reset(&self) {
        self.executed_set.write().unwrap().clear();
    }

    fn query_candidates_id_and_weight<T>(
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

    fn query_lot_of_category<'a>(
        &'a self,
        category: &'a Category,
        query: &QueryInfo,
    ) -> Option<&'a Lot> {
        let lot = self.query_random_item(
            &category.lots,
            query,
            |(_, lot)| !lot.one_time || !self.executed_set.read().unwrap().contains(&lot.id),
            |(idx, lot)| TimingWrapper {
                id: idx,
                timings: &lot.timings,
            },
        );

        if let Some(lot) = lot {
            self.executed_set.write().unwrap().insert(lot.id);
        }

        lot
    }

    fn query_random_item<'a, T, M, P>(
        &self,
        list: &'a [T],
        query: &QueryInfo,
        predicate: P,
        wrapper_mapper: M,
    ) -> Option<&'a T>
    where
        M: FnMut((usize, &T)) -> TimingWrapper<usize>,
        P: FnMut(&(usize, &T)) -> bool,
    {
        let wrapper_list = list
            .iter()
            .enumerate()
            .filter(predicate)
            .map(wrapper_mapper)
            .collect::<Vec<TimingWrapper<usize>>>();

        let candidates = self.query_candidates_id_and_weight(&wrapper_list, query);

        let index = get_weighted_random(candidates);

        index.and_then(|index| list.iter().nth(index))
    }
}

fn deserialize_attrs<'de, D>(deserializer: D) -> Result<HashMap<u32, Attribute>, D::Error>
where
    D: Deserializer<'de>,
{
    let original = <Vec<Attribute>>::deserialize(deserializer)?;

    let original_len = original.len();

    let data = original
        .into_iter()
        .map(|attr| (attr.id, attr))
        .collect::<HashMap<_, _>>();

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
