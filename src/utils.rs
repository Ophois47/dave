use std::io;
use std::fs;
use std::path::Path;
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

    Ok(())
}
