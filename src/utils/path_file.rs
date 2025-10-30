use std::{env, path};

pub fn current_path() -> String {
    let current_dir = env::current_dir().unwrap();
    current_dir.display().to_string().replace("\\", "/")
}

pub fn executable_path() -> String {
    let exe_path = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    exe_path.display().to_string().replace("\\", "/")
}

pub fn locate_file_path(file_name: &str, path_list: Vec<&str>) -> String {
    let mut file_path = String::new();
    for path in path_list {
        let candidate = path::Path::new(path).join(file_name);
        if candidate.exists() {
            file_path = candidate.to_string_lossy().to_string();
            break;
        }
    }

    path::Path::new(&file_path)
        .parent()
        .unwrap_or_else(|| path::Path::new(""))
        .to_string_lossy()
        .to_string()
}
