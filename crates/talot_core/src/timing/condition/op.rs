use std::{collections::HashSet, hash::Hash};

use serde::Deserialize;

pub trait OpImpl {
    type Value;

    fn is_matched(&self, value: &Self::Value) -> bool;
}

#[derive(Clone, Debug, Deserialize)]
pub enum InclusionOp<T: Eq + Hash> {
    With(T),
    Without(T),
}

impl<T: Eq + Hash> OpImpl for InclusionOp<T> {
    type Value = HashSet<T>;

    fn is_matched(&self, value: &Self::Value) -> bool {
        match self {
            Self::With(v) => value.contains(v),
            Self::Without(v) => !value.contains(v),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub enum RelationalOp<T: PartialEq + PartialOrd> {
    Eq(T),
    NotEq(T),
    Gt(T),
    GtEq(T),
    Lt(T),
    LtEq(T),
}

impl<T: PartialEq + PartialOrd> OpImpl for RelationalOp<T> {
    type Value = T;

    fn is_matched(&self, value: &Self::Value) -> bool {
        match self {
            Self::Eq(v) => value == v,
            Self::NotEq(v) => value != v,
            Self::Gt(v) => value > v,
            Self::GtEq(v) => value >= v,
            Self::Lt(v) => value < v,
            Self::LtEq(v) => value <= v,
        }
    }
}
