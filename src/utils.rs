use std::io;
use std::f32::consts::*;
use std::fmt;
use std::fs;
use std::path::Path;
use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::CursorGrabMode,
};
use bytesize::ByteSize;
use colored::*;
use rand::Rng;
use spinners::{
    Spinner,
    Spinners,
};
use sysinfo::{
    Components,
    Disks,
    Networks,
    System,
};
use walkdir::WalkDir;

pub fn generate_random_number(min_value: u16, max_value: u16) -> u16 {
    let mut rng = rand::thread_rng();
    let random_value: u16 = rng.gen_range(min_value..max_value);
    random_value
}

pub fn get_file_size(path: &Path) -> io::Result<()> {
    if path.exists() {
        let file_metadata = fs::metadata(path)?;
        let stop_symbol = format!("{}", "🗸".green());

        if file_metadata.is_dir() {
            println!("##==> Path '{}' Points to a Directory", path.display());
            println!("##==> Calculating Size of Directory ...");

            let mut spinner = Spinner::new(Spinners::Arc, String::new());
            let total_size = WalkDir::new(path)
                .min_depth(1)
                .max_depth(100)
                .into_iter()
                .filter_map(|entry| entry.ok())
                .filter_map(|entry| entry.metadata().ok())
                .filter(|metadata| metadata.is_file())
                .fold(0, |acc, m| acc + m.len());

            spinner.stop_with_symbol(&stop_symbol);
            println!("##==>> Directory '{}' is {}", path.display(), ByteSize::b(total_size));
        } else if file_metadata.is_file() {
            println!("##==> Path '{}' Points to a File", path.display());
            println!("##==> Calculating Size of File ...");

            let mut spinner = Spinner::new(Spinners::Arc, String::new());
            println!("##==>> File '{}' is {}", path.display(), ByteSize::b(file_metadata.len()));
            spinner.stop_with_symbol(&stop_symbol);
        } else {
            println!("{}", "##==>>> Warning! Where did you even find this? Spit it out".red());
        }
    } else {
        eprintln!("{}{}", "##==>>>> ERROR: File Not Found: ".red(), path.display());
    }
    
    Ok(())
}

pub fn get_system_info() -> io::Result<()> {
    // Ensure List of Components, Network
    // Interfaces, Disks and Users are Instantiated
    let mut sys = System::new_all();

    // Update Information of System Struct
    sys.refresh_all();

    println!("#==>> System:");

    // Number of CPU Cores/Threads
    println!("#==> CPU Cores/Threads: '{}'", sys.cpus().len());

    // RAM + SWAP Information
    println!("#==> Total RAM: '{} bytes'", sys.total_memory());
    println!("#==> Used RAM: '{} bytes'", sys.used_memory());
    println!();

    // Display System Information
    match System::host_name() {
        Some(sys_host) => println!("#==> System Host Name: '{}'", sys_host),
        _ => println!("#==> System Host Name: 'Unknown'"),
    };
    match System::name() {
        Some(sys_type) => println!("#==> System Type: '{}'", sys_type),
        _ => println!("#==> System Type: 'Unknown'"),
    };
    match System::os_version() {
        Some(sys_os) => println!("#==> System OS Version: '{}'", sys_os),
        _ => println!("#==> System OS Version: 'Unknown'"),
    };
    match System::kernel_version() {
        Some(sys_kernel) => println!("#==> System Kernel Version: '{}'", sys_kernel),
        _ => println!("#==> System Kernel Version: 'Unknown'"),
    };
    println!();

    // Display Disks Information
    println!("#==>> Disks:");
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        println!("##==> Disk: '{}'", disk.name().to_str().unwrap_or("Unknown"));
        println!("##==> Disk Mounted On: '{}'", disk.mount_point().display());
        println!("##==> Disk Type: '{}'", disk.kind());
        println!(
            "##==> Disk File System Type: '{}'",
            disk.file_system().to_str().unwrap_or("Unknown"),
        );
        println!(
            "##==> Disk Space Available: '{} B / {} B'",
            disk.available_space(),
            disk.total_space(),
        );
        println!("##==> Disk Removable: '{}'", disk.is_removable());
        println!();
    }

    // Network Interfaces
    let networks = Networks::new_with_refreshed_list();
    println!("#==>> Networks:");
    for (interface_name, data) in &networks {
        println!(
            "#==> '{}: {} B (Down) / {} B (Up)'",
            interface_name,
            data.total_received(),
            data.total_transmitted(),
        );
    }
    println!();

    // Components Temperature
    let components = Components::new_with_refreshed_list();
    if components.len() > 0 {
        println!("#==>> Components:");
        for component in &components {
            match component.critical() {
                Some(component_critical) => {
                    println!(
                        "#==> '{}: {:.1}°C (Max: {:.1}°C, Critical: {:.1}°C)'",
                        component.label(),
                        component.temperature(),
                        component.max(),
                        component_critical,
                    );
                },
                _ => {
                    println!(
                        "#==> '{}: {:.1}°C (Max: {:.1}°C)'",
                        component.label(),
                        component.temperature(),
                        component.max(),
                    );
                },
            };
        }
    }

    Ok(())
}

// Camera Controller For Bevy
pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, run_camera_controller);
    }
}

pub const RADIANS_PER_DOT: f32 = 1.0 / 180.0;

#[derive(Component)]
pub struct CameraController {
    pub enabled: bool,
    pub initialized: bool,
    pub sensitivity: f32,
    pub key_forward: KeyCode,
    pub key_back: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub key_run: KeyCode,
    pub mouse_key_cursor_grab: MouseButton,
    pub keyboard_key_toggle_cursor_grab: KeyCode,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub friction: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub velocity: Vec3,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            enabled: true,
            initialized: false,
            sensitivity: 1.0,
            key_forward: KeyCode::KeyW,
            key_back: KeyCode::KeyS,
            key_left: KeyCode::KeyA,
            key_right: KeyCode::KeyD,
            key_up: KeyCode::KeyE,
            key_down: KeyCode::KeyZ,
            key_run: KeyCode::ShiftLeft,
            mouse_key_cursor_grab: MouseButton::Left,
            keyboard_key_toggle_cursor_grab: KeyCode::KeyM,
            walk_speed: 5.0,
            run_speed: 15.0,
            friction: 0.5,
            pitch: 0.0,
            yaw: 0.0,
            velocity: Vec3::ZERO,
        }
    }
}

impl fmt::Display for CameraController {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Freecam Controls:
            Mouse\t - Move Camera Orientation
            {:?}\t - Hold to Grab Cursor
            {:?}\t - Toggle Cursor Grab
            {:?} & {:?}\t - Fly Forwards & Backwards
            {:?} & {:?}\t - Fly Sideways Left & Right
            {:?} & {:?}\t - Fly Up & Down
            {:?}\t - Fly Faster While Held",
            self.mouse_key_cursor_grab,
            self.keyboard_key_toggle_cursor_grab,
            self.key_forward,
            self.key_back,
            self.key_left,
            self.key_right,
            self.key_up,
            self.key_down,
            self.key_run,
        )
    }
}

#[allow(clippy::too_many_arguments)]
fn run_camera_controller(
    time: Res<Time>,
    mut windows: Query<&mut Window>,
    mut mouse_events: EventReader<MouseMotion>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut toggle_cursor_grab: Local<bool>,
    mut mouse_cursor_grab: Local<bool>,
    mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
) {
    let dt = time.delta_seconds();

    if let Ok((mut transform, mut controller)) = query.get_single_mut() {
        if !controller.initialized {
            let (yaw, pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
            controller.yaw = yaw;
            controller.pitch = pitch;
            controller.initialized = true;
            info!("{}", *controller);
        }
        if !controller.enabled {
            mouse_events.clear();
            return
        }

        // Handle Key Input
        let mut axis_input = Vec3::ZERO;
        if key_input.pressed(controller.key_forward) {
            axis_input.z += 1.0;
        }
        if key_input.pressed(controller.key_back) {
            axis_input.z -= 1.0;
        }
        if key_input.pressed(controller.key_right) {
            axis_input.x += 1.0;
        }
        if key_input.pressed(controller.key_left) {
            axis_input.x -= 1.0;
        }
        if key_input.pressed(controller.key_up) {
            axis_input.y += 1.0;
        }
        if key_input.pressed(controller.key_down) {
            axis_input.y -= 1.0;
        }

        let mut cursor_grab_change = false;
        if key_input.just_pressed(controller.keyboard_key_toggle_cursor_grab) {
            *toggle_cursor_grab = !*toggle_cursor_grab;
            cursor_grab_change = true;
        }
        if mouse_button_input.just_pressed(controller.mouse_key_cursor_grab) {
            *mouse_cursor_grab = true;
            cursor_grab_change = true;
        }
        if mouse_button_input.just_released(controller.mouse_key_cursor_grab) {
            *mouse_cursor_grab = false;
            cursor_grab_change = true;
        }
        let cursor_grab = *mouse_cursor_grab || *toggle_cursor_grab;

        // Apply movement update
        if axis_input != Vec3::ZERO {
            let max_speed = if key_input.pressed(controller.key_run) {
                controller.run_speed
            } else {
                controller.walk_speed
            };
            controller.velocity = axis_input.normalize() * max_speed;
        } else {
            let friction = controller.friction.clamp(0.0, 1.0);
            controller.velocity *= 1.0 - friction;
            if controller.velocity.length_squared() < 1e-6 {
                controller.velocity = Vec3::ZERO;
            }
        }
        let forward = *transform.forward();
        let right = *transform.right();
        transform.translation += controller.velocity.x * dt * right
            + controller.velocity.y * dt * Vec3::Y
            + controller.velocity.z * dt * forward;

        // Handle cursor grab
        if cursor_grab_change {
            if cursor_grab {
                for mut window in &mut windows {
                    if !window.focused {
                        continue;
                    }

                    window.cursor.grab_mode = CursorGrabMode::Locked;
                    window.cursor.visible = false;
                }
            } else {
                for mut window in &mut windows {
                    window.cursor.grab_mode = CursorGrabMode::None;
                    window.cursor.visible = true;
                }
            }
        }

        // Handle mouse input
        let mut mouse_delta = Vec2::ZERO;
        if cursor_grab {
            for mouse_event in mouse_events.read() {
                mouse_delta += mouse_event.delta;
            }
        } else {
            mouse_events.clear();
        }

        if mouse_delta != Vec2::ZERO {
            // Apply look update
            controller.pitch = (controller.pitch
                - mouse_delta.y * RADIANS_PER_DOT * controller.sensitivity)
                .clamp(-PI / 2., PI / 2.);
            controller.yaw -= mouse_delta.x * RADIANS_PER_DOT * controller.sensitivity;
            transform.rotation =
                Quat::from_euler(EulerRot::ZYX, 0.0, controller.yaw, controller.pitch);
        }
    }
}
