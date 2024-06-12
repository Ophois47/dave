use std::f32::consts::PI;
use std::io;
use bevy::prelude::*;
use rand::Rng;
use crate::dave_graphics::ASSETS_DIR;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
	#[default]
	Playing,
	GameOver,
}

#[derive(Resource)]
struct BonusSpawnTimer(Timer);

#[derive(Resource)]
struct ScoreSound(Handle<AudioSource>);

struct Cell {
	height: f32,
}

#[derive(Default)]
struct Player {
	entity: Option<Entity>,
	i: usize,
	j: usize,
	move_cooldown: Timer,
}

#[derive(Default)]
struct Bonus {
	entity: Option<Entity>,
	i: usize,
	j: usize,
	handle: Handle<Scene>,
}

#[derive(Resource, Default)]
struct Game {
	board: Vec<Vec<Cell>>,
	player: Player,
	bonus: Bonus,
	score: i32,
	point_consumed: u32,
	camera_should_focus: Vec3,
	camera_is_focus: Vec3,
}

const BOARD_SIZE_I: usize = 14;
const BOARD_SIZE_J: usize = 21;

const RESET_FOCUS: [f32; 3] = [
	BOARD_SIZE_I as f32 / 2.0,
	0.0,
	BOARD_SIZE_J as f32 / 2.0 - 0.5,
];

fn teardown(mut commands: Commands, entities: Query<Entity, (Without<Camera>, Without<Window>)>) {
	for entity in &entities {
		commands.entity(entity).despawn();
	}
}

fn display_score(mut commands: Commands, game: Res<Game>) {
	commands.spawn(NodeBundle {
		style: Style {
			width: Val::Percent(100.),
			align_items: AlignItems::Center,
			justify_content: JustifyContent::Center,
			..default()
		},
		..default()
	})
	.with_children(|parent| {
		parent.spawn(TextBundle::from_section(
			format!("Point Consumed: {}", game.point_consumed),
			TextStyle {
				font_size: 80.0,
				color: Color::rgb(0.5, 0.5, 1.0),
				..default()
			},
		));
	});
}

fn scoreboard_system(game: Res<Game>, mut query: Query<&mut Text>) {
	let mut text = query.single_mut();
	text.sections[0].value = format!("Score: {}", game.score);
}

// Restart Game When SPACE is Pressed, End Game When ESCAPE is Pressed
fn gameover_keyboard(
	mut next_state: ResMut<NextState<GameState>>,
	keyboard_input: Res<ButtonInput<KeyCode>>,
) {
	if keyboard_input.just_pressed(KeyCode::Space) {
		next_state.set(GameState::Playing);
	}
	if keyboard_input.just_pressed(KeyCode::Escape) {
		println!();
		std::process::exit(0)
	}
}

// Let Point Turn on Itself
fn rotate_bonus(game: Res<Game>, time: Res<Time>, mut transforms: Query<&mut Transform>) {
	if let Some(entity) = game.bonus.entity {
		if let Ok(mut point_transform) = transforms.get_mut(entity) {
			point_transform.rotate_y(time.delta_seconds());
			point_transform.scale = Vec3::splat(
				1.0 + (game.score as f32 / 10.0 * time.elapsed_seconds().sin()).abs(),
			);
		}
	}
}

// Despawn Bonus if There is One, Then Spawn New One at Random Location
fn spawn_bonus(
	time: Res<Time>,
	mut timer: ResMut<BonusSpawnTimer>,
	mut next_state: ResMut<NextState<GameState>>,
	mut commands: Commands,
	mut game: ResMut<Game>,
) {
	// Wait Long Enough Before Spawning Next Point
	if !timer.0.tick(time.delta()).finished() {
		return;
	}

	if let Some(entity) = game.bonus.entity {
		game.score -= 3;
		commands.entity(entity).despawn_recursive();
		game.bonus.entity = None;
		if game.score <= -5 {
			next_state.set(GameState::GameOver);
			return;
		}
	}

	// Ensure Bonus Doesn't Spawn on Player
	loop {
		game.bonus.i = rand::thread_rng().gen_range(0..BOARD_SIZE_I);
		game.bonus.j = rand::thread_rng().gen_range(0..BOARD_SIZE_J);
		if game.bonus.i != game.player.i || game.bonus.j != game.player.j {
			break;
		}
	}

	game.bonus.entity = Some(
		commands.spawn(SceneBundle {
			transform: Transform::from_xyz(
				game.bonus.i as f32,
				game.board[game.bonus.j][game.bonus.i].height + 0.2,
				game.bonus.j as f32,
			),
			scene: game.bonus.handle.clone(),
			..default()
		})
		.with_children(|children| {
			children.spawn(PointLightBundle {
				point_light: PointLight {
					color: Color::rgb(1.0, 1.0, 0.0),
					intensity: 500_000.0,
					range: 10.0,
					..default()
				},
				transform: Transform::from_xyz(0.0, 2.0, 0.0),
				..default()
			});
		})
		.id(),
	);
}

// Change Focus of Camera
fn focus_camera(
	time: Res<Time>,
	mut game: ResMut<Game>,
	mut transforms: ParamSet<(Query<&mut Transform, With<Camera3d>>, Query<&Transform>)>,
) {
	const SPEED: f32 = 2.0;

	// If Player and Bonus Exist, Target Mid-Point
	if let (Some(player_entity), Some(bonus_entity)) = (game.player.entity, game.bonus.entity) {
		let transform_query = transforms.p1();
		if let (Ok(player_transform), Ok(bonus_transform)) = (
			transform_query.get(player_entity),
			transform_query.get(bonus_entity),
		) {
			game.camera_should_focus = player_transform
				.translation
				.lerp(bonus_transform.translation, 0.5);
		}
	// If Only Player Exists, Target Player
	} else if let Some(player_entity) = game.player.entity {
		if let Ok(player_transform) = transforms.p1().get(player_entity) {
			game.camera_should_focus = player_transform.translation;
		}
	// Otherwise Target Middle
	} else {
		game.camera_should_focus = Vec3::from(RESET_FOCUS);
	}

	// Calculate Camera Motion Based on Difference Between Where Camera Looking
    // and Where it Should be Looking
    // Greater Distance, Faster Motion
    // Smooth Out Camera Movement Using Frame Time
    let mut camera_motion = game.camera_should_focus - game.camera_is_focus;
    if camera_motion.length() > 0.2 {
    	camera_motion *= SPEED * time.delta_seconds();
    	// Set New Camera's Actual Focus
    	game.camera_is_focus += camera_motion;
    }
    // Look At New Camera's Actual Focus
    for mut transform in transforms.p0().iter_mut() {
    	*transform = transform.looking_at(game.camera_is_focus, Vec3::Y);
    }
}

fn move_player(
	mut commands: Commands,
	keyboard_input: Res<ButtonInput<KeyCode>>,
	mut game: ResMut<Game>,
	mut transforms: Query<&mut Transform>,
	sound: Res<ScoreSound>,
	time: Res<Time>,
) {
	if game.player.move_cooldown.tick(time.delta()).finished() {
		let mut moved = false;
		let mut rotation = 0.0;

		if keyboard_input.pressed(KeyCode::ArrowUp) {
			if game.player.i < BOARD_SIZE_I - 1 {
				game.player.i += 1;
			}
			rotation = -PI / 2.;
			moved = true;
		}
		if keyboard_input.pressed(KeyCode::ArrowDown) {
			if game.player.i > 0 {
				game.player.i -= 1;
			}
			rotation = PI / 2.;
			moved = true;
		}
		if keyboard_input.pressed(KeyCode::ArrowRight) {
			if game.player.j < BOARD_SIZE_J - 1 {
				game.player.j += 1;
			}
			rotation = PI;
			moved = true;
		}
		if keyboard_input.pressed(KeyCode::ArrowLeft) {
			if game.player.j > 0 {
				game.player.j -= 1;
			}
			rotation = 0.0;
			moved = true;
		}

		// Move On The Board
		if moved {
			game.player.move_cooldown.reset();
			*transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
				translation: Vec3::new(
					game.player.i as f32,
					game.board[game.player.j][game.player.i].height,
					game.player.j as f32,
				),
				rotation: Quat::from_rotation_y(rotation),
				..default()
			};
		}
	}

	// Consume Point!
	if let Some(entity) = game.bonus.entity {
		if game.player.i == game.bonus.i && game.player.j == game.bonus.j {
			// Play Sound for Point Score
			commands.spawn(AudioBundle {
				source: sound.0.clone(),
				// Auto-Despawn Entity When Playback Finishes
				settings: PlaybackSettings::DESPAWN,
			});
			// Increment Score
			game.score += 2;
			game.point_consumed += 1;
			commands.entity(entity).despawn_recursive();
			game.bonus.entity = None;
		}
	}
}

fn setup_cameras(mut commands: Commands, mut game: ResMut<Game>) {
	game.camera_should_focus = Vec3::from(RESET_FOCUS);
	game.camera_is_focus = game.camera_should_focus;
	commands.spawn(Camera3dBundle {
		transform: Transform::from_xyz(
			-(BOARD_SIZE_I as f32 / 2.0),
			2.0 * BOARD_SIZE_J as f32 / 3.0,
			BOARD_SIZE_J as f32 / 2.0 - 0.5,
		)
		.looking_at(game.camera_is_focus, Vec3::Y),
		..default()
	});
}

fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut game: ResMut<Game>,
) {
	// Reset Game State
	game.point_consumed = 0;
	game.score = 0;
	game.player.i = BOARD_SIZE_I / 2;
	game.player.j = BOARD_SIZE_J / 2;
	game.player.move_cooldown = Timer::from_seconds(0.3, TimerMode::Once);

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

	// Game Audio
	commands.spawn(AudioBundle {
		source: asset_server.load(ASSETS_DIR.to_owned() + "/audio/slopes.ogg"),
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

	// Spawn Game Board
	let cell_scene = asset_server.load(
		ASSETS_DIR.to_owned() + "/models/dave_game/tile.glb#Scene0",
	);
	game.board = (0..BOARD_SIZE_J)
		.map(|j| {
			(0..BOARD_SIZE_I)
				.map(|i| {
					let height = rand::thread_rng().gen_range(-0.1..0.1);
					commands.spawn(SceneBundle {
						transform: Transform::from_xyz(i as f32, height - 0.2, j as f32),
						scene: cell_scene.clone(),
						..default()
					});
					Cell { height }
				})
				.collect()
		})
		.collect();

	// Spawn Game Character
	game.player.entity = Some(
		commands.spawn(SceneBundle {
			transform: Transform {
				translation: Vec3::new(
					game.player.i as f32,
					game.board[game.player.j][game.player.i].height,
					game.player.j as f32,
				),
				rotation: Quat::from_rotation_y(-PI / 2.),
				..default()
			},
			scene: asset_server.load(
				ASSETS_DIR.to_owned() + "/models/dave_game/alien.glb#Scene0",
			),
			..default()
		})
		.id(),
	);

	// Load Scene for Point
	game.bonus.handle = asset_server.load(
		ASSETS_DIR.to_owned() + "/models/dave_game/cakeBirthday.glb#Scene0",
	);

	// Scoreboard
	commands.spawn(
		TextBundle::from_section(
			"Score:",
			TextStyle {
				font_size: 30.0,
				color: Color::rgb(0.5, 0.5, 1.0),
				..default()
			},
		)
		.with_style(Style {
			position_type: PositionType::Absolute,
			top: Val::Px(5.0),
			left: Val::Px(5.0),
			..default()
		}),
	);

	info!("Press ESC to Quit")
}

pub fn davegame_main() -> io::Result<()> {
	App::new()
		.add_plugins(DefaultPlugins)
		.init_resource::<Game>()
		.insert_resource(BonusSpawnTimer(Timer::from_seconds(
			5.0,
			TimerMode::Repeating,
		)))
		.init_state::<GameState>()
		.add_systems(Startup, setup_cameras)
		.add_systems(OnEnter(GameState::Playing), setup)
		.add_systems(
			Update,
			(
				move_player,
				focus_camera,
				rotate_bonus,
				scoreboard_system,
				spawn_bonus,
			)
			.run_if(in_state(GameState::Playing)),
		)
		.add_systems(OnExit(GameState::Playing), teardown)
		.add_systems(OnEnter(GameState::GameOver), display_score)
		.add_systems(
			Update,
			(
				gameover_keyboard.run_if(in_state(GameState::GameOver)),
				bevy::window::close_on_esc,
			),
		)
		.add_systems(OnExit(GameState::GameOver), teardown)
		.run();

	Ok(())
}
