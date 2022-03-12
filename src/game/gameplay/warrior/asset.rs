use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::utils::HashMap;
use bevy_asset_loader::AssetCollection;
use serde::{Deserialize, Serialize};

use super::{ActionPoints, Actions, Attribute, Health, MovementPoints, Shield};

// TODO find a way to load a folder into as HashMap<String, Handle<..>>
#[derive(AssetCollection, Reflect)]
pub struct AnimationCollection {
    #[asset(key = "animations.ella")]
    pub ella: Handle<TextureAtlas>,
}

impl AnimationCollection {
    /// Get an image handle giving an icon key
    pub fn get(&self, key: &str) -> Option<Handle<TextureAtlas>> {
        self.field(key)
            .and_then(|field| field.downcast_ref::<Handle<TextureAtlas>>())
            .cloned()
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
    pub portrait_key: String,
    pub render: WarriorAssetRender,
    pub health: Attribute<Health>,
    pub shield: Attribute<Shield>,
    pub action_points: Attribute<ActionPoints>,
    pub movement_points: Attribute<MovementPoints>,
    pub actions: Actions,
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
        static EXTENSIONS: &[&str] = &["ron"];
        EXTENSIONS
    }
}
