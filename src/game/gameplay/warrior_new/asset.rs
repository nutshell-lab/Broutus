use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::utils::HashMap;
use bevy_asset_loader::AssetCollection;
use serde::{Deserialize, Serialize};

use super::{Action, ActionPoints, Attribute, Health, MovementPoints, Shield};

// TODO find a way to load a folder into as HashMap<String, Handle<..>>
#[derive(AssetCollection, Reflect)]
pub struct AnimationCollection {
    #[asset(key = "animations.freddy")]
    pub freddy: Handle<TextureAtlas>,
}

impl AnimationCollection {
    /// Get an image handle giving an icon key
    pub fn get(&self, key: &str) -> Option<Handle<TextureAtlas>> {
        self.field(key)
            .and_then(|field| field.downcast_ref::<Handle<TextureAtlas>>())
            .cloned()
    }
}

// TODO find a way to load a folder into as HashMap<String, Handle<..>>
#[derive(AssetCollection, Reflect)]
pub struct IconCollection {
    #[asset(key = "icons.action_blind")]
    pub action_blind: Handle<Image>,

    #[asset(key = "icons.action_cripple")]
    pub action_cripple: Handle<Image>,

    #[asset(key = "icons.action_heal")]
    pub action_heal: Handle<Image>,

    #[asset(key = "icons.action_push")]
    pub action_push: Handle<Image>,

    #[asset(key = "icons.action_shield")]
    pub action_shield: Handle<Image>,

    #[asset(key = "icons.action_shoot")]
    pub action_shoot: Handle<Image>,

    #[asset(key = "icons.action_slash")]
    pub action_slash: Handle<Image>,

    #[asset(key = "icons.action_teleport")]
    pub action_teleport: Handle<Image>,
}

impl IconCollection {
    /// Get an image handle giving an icon key
    pub fn get(&self, key: &str) -> Option<Handle<Image>> {
        self.field(key)
            .and_then(|field| field.downcast_ref::<Handle<Image>>())
            .cloned()
    }

    pub fn get_index(&self, key: &str) -> Option<usize> {
        if let Some(right) = self.get(key) {
            self.get_all().iter().position(|left| left.eq(&right))
        } else {
            None
        }
    }

    pub fn get_all(&self) -> Vec<Handle<Image>> {
        self.iter_fields()
            .map(|field| field.downcast_ref::<Handle<Image>>())
            .filter(|res| res.is_some())
            .map(|res| res.unwrap())
            .cloned()
            .collect()
    }
}

#[derive(AssetCollection)]
pub struct WarriorCollection {
    #[asset(path = "warriors", folder(typed))]
    pub warriors: Vec<Handle<WarriorAsset>>,
}

/// Loadable asset struct used to spawn WarriorBundle(s)
#[derive(TypeUuid, Debug, Deserialize, Serialize)]
#[uuid = "e51081d0-6168-4881-a1c6-1249b2000e7f"]
pub struct WarriorAsset {
    pub name: String,
    pub render: WarriorAssetRender,
    pub health: Attribute<Health>,
    pub shield: Attribute<Shield>,
    pub action_points: Attribute<ActionPoints>,
    pub movement_points: Attribute<MovementPoints>,
    pub actions: Vec<Action>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WarriorAssetRender {
    pub atlas_texture: String,
    pub animations: HashMap<String, (usize, usize)>,
}

pub struct WarriorAssetLoader;

impl AssetLoader for WarriorAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let asset = ron::de::from_bytes::<WarriorAsset>(bytes)?;
            let asset = LoadedAsset::new(asset);

            load_context.set_default_asset(asset);

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["warrior"];
        EXTENSIONS
    }
}
