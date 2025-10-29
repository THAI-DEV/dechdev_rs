use std::{env, path};

pub fn get_current_path() -> String {
    let current_dir = env::current_dir().unwrap();
    current_dir.display().to_string().replace("\\", "/")
}

pub fn get_executable_path() -> String {
    let exe_path = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    exe_path.display().to_string().replace("\\", "/")
}

/// Searches for a file in a list of directories and returns its full path and parent directory.
///
/// # Arguments
///
/// * `file_name` - The name of the file to locate.
/// * `path_list` - A vector of directory paths to search.
///
/// # Returns
///
/// A tuple containing:
/// - The full path with file name to the located file as a `String`. If the file is not found, this will be an empty string.
/// - The full path without file name to the located file as a `String`. If the file is not found, this will be an empty string.
///
/// ```
pub fn locate_file_in_path(file_name: &str, path_list: Vec<&str>) -> (String, String) {
    let mut file_path = String::new();
    for path in path_list {
        let candidate = path::Path::new(path).join(file_name);
        if candidate.exists() {
            file_path = candidate.to_string_lossy().to_string();
            break;
        }
    }

    let file_path_without_name = path::Path::new(&file_path)
        .parent()
        .unwrap_or_else(|| path::Path::new(""))
        .to_string_lossy()
        .to_string();

    (file_path, file_path_without_name)
}
