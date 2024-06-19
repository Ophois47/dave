use bevy::{
	prelude::*,
	time::common_conditions::on_timer,
};
use rand::{
	distributions::{
		Distribution,
		Uniform,
	},
	Rng,
};
use std::f32::consts::{
	FRAC_PI_2,
	PI,
};
use std::io;
use std::time::Duration;
use crate::dave_graphics::ASSETS_DIR;

#[derive(Clone, Copy, Eq, PartialEq, Debug, Default, Hash, States, Reflect)]
pub enum GameState {
	#[default]
	Playing,
	GameOver,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Obstacle;

#[derive(Component)]
struct Street;

#[derive(Component)]
struct Point;

#[derive(Component)]
struct PointText;

#[derive(Component)]
struct BestText;

#[derive(Resource)]
struct ScoreSound(Handle<AudioSource>);

#[derive(Resource)]
struct CrashSound(Handle<AudioSource>);

#[derive(Resource)]
struct EngineSound(Handle<AudioSource>);

#[derive(Resource)]
struct TireSound(Handle<AudioSource>);

#[derive(Resource)]
struct Score {
	value: i32,
	best: i32,
}

impl Default for Score {
	fn default() -> Self {
		Self {
			value: 0,
			best: 0,
		}
	}
}

pub fn dave_cars_main() -> io::Result<()> {
	App::new()
		// Add Config Resources
		.insert_resource(Msaa::Sample4)
		.insert_resource(Score::default())
		.add_plugins(DefaultPlugins)
		.init_state::<GameState>()
		.add_systems(OnEnter(GameState::Playing), dave_cars_setup)
		.add_systems(
			Update,
			(
				move_car,
				move_street,
				move_point,
				move_obstacle,
				collision_point,
				collision_obstacle,
				rotate_point,
				scoreboard
			),
		)
		.add_systems(Update, spawn_obstacle.run_if(on_timer(Duration::from_secs_f32(1.2))))
		.add_systems(OnEnter(GameState::GameOver), show_text)
		.add_systems(Update, gameover_keyboard)
		.add_systems(OnExit(GameState::GameOver), teardown)
		.run();

	Ok(())
}

fn dave_cars_setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	// Camera
	commands.spawn(Camera3dBundle {
		transform: Transform::from_xyz(1.0, 6.0, 3.0).looking_at(Vec3::new(1., 0., -2.), Vec3::Y),
		..Default::default()
	});

	// Lights
	commands.spawn(PointLightBundle {
		transform: Transform::from_xyz(4.0, 10.0, 4.0),
		point_light: PointLight {
			intensity: 2_000_000.0,
			shadows_enabled: true,
			range: 30.0,
			..default()
		},
		..default()
	});

	// Setup Game Audio
	commands.spawn(AudioBundle {
		source: asset_server.load(ASSETS_DIR.to_owned() + "/audio/stalker.ogg"),
		settings: PlaybackSettings {
			mode: bevy::audio::PlaybackMode::Loop,
			..default()
		},
		..default()
	});
	let score_sound = asset_server.load(
		ASSETS_DIR.to_owned() + "/audio/collision.ogg",
	);
	commands.insert_resource(ScoreSound(score_sound));
	let crash_sound = asset_server.load(
		ASSETS_DIR.to_owned() + "/audio/crash.ogg",
	);
	commands.insert_resource(CrashSound(crash_sound));
	let engine_sound = asset_server.load(
		ASSETS_DIR.to_owned() + "/audio/revving.ogg",
	);
	commands.insert_resource(EngineSound(engine_sound));
	let tire_sound = asset_server.load(
		ASSETS_DIR.to_owned() + "/audio/tires.ogg",
	);
	commands.insert_resource(TireSound(tire_sound));

	// Street
	let mut rng = rand::thread_rng();
	let die = Uniform::from(0..3);

	for j in -9..2 {
		let mut children_list: Vec<Entity> = Vec::new();
		for i in 0..3 {
			let entity = commands.spawn((
				Transform {
					translation: Vec3::new(i as f32, 0.0, 0.0),
					rotation: Quat::from_rotation_y(FRAC_PI_2),
					..Default::default()
				},
				GlobalTransform::IDENTITY,
			))
			.with_children(|parent| {
				parent.spawn(SceneBundle {
					scene: asset_server.load(
						ASSETS_DIR.to_owned() + "/models/dave_cars/road_straight.glb#Scene0",
					),
					..Default::default()
				});
			}).id();
			children_list.push(entity);
			if i == 0 {
				let lamp = commands.spawn((
					Transform {
						translation: Vec3::new(i as f32-0.45, 0.0, 0.0),
						rotation: Quat::from_rotation_y(FRAC_PI_2),
						..Default::default()
					},
					GlobalTransform::IDENTITY,
				))
				.with_children(|parent| {
					parent.spawn(SceneBundle {
						scene: asset_server.load(
							ASSETS_DIR.to_owned() + "/models/dave_cars/lamp.glb#Scene0",
						),
						..Default::default()
					});
				}).id();
				children_list.push(lamp);
			}
			if i == 2 {
				let lamp = commands.spawn((
					Transform {
						translation: Vec3::new(i as f32+0.45, 0.0, 0.0),
						rotation: Quat::from_rotation_y(-FRAC_PI_2),
						..Default::default()
					},
					GlobalTransform::IDENTITY,
				))
				.with_children(|parent| {
					parent.spawn(SceneBundle {
						scene: asset_server.load(
							ASSETS_DIR.to_owned() + "/models/dave_cars/lamp.glb#Scene0",
						),
						..Default::default()
					});
				}).id();
				children_list.push(lamp);
			}
			commands.spawn((
				Transform {
					translation: Vec3::new(0.0, 0.0, j as f32),
					..Default::default()
				},
				GlobalTransform::IDENTITY,
			)).insert(Street)
			.push_children(&children_list);
		}
		// Point Item
		if j < -1 {
			let ran_street = die.sample(&mut rng);
			commands.spawn((
				Transform {
					translation: Vec3::new(ran_street as f32, 0.2, j as f32),
					rotation: Quat::from_rotation_y(PI + 5.0),
					scale: Vec3::new(0.7, 0.7, 0.7),
					..Default::default()
				},
				GlobalTransform::IDENTITY,
			))
			.with_children(|parent| {
				parent.spawn(SceneBundle {
					scene: asset_server.load(
						ASSETS_DIR.to_owned() + "/models/dave_cars/trophy.glb#Scene0",
					),
					..Default::default()
				});
			})
			.insert(Point);
		}
	}

	// Player
	commands.spawn((
		Transform {
			translation: Vec3::new(0.0, 0.5, 0.4),
			rotation: Quat::from_rotation_y(PI),
			scale: Vec3::new(0.11, 0.11, 0.11),
			..Default::default()
		},
		GlobalTransform::IDENTITY,
	))
	.with_children(|parent| {
		parent.spawn(SceneBundle {
			scene: asset_server.load(
				ASSETS_DIR.to_owned() + "/models/dave_cars/dave_challenger.glb#Scene0",
			),
			..Default::default()
		});
	})
	.insert(Player);

	// Scoreboard
	commands.spawn(TextBundle {
		text: Text::from_section(
			"Score:",
			TextStyle {
				font: asset_server.load(
					ASSETS_DIR.to_owned() + "/fonts/FiraSans-Bold.ttf",
				),
				font_size: 40.0,
				color: Color::rgb(0.5, 0.5, 1.0),
			},
		),
		style: Style {
			position_type: PositionType::Absolute,
			top: Val::Px(5.0),
			left: Val::Px(5.0),
			..Default::default()
		},
		..Default::default()
	}).insert(PointText);

	commands.spawn(TextBundle {
		text: Text::from_section(
			"Best:",
			TextStyle {
				font: asset_server.load(
					ASSETS_DIR.to_owned() + "/fonts/FiraSans-Bold.ttf",
				),
				font_size: 40.0,
				color: Color::rgb(0.5, 0.5, 1.0),
			},
		),
		style: Style {
			position_type: PositionType::Absolute,
			top: Val::Px(5.0),
			right: Val::Px(25.0),
			..Default::default()
		},
		..Default::default()
	})
	.insert(BestText);
}

fn move_car(
	mut commands: Commands,
	keyboard_input: Res<ButtonInput<KeyCode>>,
	mut position: Query<&mut Transform, With<Player>>,
	engine_sound: Res<EngineSound>,
	tire_sound: Res<TireSound>,
) {
	for mut transform in position.iter_mut() {
		if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
			let mut x = transform.translation.x - 1.0;
			if x < 0.0 { x = 0.0 };
			transform.translation = Vec3::new(
				x,
				transform.translation.y,
				transform.translation.z,
			);
			// Play Sound for Changing Lanes
			commands.spawn(AudioBundle {
				source: tire_sound.0.clone(),
				// Auto-Despawn Entity When Playback Finishes
				settings: PlaybackSettings::DESPAWN,
			});
		}
		if keyboard_input.just_pressed(KeyCode::ArrowRight) {
			let mut x = transform.translation.x + 1.0;
			if x > 2.0 { x = 2.0 };
			transform.translation = Vec3::new(
				x,
				transform.translation.y,
				transform.translation.z,
			);
			// Play Sound for Changing Lanes
			commands.spawn(AudioBundle {
				source: tire_sound.0.clone(),
				// Auto-Despawn Entity When Playback Finishes
				settings: PlaybackSettings::DESPAWN,
			});
		}
		if keyboard_input.just_pressed(KeyCode::ArrowUp) {
			// Play Sound for Revving Engine
			commands.spawn(AudioBundle {
				source: engine_sound.0.clone(),
				// Auto-Despawn Entity When Playback Finishes
				settings: PlaybackSettings::DESPAWN,
			});
		}
		if keyboard_input.just_pressed(KeyCode::Escape) {
			println!();
			std::process::exit(0)
		}
	}
}

const STREET_SPEED: f32 = 3.0;

fn move_street(
	mut commands: Commands,
	time: Res<Time>,
	mut position: Query<&mut Transform, With<Street>>,
	asset_server: Res<AssetServer>,
) {
	for mut transform in position.iter_mut() {
		transform.translation = transform.translation + Vec3::new(0.0, 0.0, 1.0) * STREET_SPEED * time.delta_seconds();
		if transform.translation.z > 2.0 {
			transform.translation.z -= 11.0;
			let mut rng = rand::thread_rng();
			let ran_ = rng.gen_range(0..10);
			if ran_ < 2 {
				let die = Uniform::from(0..3);
				let ran_street = die.sample(&mut rng);
				commands.spawn((
					Transform {
						translation: Vec3::new(ran_street as f32, 0.2, transform.translation.z),
						rotation: Quat::from_rotation_y(PI + 5.0),
						scale: Vec3::new(0.7, 0.7, 0.7),
						..Default::default()
					},
					GlobalTransform::IDENTITY,
				))
				.with_children(|parent| {
					parent.spawn(SceneBundle {
						scene: asset_server.load(
							ASSETS_DIR.to_owned() + "/models/dave_cars/trophy.glb#Scene0",
						),
						..Default::default()
					});
				})
				.insert(Point);
			}
		}
	}
}

fn move_point(
	time: Res<Time>,
	mut commands: Commands,
	mut position: Query<(Entity, &mut Transform), With<Point>>,
) {
	for (entity, mut transform) in position.iter_mut() {
		transform.translation = transform.translation
			+ Vec3::new(0.0, 0.0, 1.0)
			* STREET_SPEED
			* time.delta_seconds();

		if transform.translation.z >= 2.0 {
			commands.entity(entity).despawn_recursive();
		}
	}
}

fn rotate_point(time: Res<Time>, mut query: Query<&mut Transform, With<Point>>) {
	for mut transform in &mut query {
		transform.rotate_y(2.0 * time.delta_seconds());
	}
}

fn collision_point(
	mut commands: Commands,
	mut score: ResMut<Score>,
	position: Query<(Entity, &Transform), With<Point>>,
	player_position: Query<&Transform, With<Player>>,
	sound: Res<ScoreSound>,
) {
	let player_transform = player_position.single();
	for (entity, transform) in position.iter() {
		if transform.translation.x == player_transform.translation.x {
			if (transform.translation.z - player_transform.translation.z).abs() < 0.4 {
				// Play Sound for Point Score
				commands.spawn(AudioBundle {
					source: sound.0.clone(),
					// Auto-Despawn Entity When Playback Finishes
					settings: PlaybackSettings::DESPAWN,
				});

				commands.entity(entity).despawn_recursive();
				score.value += 1;
			}
		}
	}
}

fn scoreboard(
	score: Res<Score>,
	mut point_query: Query<&mut Text, (With<PointText>, Without<BestText>)>,
	mut best_query: Query<&mut Text, With<BestText>>,
) {
	let mut text = point_query.single_mut();
	text.sections[0].value = format!("Score: {}", score.value);

	let mut best_text = best_query.single_mut();
	best_text.sections[0].value = format!("High Score: {}", score.best);
}

fn spawn_obstacle(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	let mut rng = rand::thread_rng();
	let die = Uniform::from(0..3);
	let ran_street = die.sample(&mut rng);

	let sports_car = ASSETS_DIR.to_owned() + "/models/dave_cars/hatchbackSports.glb#Scene0";
	let police_car = ASSETS_DIR.to_owned() + "/models/dave_cars/police.glb#Scene0";
	let sedan_car = ASSETS_DIR.to_owned() + "/models/dave_cars/sedan.glb#Scene0";
	let tractor = ASSETS_DIR.to_owned() + "/models/dave_cars/tractor.glb#Scene0";

	let obstacle_models = vec![
		sports_car,
		police_car,
		sedan_car,
		tractor,
	];

	let model = &obstacle_models[rng.gen_range(0..obstacle_models.len())];
	commands.spawn((
		Transform {
			translation: Vec3::new(ran_street as f32, 0.0, -10.0),
			scale: Vec3::new(0.4, 0.4, 0.4),
			rotation: Quat::from_rotation_y(PI),
		},
		GlobalTransform::IDENTITY,
	))
	.with_children(|parent| {
		parent.spawn(SceneBundle {
			scene: asset_server.load(model),
			..Default::default()
		});
	})
	.insert(Obstacle);
}

const OBSTACLE_SPEED: f32 = 3.5;

fn move_obstacle(
	time: Res<Time>,
	mut commands: Commands,
	mut position: Query<(Entity, &mut Transform), With<Obstacle>>,
) {
	for (entity, mut transform) in position.iter_mut() {
		transform.translation = transform.translation
			+ Vec3::new(0.0, 0.0, 1.0)
			* OBSTACLE_SPEED
			* time.delta_seconds();

		if transform.translation.z >= 2.0 {
			commands.entity(entity).despawn_recursive();
		}
	}
}

fn teardown(mut commands: Commands, entities: Query<Entity>) {
	for entity in entities.iter() {
		commands.entity(entity).despawn_recursive();
	}
}

fn show_text(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	commands.spawn(NodeBundle {
		style: Style {
			margin: UiRect::all(Val::Auto),
			justify_content: JustifyContent::Center,
			align_items: AlignItems::Center,
			..default()
		},
		background_color: Color::NONE.into(),
		..default()
	})
	.with_children(|parent| {
		parent.spawn(TextBundle {
			text: Text::from_section(
				"Press Any Key to Restart ...",
				TextStyle {
					font: asset_server.load(
						ASSETS_DIR.to_owned() + "/fonts/FiraSans-Bold.ttf",
					),
					font_size: 40.0,
					color: Color::rgb(0.5, 0.5, 1.0),
				},
			),
			..default()
		});
	});
}

fn collision_obstacle(
	mut commands: Commands,
	mut score: ResMut<Score>,
	_state: ResMut<State<GameState>>,
	position: Query<(Entity, &Transform), With<Obstacle>>,
	player_position: Query<&Transform, With<Player>>,
	sound: Res<CrashSound>,
) {
	let player_transform = player_position.single();
	for (_entity, transform) in position.iter() {
		if transform.translation.x == player_transform.translation.x {
			if (transform.translation.z - player_transform.translation.z).abs() < 0.4 {
				// state.set(Box::new(GameState::GameOver)).unwrap();
				// Play Sound for Collision
				commands.spawn(AudioBundle {
					source: sound.0.clone(),
					// Auto-Despawn Entity When Playback Finishes
					settings: PlaybackSettings::DESPAWN,
				});

				if score.value > score.best {
					score.best = score.value;
					score.value = 0;
				}
				return;
			}
		}
	}
}

fn gameover_keyboard(
	mut state: ResMut<State<GameState>>,
	keyboard_input: Res<ButtonInput<KeyCode>>,
) {
	if keyboard_input.just_pressed(KeyCode::Space) {
		state.set(Box::new(GameState::Playing)).unwrap();
	}
}
