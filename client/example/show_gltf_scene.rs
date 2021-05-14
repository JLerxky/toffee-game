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

/// 记录场景实体ID
#[derive(Default)]
struct SceneInstance(Option<InstanceId>);

/// 场景标记
struct EntityInMyScene;

/// 加载环境与场景，并记录场景实体ID
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
    mut scene_instance: ResMut<SceneInstance>,
) {
    // 多角度光源
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 5.0, 4.0),
        light: Light {
            intensity: 2_000.,
            range: 100.,
            ..Default::default()
        },
        ..Default::default()
    });
    // 多角度光源
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(-4.0, 5.0, -4.0),
        light: Light {
            intensity: 2_000.,
            range: 100.,
            ..Default::default()
        },
        ..Default::default()
    });

    // 加载对照场景
    commands
        .spawn_bundle((
            Transform {
                translation: Vec3::new(0., 0., -10.),
                scale: Vec3::new(1., 1., 1.),
                rotation: Quat::from_rotation_x(0.),
            },
            GlobalTransform::identity(),
        ))
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("glb/AE86.glb#Scene0"));
        });

    // 记录场景实体ID
    let instance_id = scene_spawner.spawn(asset_server.load("glb/AE86.glb#Scene0"));
    scene_instance.0 = Some(instance_id);
}

/// 给场景中的实体添加标记
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

/// 根据场景中实体标记遍历实体，并修改状态值
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
