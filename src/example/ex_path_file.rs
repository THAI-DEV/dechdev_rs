use crate::utils::path_file;

pub fn example_path_file() {
    let current_path = path_file::current_path();
    let executable_path = path_file::executable_path();

    println!("Current Path: {}", current_path);
    println!("Execute Path: {}", executable_path);
}
