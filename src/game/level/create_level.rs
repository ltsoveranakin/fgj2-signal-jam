use crate::game::level::{LevelData, LevelsData, SetLevelMessage, WallData};
use crate::game::maze::generator::{HALF_TILE_SIZE_F32, TILE_SIZE_F32};
use crate::game::z_coord::MAZE_Z_COORD;
use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapBundle;
use bevy_ecs_tilemap::map::{TilemapId, TilemapTexture, TilemapTileSize, TilemapType};
use bevy_ecs_tilemap::prelude::{TileBundle, TileStorage, TileTextureIndex};
use bevy_rapier2d::dynamics::RigidBody;
use bevy_rapier2d::geometry::Collider;

const PATH_TEXTURE_INDEX: usize = 0;
const WALL_TEXTURE_INDEX: usize = 1;

pub(super) struct CreateLevelPlugin;

impl Plugin for CreateLevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, create_level_on_set);
    }
}

fn create_level_on_set(
    mut commands: Commands,
    levels_data: Res<LevelsData>,
    asset_server: Res<AssetServer>,
    mut set_level_message: MessageReader<SetLevelMessage>,
) {
    for set_level in set_level_message.read() {
        let LevelData {
            player_spawn,
            target_spawn,
            level_size,
            walls,
        } = &levels_data.levels[set_level.0];

        let map_size = (*level_size).into();

        let mut tile_storage = TileStorage::empty(map_size);

        let tilemap_entity = commands.spawn_empty().id();

        for WallData { dim, coord } in walls {
            for x in 0..dim.x {
                for y in 0..dim.y {
                    let cell_coord = UVec2::new(x, y);

                    let map_coord = coord + cell_coord;
                    let tile_pos = map_coord.into();

                    if tile_storage.get(&tile_pos).is_some() {
                        continue;
                    }

                    let tile_entity = commands
                        .spawn((
                            TileBundle {
                                position: tile_pos,
                                tilemap_id: TilemapId(tilemap_entity),
                                texture_index: TileTextureIndex(WALL_TEXTURE_INDEX as u32),
                                ..default()
                            },
                            Transform::from_translation(
                                map_coord.as_vec2().extend(0.0) * TILE_SIZE_F32,
                            ),
                            Collider::cuboid(HALF_TILE_SIZE_F32, HALF_TILE_SIZE_F32),
                            RigidBody::Fixed,
                            Name::new("Tile"),
                        ))
                        .id();

                    tile_storage.set(&tile_pos, tile_entity);
                    commands.entity(tilemap_entity).add_child(tile_entity);
                }
            }
        }

        let path = asset_server.load("image/tile/path.png");
        let wall = asset_server.load("image/tile/wall.png");

        let maze_tile_image_assets = vec![path, wall];

        let tile_size = TilemapTileSize::new(TILE_SIZE_F32, TILE_SIZE_F32);
        let grid_size = tile_size.into();
        let map_type = TilemapType::Square;

        commands.entity(tilemap_entity).insert((
            TilemapBundle {
                grid_size,
                map_type,
                size: map_size,
                storage: tile_storage,
                texture: TilemapTexture::Vector(maze_tile_image_assets.to_vec()),
                tile_size,
                transform: Transform::from_xyz(0.0, 0.0, MAZE_Z_COORD),
                ..default()
            },
            Name::new("TileMap"),
        ));
    }
}
