use bevy::{prelude::*, scene::InstanceId};

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .insert_resource(SceneInstance::default())
        // 窗口背景色
        .insert_resource(ClearColor(Color::GRAY))
        .add_startup_system(setup.system())
        .add_system(scene_update.system())
        .add_system(move_scene_entities.system())
        // 自由摄像机
        .add_plugin(bevy_flycam::PlayerPlugin)
        .insert_resource(bevy_flycam::MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 12.0,          // default: 12.0
        })
        // esc退出系统
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}

// Resource to hold the scene `instance_id` until it is loaded
#[derive(Default)]
struct SceneInstance(Option<InstanceId>);

// Component that will be used to tag entities in the scene
struct EntityInMyScene;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
    mut scene_instance: ResMut<SceneInstance>,
) {
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 5.0, 4.0),
        light: Light {
            intensity: 2_000.,
            range: 100.,
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(-4.0, 5.0, -4.0),
        light: Light {
            intensity: 2_000.,
            range: 100.,
            ..Default::default()
        },
        ..Default::default()
    });
    // commands.spawn_bundle(PerspectiveCameraBundle {
    //     transform: Transform::from_xyz(1.05, 0.9, 1.5)
    //         .looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
    //     ..Default::default()
    // });

    // Spawn the scene as a child of another entity. This first scene will be translated backward
    // with its parent
    // commands
    //     .spawn_bundle((
    //         Transform::from_xyz(0.0, 0.0, -1.0),
    //         GlobalTransform::identity(),
    //     ))
    //     .with_children(|parent| {
    //         parent.spawn_scene(asset_server.load("glb/taxi.glb#Scene0"));
    //     });

    // Spawn a second scene, and keep its `instance_id`
    let instance_id = scene_spawner.spawn(asset_server.load("glb/taxi.glb#Scene0"));
    scene_instance.0 = Some(instance_id);
}

// This system will wait for the scene to be ready, and then tag entities from
// the scene with `EntityInMyScene`. All entities from the second scene will be
// tagged
fn scene_update(
    mut commands: Commands,
    scene_spawner: Res<SceneSpawner>,
    scene_instance: Res<SceneInstance>,
    mut done: Local<bool>,
) {
    if !*done {
        if let Some(instance_id) = scene_instance.0 {
            if let Some(entity_iter) = scene_spawner.iter_instance_entities(instance_id) {
                entity_iter.for_each(|entity| {
                    commands.entity(entity).insert(EntityInMyScene);
                });
                *done = true;
            }
        }
    }
}

// This system will move all entities with component `EntityInMyScene`, so all
// entities from the second scene
fn move_scene_entities(
    time: Res<Time>,
    mut scene_entities: Query<&mut Transform, With<EntityInMyScene>>,
) {
    let mut direction = 1.;
    let mut scale = 1.;
    for mut transform in scene_entities.iter_mut() {
        transform.translation = Vec3::new(
            scale * direction * time.seconds_since_startup().sin() as f32 / 10.,
            0.,
            time.seconds_since_startup().cos() as f32 / 10.,
        );
        direction *= -1.;
        scale += 1.;
    }
}
