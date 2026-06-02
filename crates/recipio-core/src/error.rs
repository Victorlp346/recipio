use std::fmt;

use thiserror::Error;

use crate::{session::SessionError, user::UserError};

#[derive(Debug, Error)]
pub enum RecipioError {
    #[error("repository error")]
    Repo(#[from] RepoError),
    #[error("user error")]
    User(#[from] UserError),
    #[error("session error")]
    Session(#[from] SessionError),
    #[error("impossible to parse {value:?} to {target:?}")]
    ParsingError { value: String, target: String },
    #[error("password hashing error")]
    HashingError,
    #[error("user is not authorized to perform this action")]
    Unauthorized,
    #[error("user is already logged in")]
    AlreadyAuthenticated,
    #[error("unknown app error")]
    Unknown,
}

pub type RecipioResult<T> = Result<T, RecipioError>;

#[derive(Debug, Clone, Default)]
pub struct Reason(pub Option<String>);

impl fmt::Display for Reason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(text) = &self.0 {
            write!(f, ": {text}")
        } else {
            Ok(())
        }
    }
}

impl<T: Into<String>> From<T> for Reason {
    fn from(text: T) -> Self {
        Reason(Some(text.into()))
    }
}

#[derive(Debug, Error)]
pub enum RepoError {
    #[error("Unknown repository error")]
    UnknownError,
}

pub type RepoResult<T> = Result<T, RepoError>;
