use crate::assign_vec::AssignVec;
use crate::control::inputs_allowed;
use crate::game::level::WallType;
use crate::game::level::create_level::LevelParent;
use crate::game::player::{Player, PlayerFacing};
use crate::game::target::{GameTarget, HitTargetMessage};
use crate::game::z_coord::LIDAR_DOT_Z_COORD;
use bevy::audio::Volume;
use bevy::color::palettes::css;
use bevy::prelude::*;
use bevy_rapier2d::pipeline::QueryFilter;
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;

const LIDAR_DOT_SPEED: f32 = 150.0;
const LIDAR_DOTS_COUNT: usize = 20;
const LIDAR_CONE: f32 = PI / 3.0;
const WALL_DOT_ALIVE_TIME: f32 = 10.0;
const LIDAR_DOT_ALIVE_TIME: f32 = 10.0;
const BOUNCE_ALIVE_TIME_PENALTY: f32 = 0.3;
const LIDAR_COOLDOWN: f32 = 5.0;

pub(super) struct LidarDotsPlugin;

impl Plugin for LidarDotsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SpawnDotsMessage>();

        app.init_resource::<LidarNextUseTime>();

        app.add_systems(Startup, load_ping_sound);

        app.add_systems(
            Update,
            (
                shoot_lidar.run_if(inputs_allowed),
                spawn_dots,
                move_dots,
                fade_wall_dots,
            ),
        );
    }
}

#[derive(Component)]
struct LidarDot {
    direction: Vec2,
    last_entity_hit: Entity,
}

#[derive(Component)]
struct FadeDot {
    total_fade_time: f32,
    fade_time_elapsed: f32,
    fade_from: Color,
    fade_to: Color,
}

impl FadeDot {
    fn new_alpha(total_fade_time: f32, fade_from: impl Into<Color>) -> Self {
        let fade_from = fade_from.into();

        Self::new(total_fade_time, fade_from, fade_from.with_alpha(0.0))
    }

    fn new(total_fade_time: f32, fade_from: impl Into<Color>, fade_to: impl Into<Color>) -> Self {
        Self {
            total_fade_time,
            fade_time_elapsed: 0.0,
            fade_from: fade_from.into(),
            fade_to: fade_to.into(),
        }
    }
}

#[derive(Resource, Default)]
pub(crate) struct LidarNextUseTime(pub(crate) f32);

#[derive(Message)]
pub(crate) struct SpawnDotsMessage {
    pub(crate) location: Vec2,
    pub(crate) fov: f32,
    pub(crate) direction: f32,
    pub(crate) dot_count: usize,
    pub(crate) alive_time: f32,
}

#[derive(Resource)]
struct PingSound(Handle<AudioSource>);

fn load_ping_sound(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PingSound(asset_server.load("audio/sfx/sonar_ping.ogg")));
}

fn spawn_dots(
    mut commands: Commands,
    level_parent_query: Query<Entity, With<LevelParent>>,
    asset_server: Res<AssetServer>,
    ping_sound: Res<PingSound>,
    mut lidar_dot_sprite_cache: Local<Option<Handle<Image>>>,
    mut spawn_dots_message: MessageReader<SpawnDotsMessage>,
) {
    for spawn_dots in spawn_dots_message.read() {
        let lidar_dot_sprite = lidar_dot_sprite_cache
            .get_or_insert_with(|| asset_server.load("image/particle/lidar_dot.png"));

        let translation = spawn_dots.location.extend(LIDAR_DOT_Z_COORD);

        let step = (spawn_dots.fov) / (spawn_dots.dot_count as f32);

        let parent_entity = level_parent_query.single().unwrap();
        let ping_audio_handle = ping_sound.0.clone();

        commands.entity(parent_entity).with_children(move |parent| {
            for i in 0..spawn_dots.dot_count {
                let angle = (i as f32 * step) + (spawn_dots.fov * -0.5) + spawn_dots.direction;
                let direction = Vec2::from_angle(angle);

                parent.spawn((
                    LidarDot {
                        direction,
                        last_entity_hit: Entity::PLACEHOLDER,
                    },
                    Sprite::from_image(lidar_dot_sprite.clone()),
                    Transform {
                        translation,
                        scale: Vec3::splat(0.5),
                        ..default()
                    },
                    FadeDot::new_alpha(spawn_dots.alive_time, css::RED),
                    AudioPlayer(ping_audio_handle.clone()),
                    PlaybackSettings::default()
                        .paused()
                        .with_volume(Volume::Linear(0.25)),
                ));
            }
        });
    }
}

fn shoot_lidar(
    player_query: Query<(&Transform, &PlayerFacing)>,
    mut lidar_next_use_time: ResMut<LidarNextUseTime>,
    key_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut spawn_dots_message: MessageWriter<SpawnDotsMessage>,
) {
    if !key_input.just_pressed(KeyCode::Space) || time.elapsed_secs() <= lidar_next_use_time.0 {
        return;
    }

    let (transform, facing) = player_query.single().unwrap();

    spawn_dots_message.write(SpawnDotsMessage {
        location: transform.translation.truncate(),
        fov: LIDAR_CONE,
        direction: facing.direction().to_angle(),
        dot_count: LIDAR_DOTS_COUNT,
        alive_time: LIDAR_DOT_ALIVE_TIME,
    });

    lidar_next_use_time.0 = time.elapsed_secs() + LIDAR_COOLDOWN;
}

fn move_dots(
    mut commands: Commands,
    mut dot_query: Query<(
        Entity,
        &mut LidarDot,
        &mut FadeDot,
        &mut Transform,
        &AudioSink,
    )>,
    player_query: Query<Entity, With<Player>>,
    target_query: Query<(Entity, &GameTarget)>,
    wall_type_query: Query<&WallType>,
    level_parent_query: Query<Entity, With<LevelParent>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut hit_target_message: MessageWriter<HitTargetMessage>,
    rapier_context: ReadRapierContext,
) {
    let player_entity = player_query.single().unwrap();
    let (target_entity, target) = target_query.single().unwrap();
    let rapier_context = rapier_context.single().unwrap();

    let toi: bevy_rapier2d::prelude::Real = LIDAR_DOT_SPEED * time.delta_secs();

    for (dot_entity, mut lidar_dot, mut fade_dot, mut dot_transform, audio_sink) in
        dot_query.iter_mut()
    {
        let current_dot_position = dot_transform.translation.truncate();

        let solid = true;
        let predicate = |entity| {
            if (entity == lidar_dot.last_entity_hit)
                || (target.is_active && entity == target_entity)
            {
                false
            } else {
                true
            }
        };

        let query = QueryFilter::new()
            .exclude_collider(player_entity)
            .predicate(&predicate);

        debug_assert!(lidar_dot.direction.is_normalized());

        let (entity_hit, hit_result) = if let Some(result) = rapier_context.cast_ray_and_get_normal(
            current_dot_position,
            lidar_dot.direction,
            toi,
            solid,
            query,
        ) {
            result
        } else {
            dot_transform
                .translation
                .assign_from(current_dot_position + (lidar_dot.direction * toi));
            continue;
        };

        dot_transform.translation.assign_from(hit_result.point);

        let mut absorbed = None;

        if let Ok(wall_type) = wall_type_query.get(entity_hit) {
            match wall_type {
                WallType::Absorb => {
                    absorbed = Some(FadeDot::new_alpha(1.0, css::TEAL));
                }

                WallType::Exit => {
                    absorbed = Some(FadeDot::new_alpha(10.0, css::GREEN));
                }

                WallType::Solid => {}
            }
        }

        if entity_hit == target_entity {
            absorbed = Some(FadeDot::new_alpha(1.5, css::YELLOW));
            hit_target_message.write_default();
            audio_sink.play();
        }

        if let Some(absorbed) = absorbed {
            *fade_dot = absorbed;
            commands.entity(dot_entity).remove::<LidarDot>();
            continue;
        }

        let level_parent = level_parent_query.single().unwrap();

        commands.entity(level_parent).with_child((
            FadeDot::new_alpha(WALL_DOT_ALIVE_TIME, Color::WHITE),
            Sprite::from_image(asset_server.load("image/particle/lidar_dot.png")),
            *dot_transform,
        ));

        let reflected_dir = (lidar_dot.direction
            - (2.0 * (lidar_dot.direction * hit_result.normal)) * hit_result.normal)
            .normalize();

        lidar_dot.last_entity_hit = entity_hit;
        lidar_dot.direction = reflected_dir;

        fade_dot.fade_time_elapsed += BOUNCE_ALIVE_TIME_PENALTY;
    }
}

fn fade_wall_dots(
    mut commands: Commands,
    mut fade_dot_query: Query<(Entity, &mut FadeDot, &mut Sprite), With<FadeDot>>,
    time: Res<Time>,
) {
    for (entity, mut fade_dot, mut sprite) in fade_dot_query.iter_mut() {
        if fade_dot.fade_time_elapsed >= fade_dot.total_fade_time {
            commands.entity(entity).despawn();
            continue;
        }

        let percent = fade_dot.fade_time_elapsed / fade_dot.total_fade_time;

        let current_color = fade_dot.fade_from.mix(&fade_dot.fade_to, percent);

        sprite.color = current_color;

        fade_dot.fade_time_elapsed += time.delta_secs();
    }
}
