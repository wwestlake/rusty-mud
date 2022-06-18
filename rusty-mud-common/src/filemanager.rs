extern crate directories;
use directories::{BaseDirs, UserDirs, ProjectDirs};
use std::{
    path::{ 
        Path,
        MAIN_SEPARATOR, PathBuf
    }
};

fn config_dir() -> PathBuf {
    if let Some(project_dirs) = ProjectDirs::from("com", "lagdaemon", "rustymud") {
        project_dirs.config_dir().to_owned()
    } else {
        Path::new("./data").to_owned()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_dir_returns_appropriate_path() {
        let p = config_dir();
        println!(
            "{:#?}", p
        );
        assert_eq!(p.to_str().unwrap(), "C:\\Users\\wwestlake\\AppData\\Roaming\\lagdaemon\\rustymud\\config");
    }
}


