use std::io;
use bevy::{
	diagnostic::{
		FrameTimeDiagnosticsPlugin,
		LogDiagnosticsPlugin,
	},
	prelude::*,
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
use crate::dave_graphics::ASSETS_DIR;

//
// Too Many Buttons!
//
/*struct Args {
	// Give Buttons Text
	no_text: bool,
	// Give Buttons Border
	no_borders: bool,
	// Perform Full Relayout Each Frame
	relayout: bool,
	// Recompute All Text Each Frame
	recompute_text: bool,
	// Default Buttons 110
	buttons: usize,
	// Give Every Nth Button an Image
	// Default 4
	image_freq: usize,
	// Use Grid Layout Model
	grid: bool,
}*/

// Temporary Arg Solution
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
	mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &IdleColor), Changed<Interaction>>,
) {
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
        Some(asset_server.load("branding/icon.png"))
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

pub fn st_too_many_buttons(
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

	app.run();
	Ok(())
}
