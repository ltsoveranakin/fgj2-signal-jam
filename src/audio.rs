use crate::ui::story_panel::StartGameMessage;
use bevy::audio::Volume;
use bevy::prelude::*;

const FADE_TIME_TOTAL: f32 = 7.0;

pub(super) struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                fade_audio,
                play_music_on_game_play.run_if(on_message::<StartGameMessage>),
            ),
        );
    }
}

#[derive(Component)]
pub(super) struct FadeAudio {
    fade_in: bool,
    max_vol: f32,
    fade_time_in: f32,
}

impl FadeAudio {
    fn new(fade_in: bool, max_vol: f32) -> Self {
        Self {
            fade_in,
            max_vol,
            fade_time_in: 0.0,
        }
    }

    pub(super) fn fade_in(max_vol: f32) -> Self {
        Self::new(true, max_vol)
    }

    pub(super) fn fade_out(max_vol: f32) -> Self {
        Self::new(false, max_vol)
    }
}

fn fade_audio(
    mut commands: Commands,
    mut fade_audio_query: Query<(Entity, &mut FadeAudio, &mut AudioSink)>,
    time: Res<Time>,
) {
    for (entity, mut fade_audio, mut audio_sink) in fade_audio_query.iter_mut() {
        let percent = fade_audio.fade_time_in / FADE_TIME_TOTAL;

        let volume_percent = if fade_audio.fade_in {
            percent
        } else {
            1.0 - percent
        };

        audio_sink.set_volume(Volume::Linear(volume_percent * fade_audio.max_vol));

        fade_audio.fade_time_in += time.delta_secs();

        if fade_audio.fade_time_in > FADE_TIME_TOTAL {
            if !fade_audio.fade_in {
                audio_sink.pause();

                commands.entity(entity).remove::<(AudioSink, AudioPlayer)>();
            }

            commands.entity(entity).remove::<FadeAudio>();
        }
    }
}

#[derive(Component)]
struct MainGameMusic;

fn play_music_on_game_play(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        MainGameMusic,
        AudioPlayer::new(asset_server.load("audio/music/sector.ogg")),
        PlaybackSettings::LOOP,
        FadeAudio::fade_in(1.0),
    ));
}
