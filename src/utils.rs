use crate::configmanager::ConfigManager;
use rand::{Rng, distr::Alphanumeric};
use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;

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

pub enum ConfigPathOutput {
    Path(String),
    Error(String),
}

pub enum GenerateFileOutput {
    Ok(String),
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

pub fn get_config_path(args: &Vec<String>, config_manager: &ConfigManager) -> ConfigPathOutput {
    if args.len() > 2 {
        let config_name = args[2].clone() + ".toml";

        if let Some(path) = config_manager.get_config_file(&config_name) {
            match fs::metadata(path) {
                Ok(metadata) => {
                    if metadata.is_file() {}
                    return ConfigPathOutput::Path(path.display().to_string());
                }
                Err(e) => {
                    return ConfigPathOutput::Error(e.to_string());
                }
            }
        } else {
            return ConfigPathOutput::Error(
                "Error: config with this name didn't exist, add it with -a flag".to_string(),
            );
        }
    }
    return ConfigPathOutput::Error(
        "Error: you have to write config name for file generate".to_string(),
    );
}
pub fn generate_file(
    keys: &Vec<(String, String)>,
    content: String,
    args: &Vec<String>,
) -> GenerateFileOutput {
    let current_dir = env::current_dir().unwrap().to_string_lossy().into_owned();
    let mut path = if args.len() - 3 == keys.len() {
        current_dir
    } else {
        let output = args.last().cloned().unwrap_or_else(|| current_dir);
        output
    };

    path = path + "/" + &get_random_string(4).as_str();

    match File::create(path) {
        Ok(_) => {
            println!("Файл создан");
            return GenerateFileOutput::Ok("hello".to_string());
        }
        Err(_e) => {
            return GenerateFileOutput::Error("Error: create file be failed".to_string());
        }
    }
}

pub fn parse_key_value_pairs(strings: &Vec<String>) -> Vec<(String, String)> {
    let mut result = Vec::new();

    for s in strings {
        let parts: Vec<&str> = s.splitn(2, '=').collect();

        if parts.len() == 2 && !parts[0].is_empty() {
            result.push((parts[0].to_string(), parts[1].to_string()));
        }
    }

    return result;
}

fn get_random_string(length: usize) -> String {
    let filename: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    return filename;
}
