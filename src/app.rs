use crate::utils::{ModeError, ModeKind, PathTypeOutput, get_app_mode, get_template_path};
use std::env;

pub struct App {
    pub args: Vec<String>,
    pub mode: ModeKind,
    pub template_path: Option<String>,
    pub template_keys: Option<Vec<(String, String)>>,
}

impl App {
    pub fn new() -> Self {
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
        };
    }
    pub fn run(&mut self) {
        self.template_path = Some(String::new());
        match self.mode {
            ModeKind::TemplateAdd => match get_template_path(&self.args) {
                PathTypeOutput::TomlFile(path) => {
                    println!("path is: {}", path)
                }
                PathTypeOutput::Error(err) => {
                    eprintln!("{}", err);
                }
            },
            ModeKind::Generate => {}
        }
    }
}
