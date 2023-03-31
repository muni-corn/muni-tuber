use bevy::{prelude::*, winit::WinitSettings};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let img_handle: Handle<Image> = asset_server.load("png_muni_quiet.png");

    commands.spawn(Camera2dBundle::default());

    commands.spawn(ImageBundle {
        image: UiImage::new(img_handle),
        style: Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(32.),
                right: Val::Px(32.),
                ..Default::default()
            },
            size: Size::width(Val::Px(256.)),
            ..Default::default()
        },
        ..Default::default()
    });
}
