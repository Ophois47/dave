use std::io;
use bevy::{
	math::bounding::{
		Aabb2d,
		BoundingCircle,
		BoundingVolume,
		IntersectsVolume,
	},
	prelude::*,
	sprite::MaterialMesh2dBundle,
};
use crate::dave_graphics::SteppingPlugin;

const PADDLE_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.0);
const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;
const PADDLE_SPEED: f32 = 500.0;
// How Close Paddle Can Get To Wall
const PADDLE_PADDING: f32 = 10.0;

// Avoid Overlapping Sprites
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_DIAMETER: f32 = 30.;
const BALL_SPEED: f32 = 400.0;
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

const WALL_THICKNESS: f32 = 10.0;
// X Coordinates
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
// Y Coordinates
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

const BRICK_SIZE: Vec2 = Vec2::new(100., 30.);
// These Values Are Exact
const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 270.0;
const GAP_BETWEEN_BRICKS: f32 = 5.0;
// These Values Are Lower Bounds, As # Of Bricks Computed
const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.0;
const GAP_BETWEEN_BRICKS_AND_SIDES: f32 = 20.0;

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const BRICK_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Event, Default)]
struct CollisionEvent;

#[derive(Component)]
struct Brick;

#[derive(Resource)]
struct CollisionSound(Handle<AudioSource>);

// This Bundle is a Collection of Components That
// Define a Wall in Our Game
#[derive(Bundle)]
struct WallBundle {
	sprite_bundle: SpriteBundle,
	collider: Collider,
}

// Which Side of Arena is Wall Located On
enum WallLocation {
	Left,
	Right,
	Bottom,
	Top,
}

impl WallLocation {
	fn position(&self) -> Vec2 {
		match self {
			WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
			WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
			WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
			WallLocation::Top => Vec2::new(0., TOP_WALL),
		}
	}

	fn size(&self) -> Vec2 {
		let arena_height = TOP_WALL - BOTTOM_WALL;
		let arena_width = RIGHT_WALL - LEFT_WALL;
		assert!(arena_height > 0.0);
		assert!(arena_width > 0.0);

		match self {
			WallLocation::Left | WallLocation::Right => {
				Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
			}
			WallLocation::Bottom | WallLocation::Top => {
				Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
			}
		}
	}
}

impl WallBundle {
	fn new(location: WallLocation) -> WallBundle {
		WallBundle {
			sprite_bundle: SpriteBundle {
				transform: Transform {
					translation: location.position().extend(0.0),
					scale: location.size().extend(1.0),
					..default()
				},
				sprite: Sprite {
					color: WALL_COLOR,
					..default()
				},
				..default()
			},
			collider: Collider,
		}
	}
}

// This Resource Tracks Game Score
#[derive(Resource)]
struct Scoreboard {
	score: usize,
}

#[derive(Component)]
struct ScoreboardUI;

// Add Game's Entities To World
fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	asset_server: Res<AssetServer>,
) {
	// Camera
	commands.spawn(Camera2dBundle::default());

	// Sound
	let ball_collision_sound = asset_server.load("sounds/breakout_collision.ogg");
	commands.insert_resource(CollisionSound(ball_collision_sound));

	// Paddle
	let paddle_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;

	commands.spawn((
		SpriteBundle {
			transform: Transform {
				translation: Vec3::new(0.0, paddle_y, 0.0),
				scale: PADDLE_SIZE,
				..default()
			},
			sprite: Sprite {
				color: PADDLE_COLOR,
				..default()
			},
			..default()
		},
		Paddle,
		Collider,
	));

	// Ball
	commands.spawn((
		MaterialMesh2dBundle {
			mesh: meshes.add(Circle::default()).into(),
			material: materials.add(BALL_COLOR),
			transform: Transform::from_translation(BALL_STARTING_POSITION)
				.with_scale(Vec2::splat(BALL_DIAMETER).extend(1.)),
			..default()
		},
		Ball,
		Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED),
	));

	// Scoreboard
	commands.spawn((
		ScoreboardUI,
		TextBundle::from_sections([
			TextSection::new(
				"Score: ",
				TextStyle {
					font_size: SCOREBOARD_FONT_SIZE,
					color: TEXT_COLOR,
					..default()
				},
			),
			TextSection::from_style(TextStyle {
				font_size: SCOREBOARD_FONT_SIZE,
				color: SCORE_COLOR,
				..default()
			}),
		])
		.with_style(Style {
			position_type: PositionType::Absolute,
			top: SCOREBOARD_TEXT_PADDING,
			left: SCOREBOARD_TEXT_PADDING,
			..default()
		}),
	));

	// Walls
	commands.spawn(WallBundle::new(WallLocation::Left));
	commands.spawn(WallBundle::new(WallLocation::Right));
	commands.spawn(WallBundle::new(WallLocation::Bottom));
	commands.spawn(WallBundle::new(WallLocation::Top));

	// Bricks
	let total_width_of_bricks = (RIGHT_WALL - LEFT_WALL) - 2. * GAP_BETWEEN_BRICKS_AND_SIDES;
	let bottom_edge_of_bricks = paddle_y + GAP_BETWEEN_PADDLE_AND_BRICKS;
	let total_height_of_bricks = TOP_WALL - bottom_edge_of_bricks - GAP_BETWEEN_BRICKS_AND_CEILING;

	assert!(total_width_of_bricks > 0.0);
	assert!(total_height_of_bricks > 0.0);

	// Given Space Available, Compute How Many Rows + Columns
	// of Bricks Can Fit
	let n_columns = (total_width_of_bricks / (BRICK_SIZE.x + GAP_BETWEEN_BRICKS)).floor() as usize;
	let n_rows = (total_height_of_bricks / (BRICK_SIZE.y + GAP_BETWEEN_BRICKS)).floor() as usize;
	let n_vertical_gaps = n_columns - 1;

	let center_of_bricks = (LEFT_WALL + RIGHT_WALL) / 2.0;
	let left_edge_of_bricks = center_of_bricks
		- (n_columns as f32 / 2.0 * BRICK_SIZE.x)
		- n_vertical_gaps as f32 / 2.0 * GAP_BETWEEN_BRICKS;

	// In Bevy, 'translation' of Entity Describes Center Point
	// Not Bottom Left Corner
	let offset_x = left_edge_of_bricks + BRICK_SIZE.x / 2.;
	let offset_y = bottom_edge_of_bricks + BRICK_SIZE.y / 2.;

	for row in 0..n_rows {
		for column in 0..n_columns {
			let brick_position = Vec2::new(
				offset_x + column as f32 * (BRICK_SIZE.x + GAP_BETWEEN_BRICKS),
				offset_y + row as f32 * (BRICK_SIZE.y + GAP_BETWEEN_BRICKS),
			);

			// Brick
			commands.spawn((
				SpriteBundle {
					sprite: Sprite {
						color: BRICK_COLOR,
						..default()
					},
					transform: Transform {
						translation: brick_position.extend(0.0),
						scale: Vec3::new(BRICK_SIZE.x, BRICK_SIZE.y, 1.0),
						..default()
					},
					..default()
				},
				Brick,
				Collider,
			));
		}
	}
}

fn move_paddle(
	keyboard_input: Res<ButtonInput<KeyCode>>,
	mut query: Query<&mut Transform, With<Paddle>>,
	time: Res<Time>,
) {
	let mut paddle_transform = query.single_mut();
	let mut direction = 0.0;

	if keyboard_input.pressed(KeyCode::ArrowLeft) {
		direction -= 1.0;
	}

	if keyboard_input.pressed(KeyCode::ArrowRight) {
		direction += 1.0;
	}

	// Calculate New Horizontal Paddle Position Based
	// On Player Input
	let new_paddle_position = paddle_transform.translation.x + direction * PADDLE_SPEED * time.delta_seconds();

	// Update Paddle Position, Honoring Arena Bounds
	let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + PADDLE_SIZE.x / 2.0 + PADDLE_PADDING;
	let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.x / 2.0 - PADDLE_PADDING;

	paddle_transform.translation.x = new_paddle_position.clamp(left_bound, right_bound);
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
	for (mut transform, velocity) in &mut query {
		transform.translation.x += velocity.x * time.delta_seconds();
		transform.translation.y += velocity.y * time.delta_seconds();
	}
}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text, With<ScoreboardUI>>) {
	let mut text = query.single_mut();
	text.sections[1].value = scoreboard.score.to_string();
}

fn check_for_collisions(
	mut commands: Commands,
	mut scoreboard: ResMut<Scoreboard>,
	mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
	collider_query: Query<(Entity, &Transform, Option<&Brick>), With<Collider>>,
	mut collision_events: EventWriter<CollisionEvent>,
) {
	let (mut ball_velocity, ball_transform) = ball_query.single_mut();

	// Check Collision With Walls
	for (collider_entity, transform, maybe_brick) in &collider_query {
		let collision = collide_with_side(
			BoundingCircle::new(ball_transform.translation.truncate(), BALL_DIAMETER / 2.),
			Aabb2d::new(
				transform.translation.truncate(),
				transform.scale.truncate() / 2.,
			),
		);

		if let Some(collision) = collision {
			// Send Collision Event So Other Systems Can React
			collision_events.send_default();

			// Bricks Should Be Despawned
			// Scoreboard Incremented
			if maybe_brick.is_some() {
				scoreboard.score += 1;
				commands.entity(collider_entity).despawn();
			}

			// Reflect Ball When It Collides
			let mut reflect_x = false;
			let mut reflect_y = false;

			// Only Reflect If Ball's Velocity Going Opposite
			// Direction of Collision
			match collision {
				Collision::Left => reflect_x = ball_velocity.x > 0.0,
				Collision::Right => reflect_x = ball_velocity.x < 0.0,
				Collision::Top => reflect_y = ball_velocity.y < 0.0,
				Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
			}

			// Reflect Velocity on X-Axis When Necessary
			if reflect_x {
				ball_velocity.x = -ball_velocity.x;
			}
			// Reflect Velocity on Y-Axis When Necessary
			if reflect_y {
				ball_velocity.y = -ball_velocity.y;
			}
		}
	}
}

fn play_collision_sound(
	mut commands: Commands,
	mut collision_events: EventReader<CollisionEvent>,
	sound: Res<CollisionSound>,
) {
	// Play Sound Once Per Frame When Collision Occurs
	if !collision_events.is_empty() {
		// This Prevents Events Staying Active On Next Frame
		collision_events.clear();
		commands.spawn(AudioBundle {
			source: sound.0.clone(),
			// Auto-Despawn Entity When Playback Finishes
			settings: PlaybackSettings::DESPAWN,
		});
	}
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
	Left,
	Right,
	Top,
	Bottom,
}

// Returns 'Some' if Ball Collides With Wall. The Returned
// Collision Is Side of Wall Ball Hit
fn collide_with_side(ball: BoundingCircle, wall: Aabb2d) -> Option<Collision> {
	if !ball.intersects(&wall) {
		return None
	}

	let closest = wall.closest_point(ball.center());
	let offset = ball.center() - closest;
	let side = if offset.x.abs() > offset.y.abs() {
		if offset.x < 0. {
			Collision::Left
		} else {
			Collision::Right
		}
	} else if offset.y > 0. {
		Collision::Top
	} else {
		Collision::Bottom
	};

	Some(side)
}

pub fn dave_breakout_main() -> io::Result<()> {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(
			SteppingPlugin::default()
				.add_schedule(Update)
				.add_schedule(FixedUpdate)
				.at(Val::Percent(35.0), Val::Percent(50.0)),
		)
		.insert_resource(Scoreboard { score: 0 })
		.insert_resource(ClearColor(BACKGROUND_COLOR))
		.add_event::<CollisionEvent>()
		.add_systems(Startup, setup)
		// Add Gameplay Simulation System to Fixed Timestep
		// Schedule Which Runs at 64Hz by Default
		.add_systems(
			FixedUpdate,
			(
				apply_velocity,
				move_paddle,
				check_for_collisions,
				play_collision_sound,
			)
			.chain(),
		)
		.add_systems(Update, (update_scoreboard, bevy::window::close_on_esc))
		.run();

	Ok(())
}
