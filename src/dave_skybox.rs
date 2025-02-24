use std::f32::consts::PI;
use std::io;
use std::path::Path;
use bevy::{
	asset::LoadState,
	core_pipeline::Skybox,
	diagnostic::{
		DiagnosticsStore,
		FrameTimeDiagnosticsPlugin,
	},
	prelude::*,
	render::{
		render_resource::{
			TextureViewDescriptor,
			TextureViewDimension,
		},
		renderer::RenderDevice,
		texture::CompressedImageFormats,
	},
};
use crate::utils::*;

const CUBEMAPS_DIR: &str = "./dave_conf/etc/daves_assets";
const CUBEMAPS: &[(&str, CompressedImageFormats)] = &[
	(
		"cube_maps/Ryfjallet_cubemap.png",
		CompressedImageFormats::NONE,
	),
	(
		"cube_maps/Ryfjallet_cubemap_astc4x4.ktx2",
		CompressedImageFormats::ASTC_LDR,
	),
	(
		"cube_maps/Ryfjallet_cubemap_bc7.ktx2",
		CompressedImageFormats::BC,
	),
	(
		"cube_maps/Ryfjallet_cubemap_etc2.ktx2",
		CompressedImageFormats::ETC2,
	),
];

#[derive(Resource)]
struct Cubemap {
	is_loaded: bool,
	index: usize,
	image_handle: Handle<Image>,
}

#[derive(Component)]
struct FPSText;

#[derive(Component)]
struct FormatText;

#[allow(unexpected_cfgs)]
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	// Direction Light
	commands.spawn(DirectionalLightBundle {
		directional_light: DirectionalLight {
			illuminance: 32000.0,
			..default()
		},
		transform: Transform::from_xyz(0.0, 2.0, 0.0)
			.with_rotation(Quat::from_rotation_x(-PI / 4.)),
		..default()
	});

	let skybox_handle = asset_server.load(CUBEMAPS[0].0);
	// Camera
	commands.spawn((
		Camera3dBundle {
			transform: Transform::from_xyz(0.0, 0.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
			..default()
		},
		CameraController::default(),
		Skybox {
			image: skybox_handle.clone(),
			brightness: 1000.0,
		},
	));

	// Text With Multiple Sections
	commands.spawn((
		TextBundle::from_sections([
			TextSection::new(
				"FPS: ",
				TextStyle {
					font: asset_server.load(
						CUBEMAPS_DIR.to_owned() + "/fonts/FiraSans-Bold.ttf",
					),
					font_size: 20.0,
					color: Color::BLACK,
					..default()
				},
			),
			TextSection::from_style(if cfg!(feature = "default_font") {
				TextStyle {
					font_size: 20.0,
					color: Color::GOLD,
					..default()
				}
			} else {
				TextStyle {
					font: asset_server.load(
						CUBEMAPS_DIR.to_owned() + "/fonts/FiraMono-Medium.ttf",
					),
					font_size: 20.0,
					color: Color::GOLD,
				}
			}),
			TextSection::new(
				"\nFormat: ",
				TextStyle {
					font: asset_server.load(
						CUBEMAPS_DIR.to_owned() + "/fonts/FiraMono-Medium.ttf",
					),
					font_size: 30.0,
					color: Color::BLUE,
					..default()
				},
			),
			TextSection::from_style(if cfg!(feature = "default_font") {
				TextStyle {
					font_size: 25.0,
					color: Color::RED,
					..default()
				}
			} else {
				TextStyle {
					font: asset_server.load(
						CUBEMAPS_DIR.to_owned() + "/fonts/FiraMono-Medium.ttf",
					),
					font_size: 25.0,
					color: Color::RED,
				}
			})
		]).with_style(Style {
			position_type: PositionType::Absolute,
			left: Val::Px(5.0),
			..default()
		}),
		FPSText,
		FormatText,
	));

	// Ambient Light
	commands.insert_resource(AmbientLight {
		color: Color::rgb_u8(210, 220, 240),
		brightness: 1.0,
	});

	commands.insert_resource(Cubemap {
		is_loaded: false,
		index: 0,
		image_handle: skybox_handle,
	});
}

fn text_fps_update_system(
	diagnostics: Res<DiagnosticsStore>,
	mut query: Query<&mut Text, With<FPSText>>,
) {
	for mut text in &mut query {
		if let Some(info) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
			if let Some(value) = info.smoothed() {
				text.sections[1].value = format!("{value:.2}");
			}
		}
	}
}

const CUBEMAP_SWAP_DELAY: f32 = 3.0;

fn cycle_cubemap_asset(
	time: Res<Time>,
	mut next_swap: Local<f32>,
	mut cubemap: ResMut<Cubemap>,
	asset_server: Res<AssetServer>,
	render_device: Res<RenderDevice>,
	mut query: Query<&mut Text, With<FormatText>>,
) {
	let now = time.elapsed_seconds();
	if *next_swap == 0.0 {
		*next_swap = now + CUBEMAP_SWAP_DELAY;
		return
	} else if now < *next_swap {
		return
	}
	*next_swap += CUBEMAP_SWAP_DELAY;

	let supported_compressed_formats = CompressedImageFormats::from_features(render_device.features());

	let mut new_index = cubemap.index;
	for _ in 0..CUBEMAPS.len() {
		new_index = (new_index + 1) % CUBEMAPS.len();
		if supported_compressed_formats.contains(CUBEMAPS[new_index].1) {
			break;
		}
		info!("##==> INFO! Skipping Unsupported Format: {:?}", CUBEMAPS[new_index]);
	}

	// Skip Swapping to Same Texture
	if new_index == cubemap.index {
		return
	}

	// Print Current Format to Screen
	for mut text in &mut query {
		let (path, _format): (&str, CompressedImageFormats) = match CUBEMAPS[new_index] {
			(p, f) => (p, f),
		};
		let format_path = Path::new(path);
		let file_string = format_path.file_name().unwrap().to_str().unwrap();
		text.sections[3].value = format!("{}", file_string);
	}

	cubemap.index = new_index;
	cubemap.image_handle = asset_server.load(CUBEMAPS[cubemap.index].0);
	cubemap.is_loaded = false;
}

fn animate_light_direction(time: Res<Time>, mut query: Query<&mut Transform, With<DirectionalLight>>) {
	for mut transform in &mut query {
		transform.rotate_y(time.delta_seconds() * 0.5);
	}
}

fn asset_loaded(
	asset_server: Res<AssetServer>,
	mut images: ResMut<Assets<Image>>,
	mut cubemap: ResMut<Cubemap>,
	mut skyboxes: Query<&mut Skybox>,
) {
	if !cubemap.is_loaded && asset_server.load_state(&cubemap.image_handle) == LoadState::Loaded {
		info!("##==> INFO! Swapping to {} ...", CUBEMAPS[cubemap.index].0);
		// TODO: Add Text Explaining Each Cubemap
		let image = images.get_mut(&cubemap.image_handle).unwrap();
		if image.texture_descriptor.array_layer_count() == 1 {
			image.reinterpret_stacked_2d_as_array(image.height() / image.width());
			image.texture_view_descriptor = Some(TextureViewDescriptor {
				dimension: Some(TextureViewDimension::Cube),
				..default()
			});
		}

		for mut skybox in &mut skyboxes {
			skybox.image = cubemap.image_handle.clone();
		}

		cubemap.is_loaded = true;
	}
}

pub fn daves_skybox_main() -> io::Result<()> {
	App::new()
		.add_plugins((DefaultPlugins, FrameTimeDiagnosticsPlugin))
		.add_plugins(CameraControllerPlugin)
		.add_systems(Startup, setup)
		.add_systems(
			Update,
			(
				cycle_cubemap_asset,
				asset_loaded.after(cycle_cubemap_asset),
				animate_light_direction,
				text_fps_update_system,
			),
		)
		.run();

	Ok(())
}
