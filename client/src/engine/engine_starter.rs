use bevy::{
    prelude::*,
    wgpu::{WgpuBackend, WgpuOptions, WgpuPowerOptions},
    window::WindowMode,
    winit::WinitPlugin,
};
use bevy_rapier3d::na::{Point3, Vector3};
use bevy_rapier3d::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy)]
enum CarPartTag {
    Wheel,
    Frame,
}

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
        .add_plugin(RapierPhysicsPlugin::<&CarPartTag>::default())
        .add_plugin(RapierRenderPlugin)
        // 自由摄像机
        .add_plugin(bevy_flycam::PlayerPlugin)
        .insert_resource(bevy_flycam::MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 12.0,          // default: 12.0
        })
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
        .add_startup_system(setup_graphics.system())
        .add_startup_system(setup_physics.system())
        .add_startup_system(enable_physics_profiling.system())
        .add_system_to_stage(CoreStage::PostUpdate, display_events.system())
        .add_system(apply_forces.system())
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
            Transform::from_xyz(0.0, 10.0, 0.0),
            GlobalTransform::identity(),
        ))
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("glb/tile01.glb#Scene0"));
        });
    commands
        .spawn_bundle((
            Transform::from_xyz(0.0, 10.0, 10.0),
            GlobalTransform::identity(),
        ))
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("glb/tile01.glb#Scene0"));
        });
    commands
        .spawn_bundle((
            Transform::from_xyz(0.0, 10.0, -10.0),
            GlobalTransform::identity(),
        ))
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("glb/tile01.glb#Scene0"));
        });
    // 地面碰撞体
    let rigid_body = RigidBodyBundle {
        body_type: RigidBodyType::Static,
        position: [0.0, 0.0, 0.0].into(),
        ..RigidBodyBundle::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(10000.0, 0.5, 10000.0),
        ..ColliderBundle::default()
    };
    commands
        .spawn_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::with_id(4));

    // // 车头
    // let rigid_body = RigidBodyBundle {
    //     body_type: RigidBodyType::Dynamic,
    //     position: [0.0, 2.5, 0.0].into(),
    //     ..RigidBodyBundle::default()
    // };
    // let collider = ColliderBundle {
    //     shape: ColliderShape::cuboid(0.5, 0.5, 0.5),
    //     ..ColliderBundle::default()
    // };
    // let headstock_entity = commands
    //     .spawn_bundle(rigid_body)
    //     .insert_bundle(collider)
    //     .insert(ColliderPositionSync::Discrete)
    //     .insert(ColliderDebugRender::with_id(1))
    //     .insert(CarPartTag::Frame)
    //     .id();
    // // 车尾
    // let rigid_body = RigidBodyBundle {
    //     body_type: RigidBodyType::Dynamic,
    //     position: [0.0, 2.5, -6.0].into(),
    //     ..RigidBodyBundle::default()
    // };
    // let collider = ColliderBundle {
    //     shape: ColliderShape::cuboid(0.5, 0.5, 0.5),
    //     ..ColliderBundle::default()
    // };
    // let tailstock_entity = commands
    //     .spawn_bundle(rigid_body)
    //     .insert_bundle(collider)
    //     .insert(ColliderPositionSync::Discrete)
    //     .insert(ColliderDebugRender::with_id(1))
    //     .insert(CarPartTag::Frame)
    //     .id();
    // // 车轴链接
    // let joint = FixedJoint::new([0.0, 0.0, 0.0].into(), [0.0, 0.0, -6.0].into());
    // commands.spawn_bundle((JointBuilderComponent::new(
    //     joint,
    //     headstock_entity,
    //     tailstock_entity,
    // ),));
    // 底盘
    let rigid_body = RigidBodyBundle {
        body_type: RigidBodyType::Dynamic,
        position: [0.0, 10.25, -3.0].into(),
        ..RigidBodyBundle::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(1.0, 0.5, 3.0),
        ..ColliderBundle::default()
    };
    let body_entity = commands
        .spawn_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::with_id(1))
        .insert(CarPartTag::Frame)
        .id();
    // 车轮1
    let rigid_body = RigidBodyBundle {
        body_type: RigidBodyType::Dynamic,
        position: [-2.0, 10.0, 0.0].into(),
        ..RigidBodyBundle::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(1.0, 1.0, 1.0),
        ..ColliderBundle::default()
    };
    let left_entity = commands
        .spawn_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::with_id(1))
        .insert(CarPartTag::Wheel)
        .id();

    // 车轮2
    let rigid_body = RigidBodyBundle {
        body_type: RigidBodyType::Dynamic,
        position: [2.0, 10.0, 0.0].into(),
        ..RigidBodyBundle::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(1.0, 1.0, 1.0),
        ..ColliderBundle::default()
    };
    let right_entity = commands
        .spawn_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::with_id(1))
        .insert(CarPartTag::Wheel)
        .id();

    // 车轮3
    let rigid_body = RigidBodyBundle {
        body_type: RigidBodyType::Dynamic,
        position: [-2.0, 10.0, -6.0].into(),
        ..RigidBodyBundle::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(1.0, 1.0, 1.0),
        ..ColliderBundle::default()
    };
    let left_tail_entity = commands
        .spawn_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::with_id(1))
        // .insert(CarPartTag::Wheel)
        .id();

    // 车轮4
    let rigid_body = RigidBodyBundle {
        body_type: RigidBodyType::Dynamic,
        position: [2.0, 10.0, -6.0].into(),
        ..RigidBodyBundle::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(1.0, 1.0, 1.0),
        ..ColliderBundle::default()
    };
    let right_tail_entity = commands
        .spawn_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::with_id(1))
        // .insert(CarPartTag::Wheel)
        .id();

    let o = Point3::origin();
    let z = Vector3::x_axis();

    // 左头车轮链接
    let joint = RevoluteJoint::new(o, z, [-2.1, 0.0, 2.5].into(), z);
    commands.spawn_bundle((JointBuilderComponent::new(joint, body_entity, left_entity),));
    // 右头车轮链接
    let joint = RevoluteJoint::new(o, z, [2.1, 0.0, 2.5].into(), z);
    commands.spawn_bundle((JointBuilderComponent::new(joint, body_entity, right_entity),));
    // 左尾车轮链接
    let joint = RevoluteJoint::new(o, z, [-2.1, 0.0, -2.5].into(), z);
    commands.spawn_bundle((JointBuilderComponent::new(
        joint,
        body_entity,
        left_tail_entity,
    ),));
    // 右尾车轮链接
    let joint = RevoluteJoint::new(o, z, [2.1, 0.0, -2.5].into(), z);
    commands.spawn_bundle((JointBuilderComponent::new(
        joint,
        body_entity,
        right_tail_entity,
    ),));

    // 材质
    // .with_children(|parent| {
    //     parent.spawn_scene(asset_server.load("glb/AE86.glb#Scene0"));
    // });
}

fn apply_forces(
    mut rigid_bodies: Query<(
        &mut RigidBodyForces,
        &mut RigidBodyVelocity,
        &RigidBodyMassProps,
        &RigidBodyType,
        &CarPartTag,
    )>,
) {
    for (mut rb_forces, mut rb_vel, rb_mprops, rb_type, carPartTag) in rigid_bodies.iter_mut() {
        match carPartTag {
            CarPartTag::Wheel => match rb_type {
                RigidBodyType::Dynamic => {
                    // 力
                    // rb_forces.force = Vec3::new(1.0, 0.0, 0.0).into();
                    // rb_forces.torque = Vec3::new(0.2, 0.4, 0.8).into();

                    // 加速度
                    // rb_vel.apply_impulse(rb_mprops, Vec3::new(1.0, 0.0, 0.0).into());
                    // rb_vel.apply_torque_impulse(rb_mprops, Vec3::new(140.0, 80.0, 20.0).into());

                    // 直接设置速度
                    // rb_vel.linvel = [1.0, 0.0, 0.0].into();
                    rb_vel.angvel = [5.0, 0.0, 0.0].into();
                    // println!("{:?}", rb_vel.linvel);
                }
                _ => {}
            },
            CarPartTag::Frame => {}
        }
    }
}
