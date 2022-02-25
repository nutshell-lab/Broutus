use bevy::{
    asset::{AssetLoader, AssetPath, BoxedFuture, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::HashMap,
};
use std::{io::BufReader, path::Path};

pub struct TiledmapPlugin;

impl Plugin for TiledmapPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<Tiledmap>()
            .add_asset_loader(TiledmapLoader)
            .add_system(process_loaded_tiledmaps);
    }
}

#[derive(TypeUuid)]
#[uuid = "e51081d0-6168-4881-a1c6-1249b2000d7f"]
pub struct Tiledmap {
    pub id: u32,
    pub inner: tiled::Map,
    pub tilesets: HashMap<u32, Handle<Image>>,
    pub ground_layer: u16,
    pub highlight_layer: u16,
    pub obstacle_layer: u16,
    pub spawn_team_a_layer: u16,
    pub spawn_team_b_layer: u16,
}

#[derive(Default, Component)]
pub struct Map {
    pub id: u32,
    pub layers: HashMap<u32, Entity>,
}

#[derive(Default, Bundle)]
pub struct MapBundle {
    tiledmap: Handle<Tiledmap>,
    map: Map,
    #[bundle]
    sprite: SpriteSheetBundle,
}

#[derive(Default, Component)]
pub struct Layer {
    pub id: u32,
    pub tiles: HashMap<(u32, u32), Entity>,
    
}

#[derive(Default, Bundle)]
pub struct LayerBundle {
    layer: Layer,
    transform: Transform,
    global_transform: GlobalTransform,
    visibility: Visibility,
}

#[derive(Default, Component)]
pub struct Tile {
    pub id: u32,
    pub position: MapPosition,
    pub sprite_index: usize,
    pub in_tileset: u32,
}

#[derive(Default, Bundle)]
pub struct TileBundle {
    tile: Tile,
    transform: Transform,
    global_transform: GlobalTransform,
    visibility: Visibility,
}

#[derive(Default)]
pub struct MapPosition {
    x: u32,
    y: u32,
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
            let mut handles = HashMap::default();

            for tileset in &map.tilesets {
                let tile_path = root_dir.join(tileset.images.first().unwrap().source.as_str());
                let asset_path = AssetPath::new(tile_path, None);
                let texture: Handle<Image> = load_context.get_handle(asset_path.clone());

                // Associate tile id to the right texture (to support multiple tilesets) <TileId, TextureHandle>
                for i in tileset.first_gid..(tileset.first_gid + tileset.tilecount.unwrap_or(1)) {
                    handles.insert(i, texture.clone());
                }

                dependencies.push(asset_path);
            }

            let loaded_asset = LoadedAsset::new(Tiledmap {
                id: 0,
                inner: map,
                tilesets: handles,
                ground_layer: 0,
                highlight_layer: 1,
                obstacle_layer: 2,
                spawn_team_a_layer: 3,
                spawn_team_b_layer: 4,
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

pub fn process_loaded_tiledmaps(
    mut commands: Commands,
    maps: Res<Assets<Tiledmap>>,
    new_maps: Query<&Handle<Tiledmap>, Added<Handle<Tiledmap>>>,
) {
    for map_handle in new_maps.iter() {
        if let Some(map) = maps.get(map_handle) {
            let mut layer_entities = HashMap::default();
            let map_entity = commands.spawn().id();

            for layer in map.inner.layers.iter() {
                let mut tile_entities = HashMap::default();
                let layer_entity = commands.spawn().id();
                layer_entities.insert(layer.layer_index, layer_entity);
                commands.entity(map_entity).add_child(layer_entity);

                if let tiled::LayerData::Finite(tiles) = &layer.tiles {
                    for (tile_y, tiles) in tiles.iter().enumerate() {
                        for (tile_x, tile) in tiles.iter().enumerate() {
                            let tileset = map.inner.tilesets
                                .iter()
                                .filter(|tileset| tileset.first_gid < tile.gid)
                                .last()
                                .unwrap();
                            let (x, y) = (tile_x as u32, tile_y as u32);
                            let tile_entity = commands.spawn().id();
                            tile_entities.insert((x, y), tile_entity);
                            commands.entity(layer_entity).add_child(tile_entity);
                            commands.entity(tile_entity).insert_bundle(TileBundle {
                                tile: Tile {
                                    id: tile.gid,
                                    position: MapPosition { x, y },
                                    sprite_index: (tile.gid - tileset.first_gid) as usize,
                                    in_tileset: tileset.first_gid,
                                },
                                ..Default::default()
                            });
                        }
                    }
                }

                commands.entity(layer_entity).insert_bundle(LayerBundle {
                    layer: Layer {
                        id: layer.layer_index,
                        tiles: tile_entities,
                    },
                    ..Default::default()
                });
            }

            commands.entity(map_entity).insert_bundle(MapBundle {
                map: Map {
                    id: map.id,
                    layers: layer_entities
                },
                // Load the texture atlas resulting of merged tileset images
                // Warning, tilesets must be of the same size
                // Warning, tilesets must be lines of sprites
                sprite: SpriteSheetBundle {
                    // texture_atlas: // TODO get the sprite texture atlas handle,
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
}
