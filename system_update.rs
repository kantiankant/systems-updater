use std::process::Command;
use std::fs;
use std::io::{self, Write};

fn main() {
    println!("System Package Updater");
    println!("======================\n");

    match detect_distro() {
        Some(distro) => {
            println!("Detected distribution: {}", distro);
            update_packages(&distro);
        }
        None => {
            println!("Could not detect distribution.");
            println!("Please update your system manually.");
        }
    }
}

fn detect_distro() -> Option<String> {
    // Try reading /etc/os-release first (standard across most modern distros)
    if let Ok(contents) = fs::read_to_string("/etc/os-release") {
        for line in contents.lines() {
            if line.starts_with("ID=") {
                let id = line.trim_start_matches("ID=").trim_matches('"');
                return Some(id.to_string());
            }
        }
    }

    // Fallback: check for specific release files
    if fs::metadata("/etc/debian_version").is_ok() {
        return Some("debian".to_string());
    }
    if fs::metadata("/etc/redhat-release").is_ok() {
        return Some("rhel".to_string());
    }
    if fs::metadata("/etc/arch-release").is_ok() {
        return Some("arch".to_string());
    }

    None
}

fn update_packages(distro: &str) {
    println!("\nUpdating packages...\n");

    let success = match distro {
        "ubuntu" | "debian" | "linuxmint" | "pop" => {
            update_apt()
        }
        "fedora" | "rhel" | "centos" | "rocky" | "almalinux" => {
            update_dnf()
        }
        "arch" | "manjaro" | "endeavouros" | "artix" => {
            update_pacman()
        }
        "opensuse" | "opensuse-tumbleweed" | "opensuse-leap" => {
            update_zypper()
        }
        "alpine" => {
            update_apk()
        }
        _ => {
            println!("Distribution '{}' is not directly supported.", distro);
            println!("Please update your system manually.");
            false
        }
    };

    if success {
        println!("\n✓ System update completed successfully!");
    } else {
        println!("\n✗ System update encountered errors.");
        println!("You may need to run this script with doas/sudo/root privileges.");
    }
}

fn update_apt() -> bool {
    println!("Using APT package manager...");
    
    // Update package list
    print!("Updating package list... ");
    io::stdout().flush().unwrap();
    let status = Command::new("apt")
        .arg("update")
        .status();
    
    match status {
        Ok(s) if s.success() => println!("✓"),
        _ => {
            println!("✗");
            return false;
        }
    }

    // Upgrade packages
    print!("Upgrading packages... ");
    io::stdout().flush().unwrap();
    let status = Command::new("apt")
        .arg("upgrade")
        .arg("-y")
        .status();
    
    match status {
        Ok(s) if s.success() => {
            println!("✓");
            true
        }
        _ => {
            println!("✗");
            false
        }
    }
}

fn update_dnf() -> bool {
    println!("Using DNF package manager...");
    
    let status = Command::new("dnf")
        .arg("upgrade")
        .arg("-y")
        .status();
    
    match status {
        Ok(s) if s.success() => true,
        _ => false,
    }
}

fn update_pacman() -> bool {
    println!("Using Pacman package manager...");
    
    let status = Command::new("pacman")
        .arg("-Syu")
        .arg("--noconfirm")
        .status();
    
    match status {
        Ok(s) if s.success() => true,
        _ => false,
    }
}

fn update_zypper() -> bool {
    println!("Using Zypper package manager...");
    
    let status = Command::new("zypper")
        .arg("update")
        .arg("-y")
        .status();
    
    match status {
        Ok(s) if s.success() => true,
        _ => false,
    }
}

fn update_apk() -> bool {
    println!("Using APK package manager...");
    
    // Update repository indexes
    let update_status = Command::new("apk")
        .arg("update")
        .status();
    
    if !matches!(update_status, Ok(s) if s.success()) {
        return false;
    }

    // Upgrade packages
    let status = Command::new("apk")
        .arg("upgrade")
        .status();
    
    match status {
        Ok(s) if s.success() => true,
        _ => false,
    }
}

