use bevy::prelude::*;
//use board_plugin::resources::BoardOptions;
//use board_plugin::BoardPlugin;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();
    // Window setup
    app.insert_resource(WindowDescriptor {
        title: "Mine Sweeper!".to_string(),
        width: 1200.,
        height: 800.,
        ..Default::default()
    })
    // Bevy default plugins
    .add_plugins(DefaultPlugins);

    //app.insert_resource(BoardOptions {
    //    map_size: (20, 20),
    //    bomb_count: 40,
    //    tile_padding: 3.0,
    //    ..Default::default()
    //})
    //.add_plugin(BoardPlugin);

    #[cfg(feature = "debug")]
    // Debug hierarchy inspector
    app.add_plugin(WorldInspectorPlugin::new());

    // Startup system (cameras)
    app.add_startup_system(camera_setup);
    app.add_startup_system(setup_track);
    // Run the app
    app.run();
}

fn setup_track(
    mut commands: Commands,
    window: Res<WindowDescriptor>,
    asset_server: Res<AssetServer>,
) {
    let track_image = asset_server.load("sprites/track1.png");
    commands.spawn_bundle(SpriteBundle {
        texture: track_image,
        transform: Transform::from_scale(Vec3::new(1.4, 1.4, 1.)),
        ..Default::default()
    })
    .insert(Name::new("Track"));

    let car_red_image = asset_server.load("sprites/car_yel.png");
    commands.spawn_bundle(SpriteBundle {
        texture: car_red_image,
        transform: Transform::identity()
            .with_translation(Vec3::new(0., 0., 1.))
            .with_scale(Vec3::new(0.1, 0.1, 1.)),
        ..Default::default()
    })
    .insert(Name::new("Red Car"));
}

fn camera_setup(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
