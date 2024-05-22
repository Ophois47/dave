use std::f32::consts::*;
use std::io;
use std::time::Duration;
use bevy::{
    animation::RepeatAnimation,
	app::MainScheduleOrder,
    asset::LoadState,
    diagnostic::{
        FrameTimeDiagnosticsPlugin,
        LogDiagnosticsPlugin,
    },
	ecs::schedule::*,
    pbr::{
        CascadeShadowConfigBuilder,
        DirectionalLightShadowMap,
        light_consts,
        NotShadowCaster,
    },
	prelude::*,
    render::{
        camera::{
            Exposure,
            PhysicalCameraParameters,
        },
        render_asset::RenderAssetUsages,
        render_resource::{
            Extent3d,
            TextureDimension,
            TextureFormat,
        },
    },
    window::{
        PresentMode,
        WindowPlugin,
        WindowResolution,
    },
    winit::{
        UpdateMode,
        WinitSettings,
    },
};
use crate::utils::*;

const ASSETS_DIR: &str = "/home/david/Documents/Programs/Rust/dave/dave_conf/var/daves_assets";

//
// Cube Program
//
fn daves_cube_setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	// Circular Base
	commands.spawn(PbrBundle {
		mesh: meshes.add(Circle::new(4.0)),
		material: materials.add(Color::WHITE),
		transform: Transform::from_rotation(
			Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
		),
		..default()
	});

	// Cube
	commands.spawn(PbrBundle {
		mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
		material: materials.add(Color::rgb_u8(124, 144, 255)),
		transform: Transform::from_xyz(0.0, 0.5, 0.0),
		..default()
	});

	// Light
	commands.spawn(PointLightBundle {
		point_light: PointLight {
			shadows_enabled: true,
			..default()
		},
		transform: Transform::from_xyz(4.0, 8.0, 4.0),
		..default()
	});

	// Camera
	commands.spawn((
        Camera3dBundle {
		  transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
	      ..default()
	   },
       CameraController::default(),
    ));
}

pub fn daves_cube_main() -> io::Result<()> {
	App::new()
		.add_plugins(DefaultPlugins)
        .add_plugins(CameraControllerPlugin)
		.add_systems(Startup, daves_cube_setup)
		.run();

	Ok(())
}

//
// Shapes Program
//
#[derive(Component)]
struct Shape;

const X_EXTENT: f32 = 12.0;

fn daves_shapes_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_shapes_texture())),
        ..default()
    });

    let shapes = [
        meshes.add(Cuboid::default()),
        meshes.add(Capsule3d::default()),
        meshes.add(Torus::default()),
        meshes.add(Cylinder::default()),
        meshes.add(Sphere::default().mesh().ico(5).unwrap()),
        meshes.add(Sphere::default().mesh().uv(32, 18)),
    ];

    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        commands.spawn((
            PbrBundle {
                mesh: shape,
                material: debug_material.clone(),
                transform: Transform::from_xyz(
                    -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                    2.0,
                    0.0,
                )
                .with_rotation(Quat::from_rotation_x(-PI / 4.)),
                ..default()
            },
            Shape,
        ));
    }

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    // Ground Plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
        material: materials.add(Color::SILVER),
        ..default()
    });

    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 6., 12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
            ..default()
        },
        CameraController::default(),
    ));
}

fn rotate_shapes(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() / 2.);
    }
}

fn uv_shapes_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 
        102, 255, 102, 255, 198, 255, 102, 198, 255, 255, 121, 102, 255, 255,
        236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}

pub fn daves_shapes_main() -> io::Result<()> {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(CameraControllerPlugin)
        .add_systems(Startup, daves_shapes_setup)
        .add_systems(Update, rotate_shapes)
        .run();

    Ok(())
}

//
// Colored Wiggle Bars!
//
#[derive(Resource)]
struct MorphData {
    the_wave: Handle<AnimationClip>,
    mesh: Handle<Mesh>,
}

fn name_morphs(
    mut has_printed: Local<bool>,
    morph_data: Res<MorphData>,
    meshes: Res<Assets<Mesh>>,
) {
    if *has_printed {
        return
    }

    let Some(mesh) = meshes.get(&morph_data.mesh) else {
        return
    };
    let Some(names) = mesh.morph_target_names() else {
        return
    };

    for name in names {
        println!("Target Name: {name}");
    }
    println!();
    *has_printed = true;
}

fn setup_animations(
    mut has_setup: Local<bool>,
    mut players: Query<(&Name, &mut AnimationPlayer)>,
    morph_data: Res<MorphData>,
) {
    if *has_setup {
        return
    }
    for (name, mut player) in &mut players {
        if name.as_str() != "Main" {
            continue;
        }
        player.play(morph_data.the_wave.clone()).repeat();
        *has_setup = true;
    }
}

fn morph_setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    // Animation
    commands.insert_resource(MorphData {
        the_wave: asset_server.load(
            ASSETS_DIR.to_owned() + "/models/morph/MorphStressTest.gltf#Animation2",
        ),
        mesh: asset_server.load(
            ASSETS_DIR.to_owned() + "/models/morph/MorphStressTest.gltf#Mesh0/Primitive0",
        ),
    });

    // Scene
    commands.spawn(SceneBundle {
        scene: asset_server.load(
            ASSETS_DIR.to_owned() + "/models/morph/MorphStressTest.gltf#Scene0",
        ),
        ..default()
    });

    // Lighting
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_rotation_z(PI / 2.0)),
        ..default()
    });

    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(3.0, 2.1, 10.2).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraController::default(),
    ));
}

pub fn daves_morph_main() -> io::Result<()> {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Morph Targets".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(CameraControllerPlugin)
        .insert_resource(AmbientLight {
            brightness: 150.0,
            ..default()
        })
        .add_systems(Startup, morph_setup)
        .add_systems(Update, (name_morphs, setup_animations))
        .run();

    Ok(())
}

//
// Physically Based Rendering
//
#[derive(Component)]
struct EnvironmentMapLabel;

fn environment_map_load_finish(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    environment_maps: Query<&EnvironmentMapLight>,
    label_query: Query<Entity, With<EnvironmentMapLabel>>,
) {
    if let Ok(environment_map) = environment_maps.get_single() {
        if asset_server.load_state(&environment_map.diffuse_map) == LoadState::Loaded
            && asset_server.load_state(&environment_map.specular_map) == LoadState::Loaded {
                if let Ok(label_entity) = label_query.get_single() {
                    commands.entity(label_entity).despawn();
                }
            }
    }
}

fn pbr_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let sphere_mesh = meshes.add(Sphere::new(0.45));
    // Add Entities To World
    for y in -2..=2 {
        for x in -5..=5 {
            let x01 = (x + 5) as f32 / 10.0;
            let y01 = (y + 2) as f32 / 4.0;
            // Sphere
            commands.spawn(PbrBundle {
                mesh: sphere_mesh.clone(),
                material: materials.add(StandardMaterial {
                    base_color: Color::hex("#ffd891").unwrap(),
                    // Vary Key PBR Params On Grid Of Spheres
                    metallic: y01,
                    perceptual_roughness: x01,
                    ..default()
                }),
                transform: Transform::from_xyz(x as f32, y as f32 + 0.5, 0.0),
                ..default()
            });
        }
    }

    // Unlit Sphere
    commands.spawn(PbrBundle {
        mesh: sphere_mesh,
        material: materials.add(StandardMaterial {
            base_color: Color::hex("#ffd891").unwrap(),
            unlit: true,
            ..default()
        }),
        transform: Transform::from_xyz(-5.0, -2.5, 0.0),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(50.0, 50.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
        directional_light: DirectionalLight {
            illuminance: 1_500.,
            ..default()
        },
        ..default()
    });

    // Labels
    commands.spawn(
        TextBundle::from_section(
            "Perceptual Roughness",
            TextStyle {
                font_size: 36.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(100.0),
            ..default()
        }),
    );

    commands.spawn(TextBundle {
        text: Text::from_section(
            "Metallic",
            TextStyle {
                font_size: 36.0,
                ..default()
            },
        ),
        style: Style {
            position_type: PositionType::Absolute,
            top: Val::Px(130.0),
            right: Val::ZERO,
            ..default()
        },
        transform: Transform {
            rotation: Quat::from_rotation_z(std::f32::consts::PI / 2.0),
            ..default()
        },
        ..default()
    });

    commands.spawn((
        TextBundle::from_section(
            "Loading Environment Map...",
            TextStyle {
                font_size: 36.0,
                color: Color::RED,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            right: Val::Px(20.0),
            ..default()
        }),
        EnvironmentMapLabel,
    ));

    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 8.0).looking_at(Vec3::default(), Vec3::Y),
            projection: OrthographicProjection {
                scale: 0.01,
                ..default()
            }
            .into(),
            ..default()
        },
        CameraController::default(),
        EnvironmentMapLight {
            diffuse_map: asset_server.load(
                ASSETS_DIR.to_owned() + "/env_maps/pisa_diffuse_rgb9e5_zstd.ktx2",
            ),
            specular_map: asset_server.load(
                ASSETS_DIR.to_owned() + "/env_maps/pisa_specular_rgb9e5_zstd.ktx2",
            ),
            intensity: 900.0,
        },
    ));
}

pub fn daves_pbr_main() -> io::Result<()> {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraControllerPlugin)
        .add_systems(Startup, pbr_setup)
        .add_systems(Update, environment_map_load_finish)
        .run();

    Ok(())
}

//
// Atmospheric Fog Program
//
fn toggle_system(keycode: Res<ButtonInput<KeyCode>>, mut fog: Query<&mut FogSettings>) {
    let mut fog_settings = fog.single_mut();

    if keycode.just_pressed(KeyCode::Space) {
        let a = fog_settings.color.a();
        fog_settings.color.set_a(1.0 - a);
    }

    if keycode.just_pressed(KeyCode::KeyX) {
        let a = fog_settings.directional_light_color.a();
        fog_settings.directional_light_color.set_a(0.5 - a);
    }
}

fn fog_setup_instructions(mut commands: Commands) {
    commands.spawn(
        TextBundle::from_section(
            "Press SPACE to Toggle Atmospheric Fog\nPress 'X' to Toggle Directional Light Fog Influence",
            TextStyle {
                font_size: 20.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }),
    );
}

fn fog_camera_setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-1.0, 0.1, 1.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        },
        CameraController::default(),
        FogSettings {
            color: Color::rgba(0.35, 0.48, 0.66, 1.0),
            directional_light_color: Color::rgba(1.0, 0.95, 0.85, 0.5),
            directional_light_exponent: 30.0,
            falloff: FogFalloff::from_visibility_colors(
                // World Unit Distance Where Objects Retain Visibility
                15.0,
                // Atmospheric Extinction Color After Light Is Lost
                // From Absorbtion by Atmospheric Particles
                Color::rgb(0.35, 0.5, 0.66),
                // Atmospheric Inscattering Color, Light Gained Due
                // To Scattering From The Sun
                Color::rgb(0.8, 0.844, 1.0),
            ),
        },
    ));
}

fn fog_setup_terrain_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Configure Properly Scaled Cascade Shadow Map For Scene
    // (Defaults Too Large)
    let cascade_shadow_config = CascadeShadowConfigBuilder {
        first_cascade_far_bound: 0.3,
        maximum_distance: 3.0,
        ..default()
    }
    .build();

    // Sun
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(0.98, 0.95, 0.82),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0)
            .looking_at(Vec3::new(-0.15, -0.05, 0.25), Vec3::Y),
        cascade_shadow_config,
        ..default()
    });

    // Terrain
    commands.spawn(SceneBundle {
        scene: asset_server.load(
            ASSETS_DIR.to_owned() + "/scenes/Mountains.gltf#Scene0",
        ),
        ..default()
    });

    // Sky
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(2.0, 1.0, 1.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("888888").unwrap(),
                unlit: true,
                cull_mode: None,
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(20.0)),
            ..default()
        },
        NotShadowCaster,
    ));
}

pub fn daves_atmo_fog_main() -> io::Result<()> {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraControllerPlugin)
        .add_systems(
            Startup,
            (fog_camera_setup, fog_setup_terrain_scene, fog_setup_instructions),
        )
        .add_systems(Update, toggle_system)
        .run();

    Ok(())
}

//
// Load and Render 3D Models
//
fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.elapsed_seconds() * PI / 5.0,
            -FRAC_PI_4,
        );
    }
}

fn render_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.7, 0.7, 1.0).looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
            ..default()
        },
        CameraController::default(),
        EnvironmentMapLight {
            diffuse_map: asset_server.load(
                ASSETS_DIR.to_owned() + "/env_maps/pisa_diffuse_rgb9e5_zstd.ktx2",
            ),
            specular_map: asset_server.load(
                ASSETS_DIR.to_owned() + "/env_maps/pisa_specular_rgb9e5_zstd.ktx2",
            ),
            intensity: 250.0,
        },
    ));

    // Lighting
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 1,
            maximum_distance: 1.6,
            ..default()
        }
        .into(),
        ..default()
    });

    // Scene
    commands.spawn(SceneBundle {
        scene: asset_server.load(
            ASSETS_DIR.to_owned() + "/models/flight_helmet/FlightHelmet.gltf#Scene0",
        ),
        ..default()
    });
}

pub fn daves_render_viewer_main() -> io::Result<()> {
    App::new()
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraControllerPlugin)
        .add_systems(Startup, render_setup)
        .add_systems(Update, animate_light_direction)
        .run();

    Ok(())
}

//
// 3D Animated Fox
//
#[derive(Resource)]
struct FoxAnimations(Vec<Handle<AnimationClip>>);

fn keyboard_fox_animation_control(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut animation_players: Query<&mut AnimationPlayer>,
    animations: Res<FoxAnimations>,
    mut current_animation: Local<usize>,
) {
    for mut player in &mut animation_players {
        if keyboard_input.just_pressed(KeyCode::Space) {
            if player.is_paused() {
                player.resume();
            } else {
                player.pause();
            }
        }

        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
            let speed = player.speed();
            player.set_speed(speed * 1.2);
        }

        if keyboard_input.just_pressed(KeyCode::ArrowDown) {
            let speed = player.speed();
            player.set_speed(speed * 0.8);
        }

        if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
            let elapsed = player.seek_time();
            player.seek_to(elapsed - 0.1);
        }

        if keyboard_input.just_pressed(KeyCode::ArrowRight) {
            let elapsed = player.seek_time();
            player.seek_to(elapsed + 0.1);
        }

        if keyboard_input.just_pressed(KeyCode::Enter) {
            *current_animation = (*current_animation + 1) % animations.0.len();
            player
                .play_with_transition(
                    animations.0[*current_animation].clone_weak(),
                    Duration::from_millis(250),
                )
                .repeat();
        }

        if keyboard_input.just_pressed(KeyCode::Digit1) {
            player.set_repeat(RepeatAnimation::Count(1));
            player.replay();
        }

        if keyboard_input.just_pressed(KeyCode::Digit3) {
            player.set_repeat(RepeatAnimation::Count(3));
            player.replay();
        }

        if keyboard_input.just_pressed(KeyCode::Digit5) {
            player.set_repeat(RepeatAnimation::Count(5));
            player.replay();
        }

        if keyboard_input.just_pressed(KeyCode::KeyL) {
            player.set_repeat(RepeatAnimation::Forever);
        }

        if keyboard_input.just_pressed(KeyCode::Escape) {
            std::process::exit(0)
        }
    }
}

// Begin Animations When Scene Has Loaded
fn setup_fox_scene_once_loaded(
    animations: Res<FoxAnimations>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in &mut players {
        player.play(animations.0[0].clone_weak()).repeat();
    }
}

fn fox_animation_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Insert Resource With Current Scene Info
    commands.insert_resource(FoxAnimations(vec![
        asset_server.load(ASSETS_DIR.to_owned() + "/models/fox/Fox.glb#Animation2"),
        asset_server.load(ASSETS_DIR.to_owned() + "/models/fox/Fox.glb#Animation1"),
        asset_server.load(ASSETS_DIR.to_owned() + "/models/fox/Fox.glb#Animation0"),
    ]));

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(100.0, 100.0, 150.0)
            .looking_at(Vec3::new(0.0, 20.0, 0.0), Vec3::Y),
        ..default()
    });

    // Plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(500000.0, 500000.0)),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
        ..default()
    });

    // Light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI/ 4.)),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 200.0,
            maximum_distance: 400.0,
            ..default()
        }
        .into(),
        ..default()
    });

    // Fox
    commands.spawn(SceneBundle {
        scene: asset_server.load(ASSETS_DIR.to_owned() + "/models/fox/Fox.glb#Scene0"),
        ..default()
    });

    println!("##==> Fox Controls:");
    println!("  - SPACE: Play / Pause");
    println!("  - Arrow Up / Arrow Down: Speed Up / Slow Down Animation Playback");
    println!("  - Arrow Left / Arrow Right: Seek Backward / Seek Forward");
    println!("  - L: Loop Animation Indefinitely");
    println!("  - ENTER: Change Animation");
    println!("  - Esc: Quit")
}

pub fn daves_animated_fox_main() -> io::Result<()> {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 2000.,
        })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, fox_animation_setup)
        .add_systems(
            Update,
            (setup_fox_scene_once_loaded, keyboard_fox_animation_control),
        )
        .run();

    Ok(())
}

//
// 3D Animated Foxes
//
#[derive(Resource)]
struct Foxes {
    count: usize,
    speed: f32,
    moving: bool,
    sync: bool,
}

const RING_SPACING: f32 = 2.0;
const FOX_SPACING: f32 = 2.0;

#[derive(Component, Clone, Copy)]
enum RotationDirection {
    CounterClockwise,
    Clockwise,
}

impl RotationDirection {
    fn sign(&self) -> f32 {
        match self {
            RotationDirection::CounterClockwise => 1.0,
            RotationDirection::Clockwise => -1.0,
        }
    }
}

#[derive(Component)]
struct Ring {
    radius: f32,
}

fn keyboard_foxes_animation_control(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut animation_player: Query<&mut AnimationPlayer>,
    animations: Res<FoxAnimations>,
    mut current_animation: Local<usize>,
    mut foxes: ResMut<Foxes>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        foxes.moving = !foxes.moving;
    }

    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        foxes.speed *= 1.25;
    }

    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        foxes.speed *= 0.8;
    }

    if keyboard_input.just_pressed(KeyCode::Enter) {
        *current_animation = (*current_animation + 1) % animations.0.len();
    }

    for mut player in &mut animation_player {
        if keyboard_input.just_pressed(KeyCode::Space) {
            if player.is_paused() {
                player.resume();
            } else {
                player.pause();
            }
        }

        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
            let speed = player.speed();
            player.set_speed(speed * 1.25);
        }

        if keyboard_input.just_pressed(KeyCode::ArrowDown) {
            let speed = player.speed();
            player.set_speed(speed * 0.8);
        }

        if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
            let elapsed = player.seek_time();
            player.seek_to(elapsed - 0.1);
        }

        if keyboard_input.just_pressed(KeyCode::ArrowRight) {
            let elapsed = player.seek_time();
            player.seek_to(elapsed + 0.1);
        }

        if keyboard_input.just_pressed(KeyCode::Enter) {
            player
                .play_with_transition(
                    animations.0[*current_animation].clone_weak(),
                    Duration::from_millis(250),
                )
                .repeat();
        }

        if keyboard_input.just_pressed(KeyCode::Escape) {
            std::process::exit(0)
        }
    }
}

fn update_fox_rings(
    time: Res<Time>,
    foxes: Res<Foxes>,
    mut rings: Query<(&Ring, &RotationDirection, &mut Transform)>,
) {
    if !foxes.moving {
        return
    }

    let dt = time.delta_seconds();
    for (ring, rotation_direction, mut transform) in &mut rings {
        let angular_velocity = foxes.speed / ring.radius;
        transform.rotate_y(rotation_direction.sign() * angular_velocity * dt);
    }
}

fn setup_foxes_scene_once_loaded(
    animations: Res<FoxAnimations>,
    foxes: Res<Foxes>,
    mut player: Query<(Entity, &mut AnimationPlayer)>,
    mut done: Local<bool>,
) {
    if !*done && player.iter().len() == foxes.count {
        for (entity, mut player) in &mut player {
            player.play(animations.0[0].clone_weak()).repeat();
            if !foxes.sync {
                player.seek_to(entity.index() as f32 / 10.0);
            }
        }
        *done = true;
    }
}

fn foxes_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    foxes: Res<Foxes>,
) {
    // Insert Resource With Current Scene Information
    commands.insert_resource(FoxAnimations(vec![
        asset_server.load(ASSETS_DIR.to_owned() + "/models/fox/Fox.glb#Animation2"),
        asset_server.load(ASSETS_DIR.to_owned() + "/models/fox/Fox.glb#Animation1"),
        asset_server.load(ASSETS_DIR.to_owned() + "/models/fox/Fox.glb#Animation0"),
    ]));

    // Concentric Ring of Foxes, Running in Opposite Directions
    let fox_handle = asset_server.load(ASSETS_DIR.to_owned() + "/models/fox/Fox.glb#Scene0");
    let ring_directions = [
        (
            Quat::from_rotation_y(PI),
            RotationDirection::CounterClockwise,
        ),
        (Quat::IDENTITY, RotationDirection::Clockwise),
    ];

    let mut ring_index = 0;
    let mut radius = RING_SPACING;
    let mut foxes_remaining = foxes.count;

    info!("Spawning {} Foxes ...", foxes.count);

    while foxes_remaining > 0 {
        let (base_rotation, ring_direction) = ring_directions[ring_index % 2];
        let ring_parent = commands.spawn((
            SpatialBundle::INHERITED_IDENTITY,
            ring_direction,
            Ring { radius },
        ))
        .id();

        let circumference = PI * 2. * radius;
        let foxes_in_ring = ((circumference / FOX_SPACING) as usize).min(foxes_remaining);
        let fox_spacing_angle = circumference / (foxes_in_ring as f32 * radius);

        for fox_i in 0..foxes_in_ring {
            let fox_angle = fox_i as f32 * fox_spacing_angle;
            let (s, c) = fox_angle.sin_cos();
            let (x, z) = (radius * c, radius * s);

            commands.entity(ring_parent).with_children(|builder| {
                builder.spawn(SceneBundle {
                    scene: fox_handle.clone(),
                    transform: Transform::from_xyz(x, 0.0, z)
                        .with_scale(Vec3::splat(0.01))
                        .with_rotation(base_rotation * Quat::from_rotation_y(-fox_angle)),
                    ..default()
                });
            });
        }

        foxes_remaining -= foxes_in_ring;
        radius += RING_SPACING;
        ring_index += 1;
    }

    // Camera
    let zoom = 0.8;
    let translation = Vec3::new(
        radius * 1.25 * zoom,
        radius * 0.5 * zoom,
        radius * 1.5 * zoom,
    );
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(translation)
                .looking_at(0.2 * Vec3::new(translation.x, 0.0, translation.z), Vec3::Y),
            ..default()
        },
        CameraController::default(),
    ));

    // Plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(5000.0, 5000.0)),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
        ..default()
    });

    // Light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI / 4.)),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 0.9 * radius,
            maximum_distance: 2.8 * radius,
            ..default()
        }
        .into(),
        ..default()
    });

    println!("##==> Foxes Controls:");
    println!("  - SPACE : Play / Pause");
    println!("  - Arrow Up / Arrow Down : Speed Up / Slow Down Animation Playback");
    println!("  - Arrow Left / Arrow Right : Seek Backward / Seek Forward");
    println!("  - ENTER : Change Animation");
    println!("  - Esc : Quit")
}

pub fn daves_animated_foxes_main() -> io::Result<()> {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "🦊🦊🦊 Many Foxes! 🦊🦊🦊".into(),
                    present_mode: PresentMode::AutoNoVsync,
                    resolution: WindowResolution::new(1920.0, 1080.0)
                        .with_scale_factor_override(1.0),
                    ..default()
                }),
                ..default()
            }),
            FrameTimeDiagnosticsPlugin,
            LogDiagnosticsPlugin::default(),
        ))
        .insert_resource(WinitSettings {
            focused_mode: UpdateMode::Continuous,
            unfocused_mode: UpdateMode::Continuous,
        })
        .insert_resource(Foxes {
            count: 1000,
            speed: 2.0,
            moving: true,
            sync: true,
        })
        .add_systems(Startup, foxes_setup)
        .add_systems(
            Update,
            (
                setup_foxes_scene_once_loaded,
                keyboard_foxes_animation_control,
                update_fox_rings.after(keyboard_foxes_animation_control),
            ),
        )
        .run();

    Ok(())
}

//
// 3D Rendering + Lighting
//
#[derive(Resource, Default, Deref, DerefMut)]
struct Parameters(PhysicalCameraParameters);

#[derive(Component)]
struct Movable;

fn movement(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Movable>>,
) {
    for mut transform in &mut query {
        let mut direction = Vec3::ZERO;
        if input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }
        if input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }

        transform.translation += time.delta_seconds() * 2.0 * direction;
    }
}

fn update_exposure(
    key_input: Res<ButtonInput<KeyCode>>,
    mut parameters: ResMut<Parameters>,
    mut exposure: Query<&mut Exposure>,
    mut text: Query<&mut Text>,
) {
    let mut text = text.single_mut();
    if key_input.just_pressed(KeyCode::Digit2) {
        parameters.aperture_f_stops *= 2.0;
    } else if key_input.just_pressed(KeyCode::Digit1) {
        parameters.aperture_f_stops *= 0.5;
    }
    if key_input.just_pressed(KeyCode::Digit4) {
        parameters.shutter_speed_s *= 2.0;
    } else if key_input.just_pressed(KeyCode::Digit3) {
        parameters.shutter_speed_s *= 0.5;
    }
    if key_input.just_pressed(KeyCode::Digit6) {
        parameters.sensitivity_iso += 100.0;
    } else if key_input.just_pressed(KeyCode::Digit5) {
        parameters.sensitivity_iso -= 100.0;
    }
    if key_input.just_pressed(KeyCode::KeyR) {
        *parameters = Parameters::default();
    }

    text.sections[0].value = format!("Aperture: f/{:.0}\n", parameters.aperture_f_stops);
    text.sections[1].value = format!(
        "Shutter Speed: 1/{:.0}s\n",
        1.0 / parameters.shutter_speed_s,
    );
    text.sections[2].value = format!("Sensitivity: ISO {:.0}\n", parameters.sensitivity_iso);

    *exposure.single_mut() = Exposure::from_physical_camera(**parameters);
}

fn lighting_setup(
    parameters: Res<Parameters>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Ground Plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(10.0, 10.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 1.0,
            ..default()
        }),
        ..default()
    });

    // Left Wall
    let mut transform = Transform::from_xyz(2.5, 2.5, 0.0);
    transform.rotate_z(PI / 2.);
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(5.0, 0.15, 5.0)),
        transform,
        material: materials.add(StandardMaterial {
            base_color: Color::INDIGO,
            perceptual_roughness: 1.0,
            ..default()
        }),
        ..default()
    });

    // Back Right Wall
    let mut transform = Transform::from_xyz(0.0, 2.5, -2.5);
    transform.rotate_x(PI / 2.);
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(5.0, 0.15, 5.0)),
        transform,
        material: materials.add(StandardMaterial {
            base_color: Color::INDIGO,
            perceptual_roughness: 1.0,
            ..default()
        }),
        ..default()
    });

    // Bevy Logo to Demonstrate Alpha Mask Shadows
    let mut transform = Transform::from_xyz(-2.2, 0.5, 1.0);
    transform.rotate_y(PI / 8.);
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Rectangle::new(2.0, 0.5)),
            transform,
            material: materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load(
                    ASSETS_DIR.to_owned() + "/textures/bevy_logo_light.png",
                )),
                perceptual_roughness: 1.0,
                alpha_mode: AlphaMode::Mask(0.5),
                cull_mode: None,
                ..default()
            }),
            ..default()
        },
        Movable,
    ));

    // Cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: materials.add(StandardMaterial {
                base_color: Color::PINK,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Movable,
    ));

    // Sphere
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(0.5).mesh().uv(32, 18)),
            material: materials.add(StandardMaterial {
                base_color: Color::LIME_GREEN,
                ..default()
            }),
            transform: Transform::from_xyz(1.5, 1.0, 1.5),
            ..default()
        },
        Movable,
    ));

    // Ambient Light
    commands.insert_resource(AmbientLight {
        color: Color::ORANGE_RED,
        brightness: 0.02,
    });

    // Red Point Light
    commands
        .spawn(PointLightBundle {
            // transform: Transform::from_xyz(5.0, 8.0, 2.0),
            transform: Transform::from_xyz(1.0, 2.0, 0.0),
            point_light: PointLight {
                intensity: 100_000.0,
                color: Color::RED,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(PbrBundle {
                mesh: meshes.add(Sphere::new(0.1).mesh().uv(32, 18)),
                material: materials.add(StandardMaterial {
                    base_color: Color::RED,
                    emissive: Color::rgba_linear(7.13, 0.0, 0.0, 0.0),
                    ..default()
                }),
                ..default()
            });
        });

    // Green Spot Light
    commands
        .spawn(SpotLightBundle {
            transform: Transform::from_xyz(-1.0, 2.0, 0.0)
                .looking_at(Vec3::new(-1.0, 0.0, 0.0), Vec3::Z),
            spot_light: SpotLight {
                intensity: 100_000.0,
                color: Color::GREEN,
                shadows_enabled: true,
                inner_angle: 0.6,
                outer_angle: 0.8,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(PbrBundle {
                transform: Transform::from_rotation(Quat::from_rotation_x(PI / 2.0)),
                mesh: meshes.add(Capsule3d::new(0.1, 0.125)),
                material: materials.add(StandardMaterial {
                    base_color: Color::GREEN,
                    emissive: Color::rgba_linear(0.0, 7.13, 0.0, 0.0),
                    ..default()
                }),
                ..default()
            });
        });

    // Blue Point Light
    commands
        .spawn(PointLightBundle {
            // transform: Transform::from_xyz(5.0, 8.0, 2.0),
            transform: Transform::from_xyz(0.0, 4.0, 0.0),
            point_light: PointLight {
                intensity: 100_000.0,
                color: Color::BLUE,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(PbrBundle {
                mesh: meshes.add(Sphere::new(0.1).mesh().uv(32, 18)),
                material: materials.add(StandardMaterial {
                    base_color: Color::BLUE,
                    emissive: Color::rgba_linear(0.0, 0.0, 7.13, 0.0),
                    ..default()
                }),
                ..default()
            });
        });

    // Directional 'Sun' Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    });

    // Example Instructions
    let style = TextStyle {
        font_size: 20.0,
        ..default()
    };

    commands.spawn(
        TextBundle::from_sections(vec![
            TextSection::new(
                format!("Aperture: f/{:.0}\n", parameters.aperture_f_stops),
                style.clone(),
            ),
            TextSection::new(
                format!(
                    "Shutter Speed: 1/{:.0}s\n",
                    1.0 / parameters.shutter_speed_s,
                ),
                style.clone(),
            ),
            TextSection::new("\n\n", style.clone()),
            TextSection::new("Controls\n", style.clone()),
            TextSection::new("------------------\n", style.clone()),
            TextSection::new("Arrow Keys - Move Objects\n", style.clone()),
            TextSection::new("1/2 - Decrease/Increase Aperture\n", style.clone()),
            TextSection::new("3/4 - Decrease/Increase Shutter Speed\n", style.clone()),
            TextSection::new("5/6 - Decrease/Increase Sensitivity\n", style.clone()),
            TextSection::new("R - Reset Exposure\n", style),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }),
    );

    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            exposure: Exposure::from_physical_camera(**parameters),
            ..default()
        },
        CameraController::default(),
    ));
}

pub fn daves_lights_main() -> io::Result<()> {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraControllerPlugin)
        .insert_resource(Parameters(PhysicalCameraParameters {
            aperture_f_stops: 1.0,
            shutter_speed_s: 1.0 / 125.0,
            sensitivity_iso: 100.0,
        }))
        .add_systems(Startup, lighting_setup)
        .add_systems(Update, (update_exposure, movement, animate_light_direction))
        .run();

    Ok(())
}

//
// Stepping Program
//
#[derive(Debug, Hash, PartialEq, Eq, Clone, ScheduleLabel)]
struct DebugSchedule;

#[derive(Default)]
pub struct SteppingPlugin {
    schedule_labels: Vec<InternedScheduleLabel>,
    top: Val,
    left: Val,
}

impl SteppingPlugin {
    pub fn add_schedule(mut self, label: impl ScheduleLabel) -> SteppingPlugin {
        self.schedule_labels.push(label.intern());
        self
    }

    pub fn at(self, left: Val, top: Val) -> SteppingPlugin {
        SteppingPlugin { top, left, ..self }
    }
}

impl Plugin for SteppingPlugin {
    fn build(&self, app: &mut App) {
        app.init_schedule(DebugSchedule);
        let mut order = app.world.resource_mut::<MainScheduleOrder>();
        order.insert_after(Update, DebugSchedule);

        let mut stepping = Stepping::new();
        for label in &self.schedule_labels {
            stepping.add_schedule(*label);
        }
        app.insert_resource(stepping);
        app.insert_resource(State {
            ui_top: self.top,
            ui_left: self.left,
            systems: Vec::new(),
        })
        .add_systems(Startup, build_help)
        .add_systems(
            DebugSchedule,
            (
                build_ui.run_if(not(initialized)),
                handle_input,
                update_ui.run_if(initialized),
            )
                .chain(),
        );
    }
}

#[derive(Resource, Debug)]
struct State {
    systems: Vec<(InternedScheduleLabel, NodeId, usize)>,
    ui_top: Val,
    ui_left: Val,
}

fn initialized(state: Res<State>) -> bool {
    !state.systems.is_empty()
}

const FONT_SIZE: f32 = 20.0;
const FONT_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);
const FONT_BOLD: &str = "fonts/FiraSans-Bold.ttf";
const FONT_MEDIUM: &str = "fonts/FiraMono-Medium.ttf";

#[derive(Component)]
struct SteppingUi;

fn build_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    schedules: Res<Schedules>,
    mut stepping: ResMut<Stepping>,
    mut state: ResMut<State>,
) {
    let mut text_sections = Vec::new();
    let mut always_run = Vec::new();

    let Ok(schedule_order) = stepping.schedules() else {
        return;
    };

    for label in schedule_order {
        let schedule = schedules.get(*label).unwrap();
        text_sections.push(TextSection::new(
            format!("{:?}\n", label),
            TextStyle {
                font: asset_server.load(FONT_BOLD),
                font_size: FONT_SIZE,
                color: FONT_COLOR,
            },
        ));

        let Ok(systems) = schedule.systems() else {
            return;
        };

        for (node_id, system) in systems {
            if system.name().starts_with("bevy") {
                always_run.push((*label, node_id));
                continue;
            }

            state.systems.push((*label, node_id, text_sections.len()));
            text_sections.push(TextSection::new(
                "   ",
                TextStyle {
                    font: asset_server.load(FONT_MEDIUM),
                    font_size: FONT_SIZE,
                    color: FONT_COLOR,
                },
            ));

            text_sections.push(TextSection::new(
                format!("{}\n", system.name()),
                TextStyle {
                    font: asset_server.load(FONT_MEDIUM),
                    font_size: FONT_SIZE,
                    color: FONT_COLOR,
                },
            ));
        }
    }

    for (label, node) in always_run.drain(..) {
        stepping.always_run_node(label, node);
    }

    commands.spawn((
        SteppingUi,
        TextBundle {
            text: Text::from_sections(text_sections),
            style: Style {
                position_type: PositionType::Absolute,
                top: state.ui_top,
                left: state.ui_left,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::rgba(1.0, 1.0, 1.0, 0.33)),
            visibility: Visibility::Hidden,
            ..default()
        },
    ));
}

fn build_help(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((TextBundle::from_sections([TextSection::new(
        "Press ` to toggle stepping mode (S: step system, Space: step frame)",
        TextStyle {
            font: asset_server.load(FONT_MEDIUM),
            font_size: 18.0,
            color: FONT_COLOR,
        },
    )])
    .with_style(Style {
        position_type: PositionType::Absolute,
        bottom: Val::Px(5.0),
        left: Val::Px(5.0),
        ..default()
    }),));
}

fn handle_input(keyboard_input: Res<ButtonInput<KeyCode>>, mut stepping: ResMut<Stepping>) {
    if keyboard_input.just_pressed(KeyCode::Slash) {
        info!("{:#?}", stepping);
    }
    if keyboard_input.just_pressed(KeyCode::Backquote) {
        if stepping.is_enabled() {
            stepping.disable();
            debug!("Disabled Stepping");
        } else {
            stepping.enable();
            debug!("Enabled Stepping");
        }
    }

    if !stepping.is_enabled() {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        debug!("Continue");
        stepping.continue_frame();
    } else if keyboard_input.just_pressed(KeyCode::KeyS) {
        debug!("Stepping Frame");
        stepping.step_frame();
    }
}

fn update_ui(
    mut commands: Commands,
    state: Res<State>,
    stepping: Res<Stepping>,
    mut ui: Query<(Entity, &mut Text, &Visibility), With<SteppingUi>>,
) {
    if ui.is_empty() {
        return;
    }

    let (ui, mut text, vis) = ui.single_mut();
    match (vis, stepping.is_enabled()) {
        (Visibility::Hidden, true) => {
            commands.entity(ui).insert(Visibility::Inherited);
        }
        (Visibility::Hidden, false) | (_, true) => (),
        (_, false) => {
            commands.entity(ui).insert(Visibility::Hidden);
        }
    }

    if !stepping.is_enabled() {
        return;
    }

    let (cursor_schedule, cursor_system) = match stepping.cursor() {
        None => return,
        Some(c) => c,
    };

    for (schedule, system, text_index) in &state.systems {
        let mark = if &cursor_schedule == schedule && *system == cursor_system {
            "-> "
        } else {
            "   "
        };
        text.sections[*text_index].value = mark.to_string();
    }
}
