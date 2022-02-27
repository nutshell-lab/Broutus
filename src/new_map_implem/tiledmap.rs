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
        app.register_type::<MapPosition>()
            .register_type::<Map>()
            .register_type::<Layer>()
            .register_type::<Tile>()
            .add_asset::<Tiledmap>()
            .add_asset_loader(TiledmapLoader)
            .add_startup_system(startup)
            .add_system(process_loaded_tiledmaps);
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle: Handle<Tiledmap> = asset_server.load("maps/simple.tmx");
    let map_entity = commands.spawn().id();
    commands
        .entity(map_entity)
        .insert(Name::new("map"))
        .insert_bundle(MapBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            tiledmap: handle,
            ..Default::default()
        });
}

#[derive(TypeUuid)]
#[uuid = "e51081d0-6168-4881-a1c6-1249b2000d7f"]
pub struct Tiledmap {
    pub id: u32,
    pub inner: tiled::Map,
    pub tileset: Handle<Image>,
    pub ground_layer: u16,
    pub highlight_layer: u16,
    pub obstacle_layer: u16,
    pub spawn_team_a_layer: u16,
    pub spawn_team_b_layer: u16,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Map {
    pub id: u32,
    pub layers: HashMap<u32, Entity>,
}

#[derive(Default, Bundle)]
pub struct MapBundle {
    map: Map,
    tiledmap: Handle<Tiledmap>,
    transform: Transform,
    global_transform: GlobalTransform,
    visibility: Visibility,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
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

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tile {
    pub id: u32,
    pub sprite_index: usize,
    pub in_tileset: u32,
}

#[derive(Default, Bundle)]
pub struct TileBundle {
    tile: Tile,
    position: MapPosition,
    transform: Transform,
    global_transform: GlobalTransform,
    visibility: Visibility,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct MapPosition {
    x: u32,
    y: u32,
}

impl MapPosition {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
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
            let tile_path = root_dir.join(tileset.images.first().unwrap().source.as_str());
            let asset_path = AssetPath::new(tile_path, None);
            let texture: Handle<Image> = load_context.get_handle(asset_path.clone());

            dependencies.push(asset_path);

            let loaded_asset = LoadedAsset::new(Tiledmap {
                id: 0,
                inner: map,
                tileset: texture,
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
    mut map_events: EventReader<AssetEvent<Tiledmap>>,
    tiledmaps: Res<Assets<Tiledmap>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut maps: Query<(Entity, &mut Map, &Handle<Tiledmap>)>,
) {
    let mut changed_tiledmaps = Vec::<Handle<Tiledmap>>::default();
    for event in map_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                changed_tiledmaps.push(handle.clone());
            }
            _ => {}
        }
    }

    for changed_map in changed_tiledmaps.iter() {
        for (map_entity, mut map, map_handle) in maps.iter_mut() {
            if map_handle != changed_map {
                continue;
            }

            if let Some(tiledmap) = tiledmaps.get(map_handle) {
                let mut layer_entities = HashMap::default();

                let tileset = tiledmap
                    .inner
                    .tilesets
                    .first()
                    .expect("Tiledmap needs a tileset");
                let texture_atlas = TextureAtlas::from_grid(
                    tiledmap.tileset.clone(),
                    Vec2::new(tileset.tile_width as f32, tileset.tile_height as f32),
                    tileset.tilecount.unwrap_or(1) as usize,
                    1,
                );
                let texture_atlas_handle = texture_atlases.add(texture_atlas);

                for layer in tiledmap.inner.layers.iter() {
                    let mut tile_entities = HashMap::default();
                    let layer_entity = commands.spawn().insert(Name::new("layer")).id();
                    layer_entities.insert(layer.layer_index, layer_entity);
                    commands.entity(map_entity).add_child(layer_entity);

                    if let tiled::LayerData::Finite(tiles_y) = &layer.tiles {
                        for (tile_y, tiles_x) in tiles_y.iter().enumerate() {
                            for (tile_x, tile) in tiles_x.iter().enumerate() {
                                if tile.gid == 0 {
                                    continue;
                                }
                                let tileset = tiledmap
                                    .inner
                                    .tilesets
                                    .iter()
                                    .filter(|tileset| tileset.first_gid <= tile.gid)
                                    .last()
                                    .unwrap();
                                let (x, y) = (tile_x as u32, tile_y as u32);
                                let tile_entity = commands
                                    .spawn()
                                    .insert(Name::new(format!("tile ({:02},{:02})", x, y)))
                                    .id();

                                let world_position = project_iso(
                                    MapPosition::new(x, y),
                                    tiledmap.inner.tile_width as f32,
                                    tiledmap.inner.tile_height as f32,
                                );

                                tile_entities.insert((x, y), tile_entity);
                                commands.entity(layer_entity).add_child(tile_entity);
                                commands
                                    .entity(tile_entity)
                                    .insert_bundle(TileBundle {
                                        position: MapPosition { x, y },
                                        tile: Tile {
                                            id: tile.gid,
                                            sprite_index: (tile.gid - tileset.first_gid) as usize,
                                            in_tileset: tileset.first_gid,
                                        },
                                        ..Default::default()
                                    })
                                    .insert_bundle(SpriteSheetBundle {
                                        texture_atlas: texture_atlas_handle.clone(),
                                        sprite: TextureAtlasSprite::new(tile.gid as usize - 1),
                                        visibility: Visibility {
                                            is_visible: layer.visible,
                                        },
                                        transform: Transform::from_xyz(
                                            world_position.x,
                                            world_position.y,
                                            (x + y) as f32 / (tiles_x.len() + tiles_y.len()) as f32,
                                        ),
                                        ..Default::default()
                                    });
                            }
                        }
                    }

                    commands.entity(layer_entity).insert_bundle(LayerBundle {
                        transform: Transform::from_xyz(0.0, 0.0, layer.layer_index as f32),
                        layer: Layer {
                            id: layer.layer_index,
                            tiles: tile_entities,
                        },
                        ..Default::default()
                    });
                }

                map.id = tiledmap.id;
                map.layers = layer_entities;
            }
        }
    }
}

/// TilePos --> WorldPos
pub fn project_iso(pos: MapPosition, tile_width: f32, tile_height: f32) -> Vec2 {
    let x = (pos.x as f32 - pos.y as f32) * tile_width / 2.0;
    let y = (pos.x as f32 + pos.y as f32) * tile_height / 2.0;
    return Vec2::new(x, -y);
}

/// WorldPos --> TilePos
pub fn unproject_iso(pos: Vec2, tile_width: f32, tile_height: f32) -> MapPosition {
    let half_width = tile_width / 2.0;
    let half_height = tile_height / 2.0;
    let x = ((pos.x / half_width) + (-(pos.y) / half_height)) / 2.0;
    let y = ((-(pos.y) / half_height) - (pos.x / half_width)) / 2.0;
    MapPosition::new(x.round() as u32, y.round() as u32)
}
