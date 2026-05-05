use std::env;

pub fn simulate_reading_env() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let name1 = env::var("NAME").unwrap_or(test_unwrap_or());
    let name2 = env::var("NAME").unwrap_or_else(|_| test_unwrap_or_else());

    println!("Hello, {}!", name1);
    println!("Hello2: {}", name2);
    println!("Version: {}", VERSION);
}

pub fn test_unwrap_or() -> String {
    println!("test_unwrap_or was called");
    "bahi".to_string()
}

pub fn test_unwrap_or_else() -> String {
    println!("test_unwrap_or_else was called");
    "bahi".to_string()
}
