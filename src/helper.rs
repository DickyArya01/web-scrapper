use std::{fs, path::PathBuf};

use directories::ProjectDirs;

pub fn get_project_path() -> Option<PathBuf> {

    let project_dir = ProjectDirs::from("com", "Diary", "WebScrapper")?;

    let data_dir = project_dir.data_dir();

    if !data_dir.exists() {
       fs::create_dir_all(data_dir).ok()?;
    }

    Some(data_dir.to_path_buf())
}