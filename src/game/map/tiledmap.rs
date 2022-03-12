use bevy::{
    asset::{AssetLoader, AssetPath, BoxedFuture, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::HashMap,
};
use bevy_asset_loader::AssetCollection;
use std::{io::BufReader, path::Path};

use super::MapPosition;

#[derive(AssetCollection)]
pub struct MapsAssets {
    // #[asset(key = "maps.simple")]
    // simple: Handle<Tiledmap>,
    #[asset(key = "maps.test")]
    test: Handle<Tiledmap>,
}

#[derive(TypeUuid)]
#[uuid = "e51081d0-6168-4881-a1c6-1249b2000d7f"]
pub struct Tiledmap {
    pub id: u32,
    pub inner: tiled::Map,
    pub tileset: Handle<Image>,
    pub tileset_width: u32,
    pub tileset_height: u32,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Map {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub tile_width: u32,
    pub tile_height: u32,
    pub tiles: HashMap<(u32, u32, u32), Entity>,
}

impl Map {
    pub fn all_positions(&self) -> Vec<MapPosition> {
        let mut positions = Vec::new();
        for x in 0..(self.width - 1) {
            for y in 0..(self.height - 1) {
                positions.push(MapPosition::new(x, y));
            }
        }
        positions
    }
}

#[derive(Default, Bundle)]
pub struct MapBundle {
    pub map: Map,
    pub tiledmap: Handle<Tiledmap>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
}

#[derive(Reflect, Component, Default, Clone, Copy)]
#[reflect(Component)]
pub struct LayerIndex(pub u32);

#[derive(Reflect, Component, Default, Clone, Copy)]
#[reflect(Component)]
pub struct Tile;

#[derive(Default, Bundle)]
pub struct TileBundle {
    pub tile: Tile,
    pub layer: LayerIndex,
    pub position: MapPosition,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
}

pub struct TiledmapLoader;

impl AssetLoader for TiledmapLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            // TODO find a nicer way to retreive the asset full_path from load_context
            let current_dir_path = std::env::current_dir().unwrap();
            let cargo_debug_dir = std::env::var("CARGO_MANIFEST_DIR");
            let cargo_debug_dir_path = cargo_debug_dir
                .as_ref()
                .map(|v| Path::new(v).join("assets").join(load_context.path()));
            let path = cargo_debug_dir_path.unwrap_or(current_dir_path);

            // Parse the map providing the asset path to support external tilesets
            let root_dir = load_context.path().parent().unwrap();
            let map = tiled::parse_with_path(BufReader::new(bytes), path.as_path())?;

            let mut dependencies = Vec::new();

            let tileset = &map.tilesets.first().expect("Missing tileset");
            let image = tileset.images.first().unwrap();
            let image_width = image.width;
            let image_height = image.height;
            let tile_path = root_dir.join(image.source.as_str());
            let asset_path = AssetPath::new(tile_path, None);
            let texture: Handle<Image> = load_context.get_handle(asset_path.clone());

            dependencies.push(asset_path);

            let loaded_asset = LoadedAsset::new(Tiledmap {
                id: 0,
                inner: map,
                tileset: texture,
                tileset_width: image_width as u32,
                tileset_height: image_height as u32,
            });
            load_context.set_default_asset(loaded_asset.with_dependencies(dependencies));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["tmx"];
        EXTENSIONS
    }
}

pub fn spawn_tiledmap(
    mut commands: Commands,
    maps_assets: Res<MapsAssets>,
    tiledmaps: Res<Assets<Tiledmap>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let map_entity = commands.spawn().id();

    if let Some(tiledmap) = tiledmaps.get(maps_assets.test.clone()) {
        let mut tiles_entities = HashMap::default();

        let tileset = tiledmap
            .inner
            .tilesets
            .first()
            .expect("Tiledmap needs a tileset");
        let texture_atlas = TextureAtlas::from_grid(
            tiledmap.tileset.clone(),
            Vec2::new(tileset.tile_width as f32, tileset.tile_height as f32),
            (tiledmap.tileset_width / tileset.tile_width) as usize,
            (tiledmap.tileset_height / tileset.tile_height) as usize,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        for (layer_index, layer) in tiledmap.inner.layers.iter().enumerate() {
            let layer_index = layer_index as u32;

            if let tiled::LayerData::Finite(tiles_y) = &layer.tiles {
                for (tile_y, tiles_x) in tiles_y.iter().enumerate() {
                    for (tile_x, tile) in tiles_x.iter().enumerate() {
                        if tile.gid == 0 {
                            continue;
                        }
                        let (x, y) = (tile_x as u32, tile_y as u32);
                        let tile_entity = commands
                            .spawn()
                            .insert(Name::new(format!(
                                "{} - tile ({:02},{:02})",
                                layer_index, x, y
                            )))
                            .id();

                        let mut sprite = TextureAtlasSprite::new(tile.gid as usize - 1);
                        // Set obstacle a little transparent
                        sprite.color = if layer_index == 2 {
                            Color::rgba(1., 1., 1., 0.95)
                        } else {
                            Color::rgba(1., 1., 1., 1.)
                        };

                        tiles_entities.insert((layer_index, x, y), tile_entity);
                        commands.entity(map_entity).add_child(tile_entity);
                        commands
                            .entity(tile_entity)
                            .insert_bundle(TileBundle {
                                position: MapPosition { x, y },
                                tile: Tile,
                                layer: LayerIndex(layer_index),
                                ..Default::default()
                            })
                            .insert_bundle(SpriteSheetBundle {
                                texture_atlas: texture_atlas_handle.clone(),
                                sprite: sprite,
                                visibility: Visibility {
                                    is_visible: layer.visible,
                                },
                                ..Default::default()
                            });
                    }
                }
            }
        }

        commands
            .entity(map_entity)
            .insert(Name::new("simple"))
            .insert_bundle(MapBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                tiledmap: maps_assets.test.clone(),
                map: Map {
                    id: tiledmap.id,
                    width: tiledmap.inner.width,
                    height: tiledmap.inner.height,
                    tile_width: tiledmap.inner.tile_width,
                    tile_height: tiledmap.inner.tile_height,
                    tiles: tiles_entities,
                },
                ..Default::default()
            });
    }
}
