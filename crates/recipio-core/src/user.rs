use crate::{Id, RecipioError, RepoResult};
use async_trait::async_trait;
use derive_more::{Display, From};
use std::{fmt::Debug, str::FromStr};

#[derive(Display, Clone, Debug)]
pub struct Username(String);

impl FromStr for Username {
    type Err = RecipioError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.len() < 3 {
            return Err(RecipioError::ParsingError {
                value: value.to_string(),
                target: "Username (min 3 chars)".to_string(),
            });
        }
        Ok(Self(value.to_string()))
    }
}

#[derive(Display, Clone, Debug)]
pub struct Email(String);

impl FromStr for Email {
    type Err = RecipioError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if !value.contains('@') {
            return Err(RecipioError::ParsingError {
                value: value.to_string(),
                target: "Email (must contain @)".to_string(),
            });
        }
        Ok(Self(value.to_string()))
    }
}

#[derive(From, Clone)]
pub struct HashedPassword(String);

impl Debug for HashedPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("HashedPassword(*****)")
    }
}

//TODO: Add password rule validations
#[derive(From, Clone)]
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

#[derive(Debug, Clone)]
pub struct CreateUserDTO {
    username: Username,
    email: Email,
    password: UnhashedPassword,
}

#[derive(Debug, Clone)]
pub struct RetrieveUserDTO {
    id: Id<User>,
    username: Username,
    email: Email,
}

#[async_trait]
pub trait UserRepository {
    async fn add(user: User) -> RepoResult<User>;
}
