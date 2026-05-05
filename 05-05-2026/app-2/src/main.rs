mod environment;
mod file;

use dotenvy::dotenv;

fn main() {
    load_env();
    println!("----------------");

    environment::simulate_reading_env();
    println!("----------------");

    let file_name = file_name_input();
    println!("----------------");

    let file_content = file::simulate_reading_files(&file_name);
    println!("File content:\n {:#?}", file_content);
}

fn load_env() {
    match dotenv() {
        Ok(_) => println!("Environment variables loaded"),
        Err(e) => panic!("Failed to load environment variables: {}", e),
    }
}

fn file_name_input() -> String {
    println!("Please enter a file name");
    let mut file_name = String::new();
    std::io::stdin()
        .read_line(&mut file_name)
        .expect("Failed to read line");
    file_name.trim().to_string()
}
