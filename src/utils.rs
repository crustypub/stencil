pub enum ModeKind {
    ConfigAdd,
    Generate,
}

#[derive(Debug)]
pub enum ModeError {
    NoArguments,
    InvalidMode(String),
}

pub fn get_app_mode(args: &Vec<String>) -> Result<ModeKind, ModeError> {
    if args.len() <= 1 {
        return Err(ModeError::NoArguments);
    }

    match args[1].as_str() {
        "-a" => Ok(ModeKind::ConfigAdd),
        "-g" => Ok(ModeKind::Generate),
        invalid => Err(ModeError::InvalidMode(invalid.to_string())),
    }
}
