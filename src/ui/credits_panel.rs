use crate::control::inputs_allowed;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;

pub(super) struct CreditsPanelPlugin;

impl Plugin for CreditsPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ShowCreditsMessage>();

        app.add_systems(
            Update,
            (
                spawn_credits_panel.run_if(on_message::<ShowCreditsMessage>),
                credits_panel_click.run_if(not(inputs_allowed)),
            ),
        );
    }
}

#[derive(Message, Default)]
pub(super) struct ShowCreditsMessage;

#[derive(Component)]
struct CreditsPanelContainer;

fn spawn_credits_panel(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            FocusPolicy::Block,
            Button,
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

fn credits_panel_click(
    mut commands: Commands,
    mut credits_container_interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<CreditsPanelContainer>),
    >,
    credits_container_query: Query<Entity, With<CreditsPanelContainer>>,
    key_input: Res<ButtonInput<KeyCode>>,
) {
    let mut should_close = false;

    for interaction in credits_container_interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            should_close = true;
        }
    }

    if key_input.any_just_pressed([
        KeyCode::Escape,
        KeyCode::Space,
        KeyCode::Enter,
        KeyCode::KeyE,
    ]) {
        should_close = true;
    }

    if should_close {
        if let Ok(container_entity) = credits_container_query.single() {
            commands.entity(container_entity).despawn();
        }
    }
}
