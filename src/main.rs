use bevy::{prelude::*, winit::WinitSettings};
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_startup_system(setup)
        .add_system(animate_sprite_system)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("speaking_atlas.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(200.0, 200.0), 3, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            ..Default::default()
        })
        .insert(SpriteAnimation {
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
            frame: 0,
        });
}

#[derive(Component)]
struct SpriteAnimation {
    timer: Timer,
    frame: usize,
}

fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut SpriteAnimation,
        &Handle<TextureAtlas>,
        &mut TextureAtlasSprite,
    )>,
) {
    for (mut sprite_animation, texture_atlas_handle, mut sprite) in query.iter_mut() {
        sprite_animation.timer.tick(time.delta());

        if sprite_animation.timer.finished() {
            sprite_animation.frame += rand::thread_rng().gen_range(1..=2);

            if let Some(texture_atlas) = texture_atlases.get(texture_atlas_handle) {
                sprite.index = sprite_animation.frame % texture_atlas.textures.len();
            }
        }
    }
}
