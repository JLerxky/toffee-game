use bevy::{
    prelude::*,
    wgpu::{WgpuBackend, WgpuOptions, WgpuPowerOptions},
    window::WindowMode,
    winit::WinitPlugin,
};
use bevy_rapier3d::prelude::*;

pub fn engine_start() {
    App::build()
        .insert_resource(WgpuOptions {
            backend: WgpuBackend::Auto,
            power_pref: WgpuPowerOptions::HighPerformance,
            features: bevy::wgpu::WgpuFeatures {
                features: vec![bevy::wgpu::WgpuFeature::NonFillPolygonMode],
            },
            ..Default::default()
        })
        .insert_resource(WindowDescriptor {
            title: String::from("初始游戏"),
            // 垂直同步
            vsync: true,
            // 是否可调整窗口大小
            resizable: false,
            // 是否有窗口外壳
            decorations: true,
            // width: 1366f32,
            // height: 768f32,
            width: 1920f32,
            height: 1080f32,
            // 窗口模式
            mode: WindowMode::Windowed,
            // 鼠标隐藏并锁定
            // cursor_locked: true,
            // cursor_visible: false,
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        // 窗口背景色
        // .insert_resource(ClearColor(Color::rgb_u8(192, 126, 104)))
        // 默认插件
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy::render::wireframe::WireframePlugin)
        // esc退出系统
        .add_system(bevy::input::system::exit_on_esc_system.system())
        // 窗口插件
        .add_plugin(WinitPlugin::default())
        // 物理插件
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        // 自由摄像机
        .add_plugin(bevy_flycam::PlayerPlugin)
        .insert_resource(bevy_flycam::MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 12.0,          // default: 12.0
        })
        // 日志输出
        // Adds frame time diagnostics
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        // Adds a system that prints diagnostics to the console
        .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        // Any plugin can register diagnostics
        // Uncomment this to add some render resource diagnostics:
        // .add_plugin(bevy::wgpu::diagnostic::WgpuResourceDiagnosticsPlugin::default())
        // Uncomment this to add an entity count diagnostics:
        // .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin::default())
        // Uncomment this to add an asset count diagnostics:
        // .add_plugin(bevy::asset::diagnostic::AssetCountDiagnosticsPlugin::<Texture>::default())
        // 测试
        .add_startup_system(setup_graphics.system())
        .add_startup_system(setup_physics.system())
        .add_startup_system(enable_physics_profiling.system())
        .add_system_to_stage(CoreStage::PostUpdate, display_events.system())
        .run();
}

fn enable_physics_profiling(mut pipeline: ResMut<PhysicsPipeline>) {
    pipeline.counters.enable()
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn().insert_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(100.0, 100.0, 200.0)),
        light: Light {
            intensity: 1000000.0,
            range: 30000.0,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn display_events(
    mut intersection_events: EventReader<IntersectionEvent>,
    mut contact_events: EventReader<ContactEvent>,
) {
    for intersection_event in intersection_events.iter() {
        println!("Received intersection event: {:?}", intersection_event);
    }

    for contact_event in contact_events.iter() {
        println!("Received contact event: {:?}", contact_event);
    }
}

pub fn setup_physics(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 天空球
    commands
        .spawn_bundle((
            Transform {
                translation: Vec3::new(0., 0., -10.),
                scale: Vec3::new(0.12, 0.12, 0.12),
                rotation: Quat::from_rotation_x(0.),
            },
            GlobalTransform::identity(),
        ))
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("glb/sky.glb#Scene0"));
        });

    // 地面
    commands
        .spawn_bundle((
            Transform::from_xyz(0.0, 8.0, 0.0),
            GlobalTransform::identity(),
        ))
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("glb/tile01.glb#Scene0"));
        });
    // 地面碰撞体
    let rigid_body = RigidBodyBundle {
        position: [0.0, 0.0, 0.0].into(),
        ..RigidBodyBundle::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(10.0, 0.0005, 10.0),
        ..ColliderBundle::default()
    };
    commands
        .spawn_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(ColliderDebugRender::with_id(2));

    // 车体
    let rigid_body = RigidBodyBundle {
        position: [0.0, 10.0, 0.0].into(),
        ..RigidBodyBundle::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::ball(1.0),
        ..ColliderBundle::default()
    };

    commands
        .spawn_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(ColliderDebugRender::with_id(1))
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("glb/AE86.glb#Scene0"));
        });
}
