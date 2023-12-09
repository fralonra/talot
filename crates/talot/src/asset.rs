use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use serde::Deserialize;
use talot_core::Engine;

#[derive(Asset, Debug, Deserialize, Deref, DerefMut, TypePath)]
pub struct GameAsset(pub Engine);

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audios/lol.ogg")]
    pub lol: Handle<AudioSource>,
    #[asset(path = "audios/tot.ogg")]
    pub tot: Handle<AudioSource>,
    #[asset(path = "audios/lot.ogg")]
    pub lot: Handle<AudioSource>,
    #[asset(path = "audios/miss.ogg")]
    pub miss: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct GameDataAssets {
    #[asset(path = "core.asset.json")]
    pub core: Handle<GameAsset>,
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "images/empty_ef.png")]
    pub empty_ef: Handle<Image>,
    #[asset(path = "images/lol.png")]
    pub lol: Handle<Image>,
    #[asset(path = "images/tot.png")]
    pub tot: Handle<Image>,
}
