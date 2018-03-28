use serenity::prelude::SerenityError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum CommandError {
    Serenity(SerenityError),
    Generic(String),
}

impl From<SerenityError> for CommandError {
    fn from(err: SerenityError) -> CommandError {
        CommandError::Serenity(err)
    }
}

impl From<String> for CommandError {
    fn from(err: String) -> CommandError {
        CommandError::Generic(err)
    }
}

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            CommandError::Serenity(ref err) => write!(f, "{}", err),
            CommandError::Generic(ref err) => write!(f, "{}", err),
        }
    }
}

impl Error for CommandError {
    fn description(&self) -> &str {
        match *self {
            CommandError::Serenity(ref err) => err.description(),
            CommandError::Generic(ref str) => &str,
        }
    }
}
