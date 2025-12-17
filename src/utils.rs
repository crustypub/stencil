use directories::ProjectDirs;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

pub enum ModeKind {
    TemplateAdd,
    Generate,
}

#[derive(Debug)]
pub enum ModeError {
    NoArguments,
    InvalidMode(String),
}

pub enum PathTypeOutput {
    TomlFile(String),
    Error(String),
}

pub fn get_app_mode(args: &Vec<String>) -> Result<ModeKind, ModeError> {
    if args.len() <= 1 {
        return Err(ModeError::NoArguments);
    }

    match args[1].as_str() {
        "-a" => Ok(ModeKind::TemplateAdd),
        "-g" => Ok(ModeKind::Generate),
        invalid => Err(ModeError::InvalidMode(invalid.to_string())),
    }
}

pub fn get_template_path(args: &Vec<String>) -> PathTypeOutput {
    if args.len() > 2 {
        let path = Path::new(&args[2]);

        match fs::metadata(path) {
            Ok(metadata) => {
                if metadata.is_dir() {
                    return PathTypeOutput::Error("Error: this is directory path".to_string());
                } else if metadata.is_file() {
                    if path.extension().map_or(false, |ext| ext == "toml") {
                        return PathTypeOutput::TomlFile(args[2].clone());
                    } else {
                        return PathTypeOutput::Error("Error: isn't .toml file".to_string());
                    }
                } else {
                    return PathTypeOutput::Error("Error: path didn't exist".to_string());
                }
            }
            Err(_e) => {
                return PathTypeOutput::Error(_e.to_string());
            }
        }
    }
    return PathTypeOutput::Error("Error: you have to write path to config .toml file".to_string());
}

pub fn save_config_file(original_path: &Path) -> io::Result<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "mycompany", "myapp") {
        let config_dir = proj_dirs.config_dir();

        fs::create_dir_all(config_dir)?;

        let file_name = original_path
            .file_name()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid file path"))?;

        let target_path = config_dir.join(file_name);

        fs::copy(original_path, &target_path)?;

        Ok(target_path)
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Could not determine config directory",
        ))
    }
}
