use bevy::prelude::Component;

use bevy::prelude::*;

pub(super) struct GlowPlugin;

impl Plugin for GlowPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<StopGlowMessage>();

        app.add_systems(
            Update,
            (
                emanate_glow,
                stop_glowing.run_if(on_message::<StopGlowMessage>),
            ),
        );
    }
}

#[derive(Message)]
pub(super) struct StopGlowMessage;

#[derive(Component)]
pub(super) struct Glow {
    pub(super) is_glowing: bool,
    pub(super) glow_color: LinearRgba,
}

fn emanate_glow(mut glow_query: Query<(&Glow, &mut Sprite), With<Glow>>, time: Res<Time>) {
    for (glow, mut sprite) in glow_query.iter_mut() {
        let alpha = sprite.color.alpha();

        if !glow.is_glowing {
            sprite.color = Color::WHITE.with_alpha(alpha);
            continue;
        }

        let col_offset = (time.elapsed_secs().sin()) + 0.5;

        let color = glow.glow_color.to_vec3() + col_offset;

        sprite.color = LinearRgba::from_vec4(color.extend(alpha)).into();
    }
}

fn stop_glowing() {}
