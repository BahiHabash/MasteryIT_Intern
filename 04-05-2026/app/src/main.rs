use std::io;

#[derive(Debug)]
enum SystemStatus {
    Online,
    Offline(String),
    Maintenance { technician: String, progress: u8 },
}

#[derive(Debug)]
enum Command {
    Activate,
    Deactivate,
    PrintMessage(String),
    SetSafetyLevel(u32),
}

fn main() {
    let code = get_input("Please enter an access code (integer):");

    let status = get_system_status(code);

    match status {
        Some(SystemStatus::Online) => {
            println!("System is fully operational.");
            process_command(Command::PrintMessage(String::from("Welcome back, Captain.")));
        }
        Some(SystemStatus::Offline(reason)) => {
            println!("System is DOWN. Reason: {}", reason);
        }
        Some(SystemStatus::Maintenance { technician, progress }) => {
            println!("System is under maintenance by {}. Progress: {}%", technician, progress);
            if progress < 50 {
                println!("Warning: Maintenance is still in early stages.");
            }
        }
        None => println!("Access Denied: No system found for code {}.", code),
    }
}

fn get_system_status(code: u32) -> Option<SystemStatus> {
    match code {
        100..=199 => Some(SystemStatus::Online),
        200..=299 => Some(SystemStatus::Offline(String::from("Fuel exhaustion"))),
        300..=399 => Some(SystemStatus::Maintenance {
            technician: String::from("Bahi"),
            progress: 42,
        }),
        _ => None,
    }
}

fn process_command(cmd: Command) {
    match cmd {
        Command::Activate => println!("Executing: [ACTIVATE ALL SYSTEMS]"),
        Command::Deactivate => println!("Executing: [DEACTIVATE ALL SYSTEMS]"),
        Command::PrintMessage(msg) => println!("COMMUNICATIONS: {}", msg),
        Command::SetSafetyLevel(level) => {
            match level {
                0 => println!("DANGER: Safety systems disabled!"),
                1..=5 => println!("Caution: Minimal safety protocols active."),
                _ => println!("Safety systems set to high."),
            }
        }
    }
}

fn get_input(prompt: &str) -> u32 {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    input.trim().parse::<u32>().expect("Invalid input")
}