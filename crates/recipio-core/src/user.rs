use crate::{Id, RecipioError, error::RepoResult};
use async_trait::async_trait;
use nutype::nutype;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use thiserror::Error;

#[nutype(
    sanitize(trim, lowercase),
    validate(len_char_min = 4, not_empty, len_char_max = 255),
    derive(Debug, Display, Clone, TryFrom, Into, FromStr, Serialize, Deserialize)
)]
pub struct Username(String);

#[nutype(
    sanitize(trim, lowercase),
    validate(len_char_max = 255, regex = r"\S+@\S+\.\S+"),
    derive(Debug, Display, Clone, TryFrom, Into, FromStr, Serialize, Deserialize)
)]
pub struct Email(String);

#[nutype(sanitize(trim), derive(Clone, Into, FromStr, From))]
pub struct HashedPassword(String);

impl Debug for HashedPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("HashedPassword(*****)")
    }
}

//TODO: Add password rule validations
#[nutype(
    sanitize(trim),
    validate(len_char_max = 255, len_char_min = 5),
    derive(Clone, Into, TryFrom, FromStr, AsRef, Deserialize)
)]
pub struct UnhashedPassword(String);

impl Debug for UnhashedPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("UnhashedPassword(****)")
    }
}

#[derive(Debug, Clone)]
pub struct User {
    id: Id<User>,
    username: Username,
    email: Email,
    password: HashedPassword,
}

impl User {
    pub fn new(id: Id<User>, username: Username, email: Email, password: HashedPassword) -> Self {
        Self {
            id,
            username,
            email,
            password,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserDTO {
    pub username: Username,
    pub email: Email,
    pub password: UnhashedPassword,
}

#[derive(Debug, Clone, Serialize)]
pub struct RetrieveUserDTO {
    pub id: Id<User>,
    pub username: Username,
    pub email: Email,
}

impl From<User> for RetrieveUserDTO {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
        }
    }
}

#[async_trait]
pub trait UserRepository {
    async fn add(&self, user: User) -> RepoResult<User>;
}

#[derive(Debug, Error)]
pub enum UserError {
    #[error("the email provided is not valid")]
    InvalidEmail(#[from] EmailError),
    #[error("the username provided is not valid")]
    InvalidUsername(#[from] UsernameError),
    #[error("the passowrd provided is not valid")]
    InvalidPassword(#[from] UnhashedPasswordError),
}

impl From<UsernameError> for RecipioError {
    fn from(err: UsernameError) -> Self {
        RecipioError::User(UserError::InvalidUsername(err))
    }
}
impl From<EmailError> for RecipioError {
    fn from(err: EmailError) -> Self {
        RecipioError::User(UserError::InvalidEmail(err))
    }
}
impl From<UnhashedPasswordError> for RecipioError {
    fn from(err: UnhashedPasswordError) -> Self {
        RecipioError::User(UserError::InvalidPassword(err))
    }
}
