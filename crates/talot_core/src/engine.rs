use std::collections::HashSet;

use rand_distr::{Distribution, WeightedIndex};
use serde::Deserialize;

use crate::{Category, Lot, Stat, Stats, Timing, TimingImpl, ER};

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
    pub categories: Vec<Category>,
}

impl Engine {
    pub fn apply_lot(&self, lot: &Lot, query: &QueryInfo) -> RespInfo {
        lot.apply(query)
    }

    pub fn get_lot(&self, query: &QueryInfo) -> Option<Lot> {
        let category = self.get_category(query);

        category.and_then(|id| self.get_lot_of_category(id, query))
    }

    fn get_category(&self, query: &QueryInfo) -> Option<&Category> {
        self.get_random_item(&self.categories, query, |(id, category)| TimingWrapper {
            id,
            timings: &category.timings,
        })
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

    fn get_lot_of_category(&self, category: &Category, query: &QueryInfo) -> Option<Lot> {
        self.get_random_item(&category.lots, query, |(id, lot)| TimingWrapper {
            id,
            timings: &lot.timings,
        })
        .cloned()
    }

    fn get_random_item<'a, T, F>(
        &self,
        list: &'a [T],
        query: &QueryInfo,
        wrapper_mapper: F,
    ) -> Option<&'a T>
    where
        F: FnMut((usize, &T)) -> TimingWrapper<usize>,
    {
        let wrapper_list = list
            .iter()
            .enumerate()
            .map(wrapper_mapper)
            .collect::<Vec<TimingWrapper<usize>>>();

        let candidates = self.get_candidates_id_and_weight(&wrapper_list, query);

        let index = get_weighted_random(candidates);

        index.and_then(|index| list.iter().nth(index))
    }
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
