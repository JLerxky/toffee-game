use bevy::{
    prelude::*,
    wgpu::{WgpuBackend, WgpuOptions, WgpuPowerOptions},
    window::WindowMode,
    winit::WinitPlugin,
};

pub fn engine_start() {
    App::build()
        .insert_resource(WgpuOptions {
            backend: WgpuBackend::Auto,
            power_pref: WgpuPowerOptions::HighPerformance,
            // features: WgpuFeatures {
            //     features: vec![WgpuFeature::NonFillPolygonMode],
            // },
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
            width: 1366f32,
            height: 768f32,
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
        // esc退出系统
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_plugin(WinitPlugin::default())
        // .add_plugin(WgpuPlugin::default())
        // 物理插件
        // .add_plugin(RapierPhysicsPlugin)
        // .add_plugin(RapierRenderPlugin)
        // 设置摄像机
        .add_startup_system(set_camera.system())
        // 日志输出
        // Adds frame time diagnostics
        // .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        // Adds a system that prints diagnostics to the console
        // .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        // Any plugin can register diagnostics
        // Uncomment this to add some render resource diagnostics:
        // .add_plugin(bevy::wgpu::diagnostic::WgpuResourceDiagnosticsPlugin::default())
        // Uncomment this to add an entity count diagnostics:
        // .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin::default())
        // Uncomment this to add an asset count diagnostics:
        // .add_plugin(bevy::asset::diagnostic::AssetCountDiagnosticsPlugin::<Texture>::default())
        // 测试
        .add_startup_system(setup.system())
        .run();
}

fn set_camera(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    commands.spawn_bundle(UiCameraBundle::default());
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });
    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}
