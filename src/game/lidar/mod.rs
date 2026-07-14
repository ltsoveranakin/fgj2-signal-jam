mod dots;

use crate::game::lidar::dots::LidarDotsPlugin;
use crate::game::player::Player;
use crate::game::target::GameTarget;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const LIDAR_DOTS_COUNT: usize = 360;
const CAST_EPSILON: f32 = 0.1;

pub(super) struct LidarPlugin;

impl Plugin for LidarPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LidarDotsPlugin);

        app.insert_resource(LidarCasts {
            casts_working: Vec::with_capacity(LIDAR_DOTS_COUNT),
            casts_done: Vec::with_capacity(LIDAR_DOTS_COUNT),
            cast_state: CastState::Inactive,
        });

        app.register_type::<LidarCasts>()
            .register_type::<LidarCast>()
            .register_type::<CastPosition>()
            .register_type::<CastState>();

        app.add_systems(Update, (send_out_lidar_dots, update_lidar_casts));
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub(crate) struct LidarCasts {
    pub(crate) casts_working: Vec<LidarCast>,
    pub(crate) casts_done: Vec<LidarCast>,
    pub(crate) cast_state: CastState,
}

#[derive(Reflect)]
pub(crate) struct LidarCast {
    pub(crate) cast_positions: Vec<CastPosition>,
    current_cast_distance: f32,
}

#[derive(Reflect)]
pub(crate) struct CastPosition {
    pub(crate) position: Vec2,
    direction: Vec2,
    entity_hit: Entity,
}

#[derive(Copy, Clone, PartialEq, Reflect)]
pub(crate) enum CastState {
    Inactive,
    Casting,
    Found {
        total_bounce_distance_to_target: f32,
    },
}

fn send_out_lidar_dots(
    player_query: Query<&Transform, With<Player>>,
    mut lidar_casts: ResMut<LidarCasts>,
    key_input: Res<ButtonInput<KeyCode>>,
) {
    if !key_input.just_pressed(KeyCode::Space) || !lidar_casts.casts_working.is_empty() {
        return;
    }

    let player_transform = player_query.single().unwrap();

    let step = (std::f32::consts::PI * 2.0) / (LIDAR_DOTS_COUNT as f32);

    for i in 0..LIDAR_DOTS_COUNT {
        let angle = i as f32 * step;
        // let angle = PI * 0.25;

        let direction = Vec2::from_angle(angle);

        lidar_casts.casts_working.push(LidarCast {
            cast_positions: vec![CastPosition {
                position: player_transform.translation.truncate(),
                direction,
                entity_hit: Entity::PLACEHOLDER,
            }],
            current_cast_distance: 0.0,
        });

        lidar_casts.cast_state = CastState::Casting;
    }
}

// R=D−2(D⋅N)N
// where D: dir
// N: surface normal
fn update_lidar_casts(
    player_query: Query<Entity, With<Player>>,
    target_query: Query<Entity, With<GameTarget>>,
    mut lidar_casts: ResMut<LidarCasts>,
    read_rapier_context: ReadRapierContext,
) {
    if lidar_casts.cast_state == CastState::Inactive {
        return;
    }

    let rapier_context = read_rapier_context.single().unwrap();

    let player_entity = player_query.single().unwrap();
    let target_entity = target_query.single().unwrap();

    let LidarCasts {
        casts_working,
        casts_done,
        cast_state,
    } = &mut *lidar_casts;

    let mut casts_to_remove = [false; LIDAR_DOTS_COUNT];

    for (
        i,
        LidarCast {
            cast_positions,
            current_cast_distance,
        },
    ) in casts_working.iter_mut().enumerate()
    {
        let cast_position = cast_positions.last().unwrap();

        let max_toi: bevy_rapier2d::prelude::Real = if let CastState::Found {
            total_bounce_distance_to_target,
        } = *cast_state
        {
            let max_toi = total_bounce_distance_to_target - *current_cast_distance;

            if max_toi <= 0.0 {
                casts_to_remove[i] = true;
                continue;
            }

            max_toi
        } else {
            f32::MAX
        };

        let solid = true;
        let predicate = |entity| {
            if entity == cast_position.entity_hit {
                false
            } else {
                true
            }
        };
        let query = QueryFilter::new()
            .exclude_collider(player_entity)
            .predicate(&predicate);

        // Cast positions will always have a length of at least 1

        let (entity_hit, cast_result) = if let Some(cast_result) = rapier_context
            .cast_ray_and_get_normal(
                cast_position.position,
                cast_position.direction,
                max_toi,
                solid,
                query,
            ) {
            cast_result
        } else {
            casts_to_remove[i] = true;
            continue;
        };

        *current_cast_distance += cast_result.time_of_impact;

        if entity_hit == target_entity {
            *cast_state = CastState::Found {
                total_bounce_distance_to_target: *current_cast_distance,
            };

            casts_to_remove[i] = true;
        }

        let reflected_dir = cast_position.direction
            - (2.0 * (cast_position.direction * cast_result.normal)) * cast_result.normal;

        cast_positions.push(CastPosition {
            position: cast_result.point,
            direction: reflected_dir,
            entity_hit,
        });
    }

    let mut i = 0;

    let casts_done_to_add = casts_working.extract_if(0.., |_| {
        let should_remove = casts_to_remove[i];

        i += 1;

        should_remove
    });

    for cast in casts_done_to_add {
        casts_done.push(cast);
    }
}
