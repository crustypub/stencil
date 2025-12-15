use crate::utils::{ModeError, ModeKind, get_app_mode};
use std::env;

pub struct App {
    args: Vec<String>,
    mode: ModeKind,
    template_path: Option<String>,
    template_keys: Option<Vec<(String, String)>>,
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
}
