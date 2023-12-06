use bevy::{prelude::*, utils::HashMap};
use rand::seq::IteratorRandom;
use serde::{de, Deserialize, Deserializer};

use super::{Category, Lot};

#[derive(Debug, Deserialize, Asset, TypePath)]
pub struct GameAsset {
    pub categories: Vec<Category>,
    #[serde(deserialize_with = "deserialize_lots_map")]
    pub lots: HashMap<u32, Vec<Lot>>,
}

impl GameAsset {
    pub fn get_lot(&self) -> Lot {
        let category = self
            .categories
            .iter()
            .choose(&mut rand::thread_rng())
            .unwrap();

        self.get_lot_of_category(category.id)
    }

    pub fn get_lot_of_category(&self, category_id: u32) -> Lot {
        let lot_list = self
            .lots
            .get(&category_id)
            .expect(&format!("Couldn't find lots for category {}", category_id));

        lot_list
            .iter()
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone()
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
