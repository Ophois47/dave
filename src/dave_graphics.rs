use std::io;
use bevy::prelude::*;

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
	/*commands.spawn(Camera3dBundle {
		transform: Transform::from_xyz(-2.5, 4.5, 9.0)
			.looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	});*/
}

pub fn daves_cube() -> io::Result<()> {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_systems(Startup, daves_cube_setup)
		.run();

	Ok(())
}
