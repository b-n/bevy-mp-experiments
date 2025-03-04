use bevy::{
    app::{App, Startup, Update},
    color::palettes::basic::WHITE,
    core_pipeline::core_2d::Camera2d,
    ecs::system::{Commands, Query, Single},
    gizmos::gizmos::Gizmos,
    render::camera::Camera,
    transform::components::GlobalTransform,
    window::Window,
    DefaultPlugins,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_cursor)
        .run();
}

fn draw_cursor(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = *camera_query;

    let Ok(window) = windows.get_single() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    // Calculate a world position based on the cursor's position.
    let Ok(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    gizmos.circle_2d(point, 10., WHITE);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
