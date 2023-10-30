use std::{fs, path::Path};

pub fn setup_filedir() {
    if !Path::new("files").exists() {
        fs::create_dir("files").expect("Failed to create files directory");
    }
}

pub fn create_user_dir(id: &str) {
    if !Path::new(format!("files/{}", id).as_str()).exists() {
        fs::create_dir(format!("files/{}", id)).expect("Failed to create user directory");
    }
}

pub fn get_user_files(id: &str) -> Vec<String> {
    create_user_dir(id);
    let mut files_vec = Vec::new();
    let files = fs::read_dir(format!("files/{}", id)).unwrap();
    for file in files {
        let file = file.unwrap();
        let file = file.file_name();
        let file = file.to_str().unwrap();
        files_vec.push(file.to_string());
    }
    files_vec
}
