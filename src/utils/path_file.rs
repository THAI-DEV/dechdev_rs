use std::{env, path};

pub fn get_current_path() -> String {
    let current_dir = env::current_dir().unwrap();
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

pub fn locate_file_from_path(
    file_name: &str,
    primary_path: &str,
    secondary_path: &str,
    display_info: bool,
) -> String {
    let file_path = {
        let cwd_candidate = path::Path::new(&primary_path).join(file_name);
        if cwd_candidate.exists() {
            cwd_candidate.to_string_lossy().to_string()
        } else {
            let exe_candidate = path::Path::new(&secondary_path).join(file_name);
            if exe_candidate.exists() {
                exe_candidate.to_string_lossy().to_string()
            } else {
                String::new() // Return an empty string or handle the fallback case
            }
        }
    };

    if display_info {
        let file_path_without_name = path::Path::new(&file_path)
            .parent()
            .unwrap_or_else(|| path::Path::new(""))
            .to_string_lossy()
            .to_string();
        println!("Use {} from {}", file_name, file_path_without_name);
    }

    file_path
}
