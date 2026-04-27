mod task;
use task::{EmailTask, LogTask, Task};

enum SystemState {
    Idle,
    Processing(Box<dyn Task>), 
    Error(String),
}

fn main() {
    println!("Task Engine Started");

    // Start the system in an Idle state
    let mut current_state = SystemState::Idle;

    let state_pointer: &mut SystemState = &mut current_state;

    *state_pointer = SystemState::Processing(Box::new(EmailTask {
        recipient: String::from("admin@example.com"),
    }));

    match current_state {
        SystemState::Idle => {
            println!("System is waiting for work.");
        }
        SystemState::Processing(active_task) => {
            println!("System is busy. Executing task:");
            active_task.execute();
        }
        SystemState::Error(err_msg) => {
            println!("System halted with error: {}", err_msg);
        }
    }

    current_state = SystemState::Processing(Box::new(LogTask {
        message: String::from("User login detected."),
    }));

    if let SystemState::Processing(task) = current_state {
        task.execute();
    }
}