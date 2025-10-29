pub fn get_current_path() -> String {
    let current_dir = std::env::current_dir().unwrap();
    current_dir.display().to_string().replace("\\", "/")
}

pub fn get_execute_path() -> String {
    let exe_path = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    exe_path.display().to_string().replace("\\", "/")
}
