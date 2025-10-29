pub fn example_path_file() {
    let current_path = crate::utils::path_file::get_current_path();
    let execute_path = crate::utils::path_file::get_execute_path();

    println!("Current Path: {}", current_path);
    println!("Execute Path: {}", execute_path);
}
