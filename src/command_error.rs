use postgres;
use r2d2;
use serenity::prelude::SerenityError;

use std::env::VarError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum CommandError {
    Serenity(SerenityError),
    Postgres(postgres::Error),
    Env(VarError),
    R2d2(r2d2::Error),
    Generic(String),
}

impl From<SerenityError> for CommandError {
    fn from(err: SerenityError) -> CommandError {
        CommandError::Serenity(err)
    }
}

impl From<postgres::Error> for CommandError {
    fn from(err: postgres::Error) -> CommandError {
        CommandError::Postgres(err)
    }
}

impl From<VarError> for CommandError {
    fn from(err: VarError) -> CommandError {
        CommandError::Env(err)
    }
}

impl From<r2d2::Error> for CommandError {
    fn from(err: r2d2::Error) -> CommandError {
        CommandError::R2d2(err)
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
            CommandError::Postgres(ref err) => write!(f, "{}", err),
            CommandError::Env(ref err) => write!(f, "{}", err),
            CommandError::R2d2(ref err) => write!(f, "{}", err),
            CommandError::Generic(ref err) => write!(f, "{}", err),
        }
    }
}

impl Error for CommandError {
    fn description(&self) -> &str {
        match *self {
            CommandError::Serenity(ref err) => err.description(),
            CommandError::Postgres(ref err) => err.description(),
            CommandError::Env(ref err) => err.description(),
            CommandError::R2d2(ref err) => err.description(),
            CommandError::Generic(ref str) => &str,
        }
    }
}
