use crate::ui::TITLE_FONT_SIZE;
use crate::ui::credits_panel::ShowCreditsMessage;
use bevy::prelude::*;

pub(super) struct OutroPlugin;

impl Plugin for OutroPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameFinished>();

        app.add_systems(Startup, spawn_game_end_ui)
            .add_systems(Update, fade_out_game);
    }
}

#[derive(Resource, Default)]
pub(super) struct GameFinished {
    pub(super) is_finished: bool,
    fade_out: f32,
}

#[derive(Component)]
struct EndScreen;

#[derive(Component)]
struct ThxPlaying;

fn spawn_game_end_ui(mut commands: Commands) {
    commands.spawn((
        EndScreen,
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            padding: UiRect::all(percent(10)),
            display: Display::None,
            ..default()
        },
        BackgroundColor::from(Color::BLACK.with_alpha(0.0)),
        children![(
            ThxPlaying,
            Text::new("Thanks For Playing!"),
            TextFont::from_font_size(TITLE_FONT_SIZE)
        )],
    ));
}

const FADE_OUT_MAX: f32 = 5.0;

fn fade_out_game(
    mut game_end_container_query: Query<(&mut Node, &mut BackgroundColor), With<EndScreen>>,
    mut thx_play_txt_q: Query<&mut TextColor, With<ThxPlaying>>,
    mut game_finished: ResMut<GameFinished>,
    mut show_credits_message: MessageWriter<ShowCreditsMessage>,
    time: Res<Time>,
) {
    if !game_finished.is_finished {
        return;
    }

    let (mut node, mut bg_col) = game_end_container_query.single_mut().unwrap();
    let mut text_color = thx_play_txt_q.single_mut().unwrap();

    game_finished.fade_out += time.delta_secs();

    if game_finished.fade_out >= FADE_OUT_MAX {
        game_finished.fade_out = FADE_OUT_MAX;
        once!(show_credits_message.write_default());
    }

    let alpha = game_finished.fade_out / FADE_OUT_MAX;

    node.display = Display::Flex;

    bg_col.set_alpha(alpha);
    text_color.set_alpha(alpha);
}
