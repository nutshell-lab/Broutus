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
            .and_then(|v| v.downcast_ref::<Handle<TextureAtlas>>())
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
            .and_then(|v| v.downcast_ref::<Handle<Image>>())
            .cloned()
    }
}

#[derive(AssetCollection)]
pub struct WarriorCollection {
    #[asset(path = "warriors", folder(typed))]
    warriors: Vec<Handle<WarriorAsset>>,
}

/// Loadable asset struct used to spawn WarriorBundle(s)
#[derive(TypeUuid, Deserialize, Serialize)]
#[uuid = "e51081d0-6168-4881-a1c6-1249b2000e7f"]
pub struct WarriorAsset {
    name: String,
    render: WarriorAssetRender,
    health: Attribute<Health>,
    shield: Attribute<Shield>,
    action_points: Attribute<ActionPoints>,
    movement_points: Attribute<MovementPoints>,
    actions: Vec<Action>,
}

#[derive(Deserialize, Serialize)]
struct WarriorAssetRender {
    atlas_texture: String,
    animations: HashMap<String, (usize, usize)>,
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
            let label = asset.name.clone().to_lowercase();
            let asset = LoadedAsset::new(asset);

            load_context.set_labeled_asset(label.as_str(), asset);

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["warrior"];
        EXTENSIONS
    }
}
