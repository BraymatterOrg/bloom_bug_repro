use std::f32::consts::FRAC_PI_2;

use bevy::{
    color::palettes::css::{RED, WHITE},
    prelude::*,
};
use bevy_vector_shapes::{painter::ShapePainter, shapes::DiscPainter, ShapePlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ShapePlugin::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, draw_gizmo)
        .run();
}

pub fn setup(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    // Remove this statement to show the gizmo
    cmds.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::default()),
        material: mats.add(Color::from(WHITE)),
        ..default()
    });

    cmds.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::Y * 10.).looking_at(Vec3::ZERO, Dir3::Y),
        ..default()
    });
}

fn draw_gizmo(mut painter: ShapePainter) {
    painter.rotate_x(FRAC_PI_2);
    painter.hollow = true;
    painter.color = RED.into();
    painter.circle(2.);
}
