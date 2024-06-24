use std::f32::consts::{PI, TAU};

use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<TargetingRay>()
        .add_systems(Startup, (setup_ground, setup_camera))
        .add_systems(Update, (update_target, (rotate_camera, draw_gizmo)).chain())
        .run();
}

pub fn setup_ground(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    cmds.spawn(MaterialMeshBundle {
        mesh: meshes.add(Cuboid::new(1.5, 1., 1.5)),
        material: mats.add(StandardMaterial {
            base_color: Color::GRAY,
            ..default()
        }),
        transform: Transform::from_scale(Vec3::new(500.0, 5.0, 500.0))
            .with_translation(Vec3::new(0.0, 25.0, 0.0)),
        ..default()
    });
}

const CAMERA_CIRCLE_RADIUS: f32 = 85.;
const CAMERA_HEIGHT: f32 = 580.;
const CAMERA_FOCUS_HEIGHT: f32 = 415.;

fn camera_angle_to_tsf(angle: f32) -> Transform {
    let camera_plane_point = Vec2::from_angle(angle) * CAMERA_CIRCLE_RADIUS;
    Transform::from_xyz(camera_plane_point.x, CAMERA_HEIGHT, camera_plane_point.y)
        .looking_at(Vec3::Y * CAMERA_FOCUS_HEIGHT, Vec3::Y)
}

fn setup_camera(mut cmds: Commands) {
    cmds.spawn((
        CameraRig::default(),
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: camera_angle_to_tsf(0.),
            global_transform: GlobalTransform::default(),
            ..default()
        },
        BloomSettings::OLD_SCHOOL,
    ));

    cmds.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -45.0, 45.0, 0.0)),
        ..default()
    });
}

fn update_target(
    mut target: ResMut<TargetingRay>,
    window: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform), With<CameraRig>>,
) {
    let window = window.single();

    let Some(cursor_window_pos) = window.cursor_position() else {
        **target = None;
        return;
    };

    let (camera, gtsf) = cameras.single();

    **target = camera.viewport_to_world(gtsf, cursor_window_pos);
}

/// Where on the screen the player is clicking/tapping, ranging from 0 to 1
#[derive(Resource, DerefMut, Deref, Default)]
struct WindowTarget(Option<Vec2>);

/// The ray pointing toward what the player is clicking/tapping on
#[derive(Resource, DerefMut, Deref, Default, Debug)]
struct TargetingRay(Option<Ray3d>);

#[derive(Component, Default)]
struct CameraRig {
    target: Option<f32>,
}

const TOP_OF_GROUND: f32 = 27.5;

fn rotate_camera(
    mut cameras_query: Query<(&mut CameraRig, &mut Transform)>,
    targeting_ray: Res<TargetingRay>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    let (mut rig, mut tsf) = cameras_query.single_mut();

    if mouse.pressed(MouseButton::Left) {
        let cam_theta = tsf.translation.xz().to_angle();

        let Some(ray) = **targeting_ray else {
            return;
        };

        // The default plane points up
        let intersection_dist = ray
            .intersect_plane(Vec3::Y * TOP_OF_GROUND, Plane3d::default())
            .unwrap();

        let targeted_point = ray.get_point(intersection_dist).xz();
        let target_theta = targeted_point.to_angle();

        if let Some(original_target_theta) = rig.target {
            let difference = target_theta - original_target_theta;
            let difference = if difference > PI {
                difference - TAU
            } else if difference < -PI {
                difference + TAU
            } else {
                difference
            };

            let target_cam_theta = cam_theta - difference;

            *tsf = camera_angle_to_tsf(target_cam_theta);
        } else {
            rig.target = Some(target_theta);
        }
    } else {
        rig.target = None;
    }
}

fn draw_gizmo(targeting_ray: Res<TargetingRay>, mut gizmos: Gizmos) {
    let Some(ray) = **targeting_ray else {
        return;
    };

    // The default plane points up
    let intersection_dist = ray
        .intersect_plane(Vec3::Y * TOP_OF_GROUND, Plane3d::default())
        .unwrap();
    let intersection = ray.get_point(intersection_dist) + Vec3::Y * 5.;

    gizmos.circle(intersection, Direction3d::Y, 40., Color::RED * 5.);
}
