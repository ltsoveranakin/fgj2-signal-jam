use crate::control::is_playing;
use crate::ui::start_menu::StartMenuContainer;
use bevy::prelude::*;

pub(super) struct CreditsPanelPlugin;

impl Plugin for CreditsPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ShowCreditsMessage>();

        app.add_systems(
            Update,
            (
                spawn_credits_panel.run_if(on_message::<ShowCreditsMessage>),
                close_credits.run_if(not(is_playing)),
            ),
        );
    }
}

#[derive(Message, Default)]
pub(crate) struct ShowCreditsMessage;

#[derive(Component)]
struct CreditsPanelContainer;

fn spawn_credits_panel(
    mut commands: Commands,
    mut start_container_query: Query<&mut Node, With<StartMenuContainer>>,
    asset_server: Res<AssetServer>,
) {
    let mut start_container_node = start_container_query.single_mut().unwrap();
    start_container_node.display = Display::None;

    commands
        .spawn((
            CreditsPanelContainer,
            Node {
                width: percent(100),
                height: percent(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::BLACK.with_alpha(0.6)),
            ZIndex(10),
        ))
        .with_child((
            Node {
                width: vmin(80),
                height: vmin(80),
                ..default()
            },
            ImageNode::new(asset_server.load("image/menu/credits.png")),
        ));
}

fn close_credits(
    mut commands: Commands,
    credits_container_query: Query<Entity, With<CreditsPanelContainer>>,
    mut start_container_query: Query<&mut Node, With<StartMenuContainer>>,
    key_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
) {
    if key_input.any_just_pressed([KeyCode::Enter, KeyCode::Space, KeyCode::Escape])
        || mouse_input.just_pressed(MouseButton::Left)
    {
        if let Ok(container_entity) = credits_container_query.single() {
            let mut start_container_node = start_container_query.single_mut().unwrap();
            start_container_node.display = Display::Flex;

            commands.entity(container_entity).despawn();
        }
    }
}
