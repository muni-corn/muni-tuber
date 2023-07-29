use bevy::{prelude::*, winit::WinitSettings};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    HostId, InputCallbackInfo, Sample, SampleFormat, Stream,
};
use rand::Rng;

use std::sync::{Arc, Mutex};

#[derive(Default, Resource)]
struct AudioState {
    volume: Arc<Mutex<f32>>,
}

fn main() {
    let (audio_state, stream) = start_default_stream();

    App::new()
        .insert_resource(audio_state)
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::game())
        .add_startup_system(setup)
        .add_system(animate_with_audio)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("speaking_atlas.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(200.0, 200.0), 3, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        ..Default::default()
    });
}

// fn animate_sprite_system(
//     time: Res<Time>,
//     texture_atlases: Res<Assets<TextureAtlas>>,
//     mut query: Query<(
//         &mut SpriteAnimation,
//         &Handle<TextureAtlas>,
//         &mut TextureAtlasSprite,
//     )>,
//     audio_state: Res<AudioState>,
// ) {
//     println!(audio_state);
//     for (mut sprite_animation, texture_atlas_handle, mut sprite) in query.iter_mut() {
//         sprite_animation.timer.tick(time.delta());

//         if sprite_animation.timer.finished() {
//             sprite_animation.frame += rand::thread_rng().gen_range(1..=2);

//             if let Some(texture_atlas) = texture_atlases.get(texture_atlas_handle) {
//                 sprite.index = sprite_animation.frame % texture_atlas.textures.len();
//             }
//         }
//     }
// }

const HALF_SPEAK_THRESHOLD_DBFS: f32 = -30.0;
const FULL_SPEAK_THRESHOLD_DBFS: f32 = -20.0;

fn animate_with_audio(audio_state: ResMut<AudioState>, mut query: Query<&mut TextureAtlasSprite>) {
    if let Some(mut sprite) = query.iter_mut().next() {
        let volume = *audio_state.volume.lock().unwrap();
        if volume > FULL_SPEAK_THRESHOLD_DBFS {
            sprite.index = 2;
        } else if volume > HALF_SPEAK_THRESHOLD_DBFS {
            sprite.index = 1;
        } else {
            sprite.index = 0;
        }
    }
}

fn u16_to_dbfs(volume: u16) -> f32 {
    let normalized = volume as f32 / u16::MAX as f32;
    20.0 * normalized.log10()
}

fn start_default_stream() -> (AudioState, Stream) {
    let host = cpal::default_host();

    let device = host
        .default_input_device()
        .expect("no input device available");

    let config = device
        .default_input_config()
        .expect("no input config available");

    let sample_format = config.sample_format();

    let volume = Arc::new(Mutex::new(0.0));
    let volume_clone = volume.clone();
    let err_fn = |e| eprintln!("error occurred on stream: {}", e);
    let stream = match sample_format {
        SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data: &[f32], _: &InputCallbackInfo| {
                let max_sample_value = data
                    .iter()
                    .map(|sample| (sample.abs() * u16::MAX as f32) as u16)
                    .max()
                    .unwrap_or(0);
                *volume_clone.lock().unwrap() = u16_to_dbfs(max_sample_value)
            },
            err_fn,
            None,
        ),
        _ => unimplemented!(),
    }
    .unwrap();

    stream.play().expect("failed to play stream");

    return (AudioState { volume }, stream);
}
