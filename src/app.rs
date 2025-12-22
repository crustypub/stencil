use crate::configmanager::ConfigManager;
use crate::utils::{
    self, ConfigPathOutput, ModeError, ModeKind, PathTypeOutput, get_app_mode, get_config_path,
    get_template_path,
};
use std::env;
use std::fs;
use std::path::Path;

pub struct App {
    pub args: Vec<String>,
    pub mode: ModeKind,
    pub template_path: Option<String>,
    pub template_keys: Option<Vec<(String, String)>>,
    pub config_manager: ConfigManager,
}

impl App {
    pub fn new() -> Self {
        let config_manager = ConfigManager::new().unwrap();
        let args: Vec<String> = env::args().collect();
        let mode = get_app_mode(&args).unwrap_or_else(|err| {
            match err {
                ModeError::NoArguments => {
                    eprintln!("Error: No arguments specified");
                    eprintln!("Usage: program <-a|-g>");
                }
                ModeError::InvalidMode(mode) => {
                    eprintln!("Error: invalid mode '{}'", mode);
                    eprintln!("Valid modes: -a (add config), -g (generate)");
                }
            }
            std::process::exit(1);
        });
        return Self {
            mode,
            args,
            template_keys: None,
            template_path: None,
            config_manager,
        };
    }
    pub fn run(&mut self) {
        self.template_path = Some(String::new());
        match self.mode {
            ModeKind::TemplateAdd => match get_template_path(&self.args) {
                PathTypeOutput::TomlFile(path) => {
                    self.config_manager
                        .save_config_file(Path::new(&path))
                        .unwrap();
                    println!("Config saved successfully.");
                }
                PathTypeOutput::Error(err) => {
                    eprintln!("{}", err);
                }
            },
            ModeKind::Generate => match get_config_path(&self.args, &self.config_manager) {
                ConfigPathOutput::Path(path) => {
                    let content = fs::read_to_string(path).unwrap();
                    let config = toml::from_str(&content).unwrap();
                    println!("content: {:?}", content);
                }
                ConfigPathOutput::Error(error) => {
                    eprintln!("{}", error);
                }
            },
        }
    }
}
