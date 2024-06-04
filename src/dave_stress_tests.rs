use std::f64::consts::PI;
use std::io;
use std::str::FromStr;
use bevy::{
	diagnostic::{
		DiagnosticsStore,
		FrameTimeDiagnosticsPlugin,
		LogDiagnosticsPlugin,
	},
	math::{
		DVec2,
		DVec3,
	},
	pbr::{
		ExtractedPointLight,
		GlobalLightMeta,
	},
	prelude::*,
	render::{
		camera::ScalingMode,
		Render,
		RenderApp,
		RenderSet,
		render_asset::RenderAssetUsages,
		render_resource::{
			Extent3d,
			TextureDimension,
			TextureFormat,
		},
	},
	sprite::{
		MaterialMesh2dBundle,
		Mesh2dHandle,
	},
	utils::Duration,
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
use rand::{
	rngs::StdRng,
	Rng,
	SeedableRng,
	seq::SliceRandom,
	thread_rng,
};
use crate::dave_graphics::ASSETS_DIR;

//
// Too Many Buttons!
//
const ARG_IMAGE_FREQ: usize = 4;
const ARG_BUTTONS: usize = 110;
const ARG_NO_BORDERS: bool = false;
const ARG_NO_TEXT: bool = false;
const ARG_RECOMPUTE_TEXT: bool = false;
const ARG_RELAYOUT: bool = false;
const FONT_SIZE: f32 = 7.0;

#[derive(Component)]
struct IdleColor(BackgroundColor);

fn button_system(
	key_input: Res<ButtonInput<KeyCode>>,
	mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &IdleColor), Changed<Interaction>>,
) {
	if key_input.pressed(KeyCode::Escape) {
		println!();
		std::process::exit(0)
	}
	for (interaction, mut button_color, IdleColor(idle_color)) in interaction_query.iter_mut() {
		*button_color = match interaction {
			Interaction::Hovered => Color::ORANGE_RED.into(),
			_ => *idle_color,
		};
	}
}

fn setup_flex(mut commands: Commands, asset_server: Res<AssetServer>) {
	let image = if 0 < ARG_IMAGE_FREQ {
		Some(asset_server.load(ASSETS_DIR.to_owned() + "/textures/bevy_logo_light.png"))
	} else {
		None
	};

	let buttons_f = ARG_BUTTONS as f32;
	let border = if ARG_NO_BORDERS {
		UiRect::ZERO
	} else {
		UiRect::all(Val::VMin(0.05 * 90. / buttons_f))
	};

	let as_rainbow = |i: usize| Color::hsl((i as f32 / buttons_f) * 360.0, 0.9, 0.8);
	commands.spawn(Camera2dBundle::default());
	commands.spawn(NodeBundle {
		style: Style {
			flex_direction: FlexDirection::Column,
			justify_content: JustifyContent::Center,
			align_items: AlignItems::Center,
			width: Val::Percent(100.),
			height: Val::Percent(100.),
			..default()
		},
		..default()
	})
	.with_children(|commands| {
		for column in 0..ARG_BUTTONS {
			commands
				.spawn(NodeBundle::default())
				.with_children(|commands| {
					for row in 0..ARG_BUTTONS {
						let color = as_rainbow(row & column.max(1)).into();
						let border_color = Color::WHITE.with_a(0.5).into();
						spawn_button(
							commands,
							color,
							buttons_f,
							column,
							row,
							ARG_NO_TEXT,
							border,
							border_color,
							image.as_ref().filter(|_| (column + row) & ARG_IMAGE_FREQ == 0).cloned()
						);
					}
				});
		}
	});
}

fn setup_grid(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image = if 0 < ARG_IMAGE_FREQ {
        Some(asset_server.load(
        	ASSETS_DIR.to_owned() + "/textures/bevy_logo_light.png",
        ))
    } else {
        None
    };

    let buttons_f = ARG_BUTTONS as f32;
    let border = if ARG_NO_BORDERS {
        UiRect::ZERO
    } else {
        UiRect::all(Val::VMin(0.05 * 90. / buttons_f))
    };

    let as_rainbow = |i: usize| Color::hsl((i as f32 / buttons_f) * 360.0, 0.9, 0.8);
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                width: Val::Percent(100.),
                height: Val::Percent(100.0),
                grid_template_columns: RepeatedGridTrack::flex(ARG_BUTTONS as u16, 1.0),
                grid_template_rows: RepeatedGridTrack::flex(ARG_BUTTONS as u16, 1.0),
                ..default()
            },
            ..default()
        })
        .with_children(|commands| {
            for column in 0..ARG_BUTTONS {
                for row in 0..ARG_BUTTONS {
                    let color = as_rainbow(row % column.max(1)).into();
                    let border_color = Color::WHITE.with_a(0.5).into();
                    spawn_button(
                        commands,
                        color,
                        buttons_f,
                        column,
                        row,
                        !ARG_NO_TEXT,
                        border,
                        border_color,
                        image
                            .as_ref()
                            .filter(|_| (column + row) % ARG_IMAGE_FREQ == 0)
                            .cloned(),
                    );
                }
            }
        });
}

#[allow(clippy::too_many_arguments)]
fn spawn_button(
	commands: &mut ChildBuilder,
	background_color: BackgroundColor,
	buttons: f32,
	column: usize,
	row: usize,
	spawn_text: bool,
	border: UiRect,
	border_color: BorderColor,
	image: Option<Handle<Image>>,
) {
	let width = Val::Vw(90.0 / buttons);
	let height = Val::Vh(90.0 / buttons);
	let margin = UiRect::axes(width * 0.05, height * 0.05);
	let mut builder = commands.spawn((
		ButtonBundle {
			style: Style {
				width,
				height,
				margin,
				align_items: AlignItems::Center,
				justify_content: JustifyContent::Center,
				border,
				..default()
			},
			background_color,
			border_color,
			..default()
		},
		IdleColor(background_color),
	));

	if let Some(image) = image {
		builder.insert(UiImage::new(image));
	}

	if spawn_text {
		builder.with_children(|parent| {
			parent.spawn(TextBundle::from_section(
				format!("{column}, {row}"),
				TextStyle {
					font_size: FONT_SIZE,
					color: Color::rgb(0.2, 0.2, 0.2),
					..default()
				},
			));
		});
	}
}

pub fn st_too_many_buttons_main(
	_num_buttons: &usize,
	_img_frq: &usize,
	grid: bool,
	_borders: bool,
	_text: bool,
) -> io::Result<()> {
	let mut app = App::new();

	app.add_plugins((
		DefaultPlugins.set(WindowPlugin {
			primary_window: Some(Window {
				present_mode: PresentMode::AutoNoVsync,
				resolution: WindowResolution::new(1920.0, 1080.0).with_scale_factor_override(1.0),
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
	.add_systems(Update, button_system);

	if grid {
		app.add_systems(Startup, setup_grid);
	} else {
		app.add_systems(Startup, setup_flex);
	}

	if ARG_RELAYOUT {
		app.add_systems(Update, |mut style_query: Query<&mut Style>| {
			style_query.iter_mut().for_each(|mut style| style.set_changed());
		});
	}

	if ARG_RECOMPUTE_TEXT {
		app.add_systems(Update, |mut text_query: Query<&mut Text>| {
			text_query.iter_mut().for_each(|mut text| text.set_changed());
		});
	}

	info!("Press ESCAPE to Quit");

	app.run();
	Ok(())
}

//
// Too Many Lights!
//
// NOTE: This epsilon value is apparently optimal for optimizing for the average
// nearest-neighbor distance. See:
// http://extremelearning.com.au/how-to-evenly-distribute-points-on-a-sphere-more-effectively-than-the-canonical-fibonacci-lattice/
const EPSILON: f64 = 0.36;

fn fibonacci_spiral_on_sphere(golden_ratio: f64, i: usize, n: usize) -> DVec2 {
	DVec2::new(
		PI * 2. * (i as f64 / golden_ratio),
		(1.0 - 2.0 * (i as f64 + EPSILON) / (n as f64 - 1.0 + 2.0 * EPSILON)).acos(),
	)
}

fn spherical_polar_to_cartesian(p: DVec2) -> DVec3 {
	let (sin_theta, cos_theta) = p.x.sin_cos();
	let (sin_phi, cos_phi) = p.y.sin_cos();
	DVec3::new(cos_theta * sin_phi, sin_theta * sin_phi, cos_phi)
}

// System for Rotating Camera
fn move_camera(
	time: Res<Time>,
	key_input: Res<ButtonInput<KeyCode>>,
	mut camera_query: Query<&mut Transform, With<Camera>>,
) {
	let mut camera_transform = camera_query.single_mut();
	let delta = time.delta_seconds() * 0.15;

	if key_input.pressed(KeyCode::Escape) {
	    println!();
	    std::process::exit(0)
	}

	camera_transform.rotate_z(delta);
	camera_transform.rotate_x(delta);
}

fn print_light_count(time: Res<Time>, mut timer: Local<PrintingTimer>, lights: Query<&PointLight>) {
	timer.0.tick(time.delta());

	if timer.0.just_finished() {
		info!("Lights: {}", lights.iter().len());
	}
}

struct LogVisibleLights;

impl Plugin for LogVisibleLights {
	fn build(&self, app: &mut App) {
		let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
			return;
		};

		render_app.add_systems(Render, print_visible_light_count.in_set(RenderSet::Prepare));
	}
}

fn print_visible_light_count(
	time: Res<Time>,
	mut timer: Local<PrintingTimer>,
	visible: Query<&ExtractedPointLight>,
	global_light_meta: Res<GlobalLightMeta>,
) {
	timer.0.tick(time.delta());

	if timer.0.just_finished() {
		info!(
			"Visible Lights: {}, Rendered Lights: {}",
			visible.iter().len(),
			global_light_meta.entity_to_index.len(),
		);
	}
}

struct PrintingTimer(Timer);

impl Default for PrintingTimer {
	fn default() -> Self {
		Self(Timer::from_seconds(1.0, TimerMode::Repeating))
	}
}

fn setup_lights(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	const LIGHT_RADIUS: f32 = 0.3;
	const LIGHT_INTENSITY: f32 = 1000.0;
	const RADIUS: f32 = 50.0;
	const N_LIGHTS: usize = 100_000;

	commands.spawn(PbrBundle {
		mesh: meshes.add(Sphere::new(RADIUS).mesh().ico(9).unwrap()),
		material: materials.add(Color::WHITE),
		transform: Transform::from_scale(Vec3::NEG_ONE),
		..default()
	});

	let mesh = meshes.add(Cuboid::default());
	let material = materials.add(StandardMaterial {
		base_color: Color::PINK,
		..default()
	});

	// NOTE: This pattern is good for testing performance of culling as it provides roughly
    // the same number of visible meshes regardless of the viewing angle.
    // NOTE: f64 is used to avoid precision issues that produce visual artifacts in the distribution
    let golden_ratio = 0.5f64 * (1.0f64 + 5.0f64.sqrt());
    let mut rng = thread_rng();

    for i in 0..N_LIGHTS {
    	let spherical_polar_theta_phi = fibonacci_spiral_on_sphere(golden_ratio, i, N_LIGHTS);
    	let unit_sphere_p = spherical_polar_to_cartesian(spherical_polar_theta_phi);
    	commands.spawn(PointLightBundle {
    		point_light: PointLight {
    			range: LIGHT_RADIUS,
    			intensity: LIGHT_INTENSITY,
    			color: Color::hsl(rng.gen_range(0.0..360.0), 1.0, 0.5),
    			..default()
    		},
    		transform: Transform::from_translation((RADIUS as f64 * unit_sphere_p).as_vec3()),
    		..default()
    	});
    }

    // Camera
    commands.spawn(Camera3dBundle {
    	projection: OrthographicProjection {
    		scale: 20.0,
    		scaling_mode: ScalingMode::FixedHorizontal(1.0),
    		..default()
    	}
    	.into(),
    	..default()
    });

    commands.spawn(PbrBundle {
    	mesh,
    	material,
    	transform: Transform {
    		translation: Vec3::new(0.0, RADIUS, 0.0),
    		scale: Vec3::splat(5.0),
    		..default()
    	},
    	..default()
    });

    info!("Press ESCAPE to Quit");
}

pub fn st_too_many_lights_main() -> io::Result<()> {
	App::new()
		.add_plugins((
			DefaultPlugins.set(WindowPlugin {
				primary_window: Some(Window {
					resolution: WindowResolution::new(1920.0, 1080.0).with_scale_factor_override(1.0),
					title: "Daves Many Lights!".into(),
					present_mode: PresentMode::AutoNoVsync,
					..default()
				}),
				..default()
			}),
			FrameTimeDiagnosticsPlugin,
			LogDiagnosticsPlugin::default(),
			LogVisibleLights,
		))
		.insert_resource(WinitSettings {
			focused_mode: UpdateMode::Continuous,
			unfocused_mode: UpdateMode::Continuous,
		})
		.add_systems(Startup, setup_lights)
		.add_systems(Update, (move_camera, print_light_count))
		.run();

	Ok(())
}

//
// DaveMark
//
const ENTITIES_PER_SECOND: u32 = 10000;
const GRAVITY: f32 = -9.8 * 100.0;
const MAX_VELOCITY: f32 = 750.;
const ENTITY_SCALE: f32 = 0.15;
const ENTITY_TEXTURE_SIZE: usize = 256;
const HALF_ENTITY_SIZE: f32 = ENTITY_TEXTURE_SIZE as f32 * ENTITY_SCALE * 0.5;

// Use Sprite or Mesh2D
const MODE: Mode = Mode::Sprite;
// Spawn All Waves Upfront
const BENCHMARK: bool = false;
// How Many Birds to Spawn Per Wave
const PER_WAVE: usize = 5;
// Number of Waves to Spawn
const WAVES: usize = 5;
// Whether to Vary the Material Data in Each Instance
const VARY_PER_INSTANCE: bool = false;
// Number of Different Textures From Which to Randomly Select
// the Material Color. 0 Means No Textures
const MATERIAL_TEXTURE_COUNT: usize = 1;
// Generate Z Values in Increasing Order Rather Than Randomly
const ORDERED_Z: bool = false;

const FIXED_TIMESTEP: f32 = 0.2;

#[derive(Resource)]
struct DaveCounter {
	pub count: usize,
	pub color: Color,
}

#[derive(Component)]
struct Entity {
	velocity: Vec3,
}

#[derive(Default, Clone)]
enum Mode {
	#[default]
	Sprite,
	Mesh2d,
}

impl FromStr for Mode {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"sprite" => Ok(Self::Sprite),
			"mesh2d" => Ok(Self::Mesh2d),
			_ => Err(format!(
				"Unknown Mode: '{s}', Valid Modes: 'Sprite', 'Mesh2d'",
			)),
		}
	}
}

#[derive(Resource)]
struct EntityScheduled {
	waves: usize,
	per_wave: usize,
}

fn scheduled_spawner(
	mut commands: Commands,
	windows: Query<&Window>,
	mut scheduled: ResMut<EntityScheduled>,
	mut counter: ResMut<DaveCounter>,
	mut entity_resources: ResMut<EntityResources>,
) {
	let window = windows.single();

	if scheduled.waves > 0 {
		spawn_entities(
			&mut commands,
			&window.resolution,
			&mut counter,
			scheduled.per_wave,
			&mut entity_resources,
			None,
			scheduled.waves - 1,
		);

		scheduled.waves -= 1;
	}
}

#[derive(Resource)]
struct EntityResources {
	textures: Vec<Handle<Image>>,
	materials: Vec<Handle<ColorMaterial>>,
	quad: Mesh2dHandle,
	color_rng: StdRng,
	material_rng: StdRng,
	velocity_rng: StdRng,
	transform_rng: StdRng,
}

#[derive(Component)]
struct StatsText;

#[allow(clippy::too_many_arguments)]
fn davemark_setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut meshes: ResMut<Assets<Mesh>>,
	material_assets: ResMut<Assets<ColorMaterial>>,
	images: ResMut<Assets<Image>>,
	windows: Query<&Window>,
	counter: ResMut<DaveCounter>,
) {
	let images = images.into_inner();
	let mut textures = Vec::with_capacity(MATERIAL_TEXTURE_COUNT.max(1));

	if matches!(MODE, Mode::Sprite) || MATERIAL_TEXTURE_COUNT > 0 {
		textures.push(asset_server.load(
			ASSETS_DIR.to_owned() + "/textures/frog.png",
		));
	}
	init_textures(&mut textures, images);

	let material_assets = material_assets.into_inner();
	let materials = init_materials(&textures, material_assets);

	let mut entity_resources = EntityResources {
		textures,
		materials,
		quad: meshes
			.add(Rectangle::from_size(Vec2::splat(ENTITY_TEXTURE_SIZE as f32)))
			.into(),
		color_rng: StdRng::seed_from_u64(42),
		material_rng: StdRng::seed_from_u64(42),
		velocity_rng: StdRng::seed_from_u64(42),
		transform_rng: StdRng::seed_from_u64(42),
	};

	let text_section = move |color, value: &str| {
		TextSection::new(
			value,
			TextStyle {
				font_size: 40.0,
				color,
				..default()
			},
		)
	};

	info!("Press ESCAPE to Quit");

	commands.spawn(Camera2dBundle::default());
	commands.spawn(NodeBundle {
			style: Style {
				position_type: PositionType::Absolute,
				padding: UiRect::all(Val::Px(5.0)),
				..default()
			},
			z_index: ZIndex::Global(i32::MAX),
			background_color: Color::BLACK.with_a(0.75).into(),
			..default()
		})
		.with_children(|c| {
			c.spawn((
				TextBundle::from_sections([
					text_section(Color::GREEN, "Entity Count: "),
					text_section(Color::CYAN, ""),
					text_section(Color::GREEN, "\nFPS (RAW): "),
					text_section(Color::CYAN, ""),
					text_section(Color::GREEN, "\nFPS (SMA): "),
					text_section(Color::CYAN, ""),
					text_section(Color::GREEN, "\nFPS (EMA): "),
					text_section(Color::CYAN, ""),
				]),
				StatsText,
			));
		});

	let mut scheduled = EntityScheduled {
		per_wave: PER_WAVE,
		waves: WAVES,
	};

	if BENCHMARK {
		let counter = counter.into_inner();
		for wave in (0..scheduled.waves).rev() {
			spawn_entities(
				&mut commands,
				&windows.single().resolution,
				counter,
				scheduled.per_wave,
				&mut entity_resources,
				Some(wave),
				wave,
			);
		}
		scheduled.waves = 0;
	}
	commands.insert_resource(entity_resources);
	commands.insert_resource(scheduled);
}

#[allow(clippy::too_many_arguments)]
fn mouse_handler(
	mut commands: Commands,
	time: Res<Time>,
	mouse_button_input: Res<ButtonInput<MouseButton>>,
	key_input: Res<ButtonInput<KeyCode>>,
	windows: Query<&Window>,
	entity_resources: ResMut<EntityResources>,
	mut counter: ResMut<DaveCounter>,
	mut rng: Local<Option<StdRng>>,
	mut wave: Local<usize>,
) {
	if rng.is_none() {
		*rng = Some(StdRng::seed_from_u64(42));
	}
	let rng = match rng.as_mut() {
		Some(r) => r,
		None => std::process::exit(1),
	};
	let window = windows.single();

	if key_input.pressed(KeyCode::Escape) {
		println!();
		std::process::exit(0)
	}

	if mouse_button_input.just_released(MouseButton::Left) {
		counter.color = Color::rgb_linear(rng.gen(), rng.gen(), rng.gen());
	}

	if mouse_button_input.pressed(MouseButton::Left) {
		let spawn_count = (ENTITIES_PER_SECOND as f64 * time.delta_seconds_f64()) as usize;
		spawn_entities(
			&mut commands,
			&window.resolution,
			&mut counter,
			spawn_count,
			entity_resources.into_inner(),
			None,
			*wave,
		);
		*wave += 1;
	}
}

fn entity_velocity_transform(
	half_extents: Vec2,
	mut translation: Vec3,
	velocity_rng: &mut StdRng,
	waves: Option<usize>,
	dt: f32,
) -> (Transform, Vec3) {
	let mut velocity = Vec3::new(MAX_VELOCITY * (velocity_rng.gen::<f32>() - 0.5), 0., 0.);

	if let Some(waves) = waves {
		for _ in 0..(waves * (FIXED_TIMESTEP / dt).round() as usize) {
			step_movement(&mut translation, &mut velocity, dt);
			handle_collision(half_extents, &translation, &mut velocity);
		}
	}
	(
		Transform::from_translation(translation).with_scale(Vec3::splat(ENTITY_SCALE)),
		velocity,
	)
}

const FIXED_DELTA_TIME: f32 = 1.0 / 60.0;

fn step_movement(translation: &mut Vec3, velocity: &mut Vec3, dt: f32) {
	translation.x += velocity.x * dt;
	translation.y += velocity.y * dt;
	velocity.y += GRAVITY * dt;
}

fn movement_system(time: Res<Time>, mut entity_query: Query<(&mut Entity, &mut Transform)>) {
	let dt = if BENCHMARK {
		FIXED_DELTA_TIME
	} else {
		time.delta_seconds()
	};
	for (mut entity, mut transform) in &mut entity_query {
		step_movement(&mut transform.translation, &mut entity.velocity, dt);
	}
}

fn handle_collision(half_extents: Vec2, translation: &Vec3, velocity: &mut Vec3) {
    if (velocity.x > 0. && translation.x + HALF_ENTITY_SIZE > half_extents.x)
        || (velocity.x <= 0. && translation.x - HALF_ENTITY_SIZE < -half_extents.x)
    {
        velocity.x = -velocity.x;
    }
    let velocity_y = velocity.y;
    if velocity_y < 0. && translation.y - HALF_ENTITY_SIZE < -half_extents.y {
        velocity.y = -velocity_y;
    }
    if translation.y + HALF_ENTITY_SIZE > half_extents.y && velocity_y > 0.0 {
        velocity.y = 0.0;
    }
}
fn collision_system(windows: Query<&Window>, mut bird_query: Query<(&mut Entity, &Transform)>) {
    let window = windows.single();
    let half_extents = 0.5 * Vec2::new(window.width(), window.height());

    for (mut bird, transform) in &mut bird_query {
        handle_collision(half_extents, &transform.translation, &mut bird.velocity);
    }
}

fn counter_system(
    diagnostics: Res<DiagnosticsStore>,
    counter: Res<DaveCounter>,
    mut query: Query<&mut Text, With<StatsText>>,
) {
    let mut text = query.single_mut();

    if counter.is_changed() {
        text.sections[1].value = counter.count.to_string();
    }

    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(raw) = fps.value() {
            text.sections[3].value = format!("{raw:.2}");
        }
        if let Some(sma) = fps.average() {
            text.sections[5].value = format!("{sma:.2}");
        }
        if let Some(ema) = fps.smoothed() {
            text.sections[7].value = format!("{ema:.2}");
        }
    };
}

fn init_textures(textures: &mut Vec<Handle<Image>>, images: &mut Assets<Image>) {
    let mut color_rng = StdRng::seed_from_u64(42);
    while textures.len() < MATERIAL_TEXTURE_COUNT {
        let pixel = [color_rng.gen(), color_rng.gen(), color_rng.gen(), 255];
        textures.push(images.add(Image::new_fill(
            Extent3d {
                width: ENTITY_TEXTURE_SIZE as u32,
                height: ENTITY_TEXTURE_SIZE as u32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            &pixel,
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::RENDER_WORLD,
        )));
    }
}

fn init_materials(
    textures: &[Handle<Image>],
    assets: &mut Assets<ColorMaterial>,
) -> Vec<Handle<ColorMaterial>> {
    let capacity = if VARY_PER_INSTANCE {
        PER_WAVE * WAVES
    } else {
        MATERIAL_TEXTURE_COUNT.max(WAVES)
    }
    .max(1);

    let mut materials = Vec::with_capacity(capacity);
    materials.push(assets.add(ColorMaterial {
        color: Color::WHITE,
        texture: textures.first().cloned(),
    }));

    let mut color_rng = StdRng::seed_from_u64(42);
    let mut texture_rng = StdRng::seed_from_u64(42);
    materials.extend(
        std::iter::repeat_with(|| {
            assets.add(ColorMaterial {
                color: Color::rgb_u8(color_rng.gen(), color_rng.gen(), color_rng.gen()),
                texture: textures.choose(&mut texture_rng).cloned(),
            })
        })
        .take(capacity - materials.len()),
    );

    materials
}

#[allow(clippy::too_many_arguments)]
fn spawn_entities(
	commands: &mut Commands,
	primary_window_resolution: &WindowResolution,
	counter: &mut DaveCounter,
	spawn_count: usize,
	entity_resources: &mut EntityResources,
	waves_to_simulate: Option<usize>,
	wave: usize,
) {
	let entity_x = (primary_window_resolution.width() / -2.) + HALF_ENTITY_SIZE;
	let entity_y = (primary_window_resolution.height() / 2.) - HALF_ENTITY_SIZE;

	let half_extents = 0.5 * Vec2::new(
		primary_window_resolution.width(),
		primary_window_resolution.height(),
	);

	let color = counter.color;
	let current_count = counter.count;

	match MODE {
        Mode::Sprite => {
            let batch = (0..spawn_count)
                .map(|count| {
                    let entity_z = if ORDERED_Z {
                        (current_count + count) as f32 * 0.00001
                    } else {
                        entity_resources.transform_rng.gen::<f32>()
                    };

                    let (transform, velocity) = entity_velocity_transform(
                        half_extents,
                        Vec3::new(entity_x, entity_y, entity_z),
                        &mut entity_resources.velocity_rng,
                        waves_to_simulate,
                        FIXED_DELTA_TIME,
                    );

                    let color = if VARY_PER_INSTANCE {
                        Color::rgb_linear(
                            entity_resources.color_rng.gen(),
                            entity_resources.color_rng.gen(),
                            entity_resources.color_rng.gen(),
                        )
                    } else {
                        color
                    };
                    (
                        SpriteBundle {
                            texture: entity_resources
                                .textures
                                .choose(&mut entity_resources.material_rng)
                                .unwrap()
                                .clone(),
                            transform,
                            sprite: Sprite { color, ..default() },
                            ..default()
                        },
                        Entity { velocity },
                    )
                })
                .collect::<Vec<_>>();
            commands.spawn_batch(batch);
        }
        Mode::Mesh2d => {
            let batch = (0..spawn_count)
                .map(|count| {
                    let bird_z = if ORDERED_Z {
                        (current_count + count) as f32 * 0.00001
                    } else {
                        entity_resources.transform_rng.gen::<f32>()
                    };

                    let (transform, velocity) = entity_velocity_transform(
                        half_extents,
                        Vec3::new(entity_x, entity_y, bird_z),
                        &mut entity_resources.velocity_rng,
                        waves_to_simulate,
                        FIXED_DELTA_TIME,
                    );

                    let material =
                        if VARY_PER_INSTANCE || MATERIAL_TEXTURE_COUNT > WAVES {
                            entity_resources
                                .materials
                                .choose(&mut entity_resources.material_rng)
                                .unwrap()
                                .clone()
                        } else {
                            entity_resources.materials[wave % entity_resources.materials.len()].clone()
                        };
                    (
                        MaterialMesh2dBundle {
                            mesh: entity_resources.quad.clone(),
                            material,
                            transform,
                            ..default()
                        },
                        Entity { velocity },
                    )
                })
                .collect::<Vec<_>>();
            commands.spawn_batch(batch);
        }
    }

    counter.count += spawn_count;
    counter.color = Color::rgb_linear(
        entity_resources.color_rng.gen(),
        entity_resources.color_rng.gen(),
        entity_resources.color_rng.gen(),
    );
}

pub fn davemark_main() -> io::Result<()> {
	App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "DaveMark".into(),
                    resolution: WindowResolution::new(1920.0, 1080.0)
                        .with_scale_factor_override(1.0),
                    present_mode: PresentMode::AutoNoVsync,
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
        .insert_resource(DaveCounter {
            count: 0,
            color: Color::WHITE,
        })
        .add_systems(Startup, davemark_setup)
        .add_systems(FixedUpdate, scheduled_spawner)
        .add_systems(
            Update,
            (
                mouse_handler,
                movement_system,
                collision_system,
                counter_system,
            ),
        )
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_secs_f32(
            FIXED_TIMESTEP,
        )))
        .run();

    Ok(())
}
