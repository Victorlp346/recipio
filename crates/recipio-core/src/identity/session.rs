use async_trait::async_trait;
use nutype::nutype;
use thiserror::Error;
use time::{Duration, OffsetDateTime};

use crate::{Id, RecipioError, RepoResult, identity::user::User};

const DEFAULT_SESSION_DURATION_IN_DAYS: i64 = 7;

#[derive(Debug, Clone)]
pub struct Session {
    id: Id<Session>,
    token: TokenHash,
    user_id: Id<User>,
    creation_date: CreationDate,
    expiration_date: ExpirationDate,
    is_revoked: bool,
}

impl Session {
    pub fn new(user_id: Id<User>, token: TokenHash) -> Self {
        Self {
            id: Id::new(),
            token,
            user_id,
            creation_date: CreationDate::default(),
            expiration_date: ExpirationDate::default(),
            is_revoked: false,
        }
    }

    pub fn id(&self) -> Id<Session> {
        self.id
    }

    pub fn token(&self) -> &TokenHash {
        &self.token
    }

    pub fn user_id(&self) -> &Id<User> {
        &self.user_id
    }

    pub fn is_active(&self) -> bool {
        if self.is_revoked {
            return false;
        }

        let now = OffsetDateTime::now_utc();
        now < *self.expiration_date.as_ref()
    }

    pub fn revoke(&mut self) {
        self.is_revoked = true;
    }
}

#[nutype(derive(Debug, Display, Clone, TryFrom, Into, Default), default = OffsetDateTime::now_utc())]
pub struct CreationDate(OffsetDateTime);

#[nutype(
    derive(Debug, Display, Clone, Copy, AsRef, TryFrom, Into, Default),
    default = OffsetDateTime::now_utc() + Duration::days(DEFAULT_SESSION_DURATION_IN_DAYS)
)]
pub struct ExpirationDate(OffsetDateTime);

#[nutype(
    sanitize(trim),
    validate(len_char_min = 64, len_char_max = 64),
    derive(Debug, Clone, Into, TryFrom, AsRef, PartialEq, Eq)
)]
pub struct TokenHash(String);

#[async_trait]
pub trait SessionRepository {
    async fn add(&self, session: Session) -> RepoResult<Session>;
    async fn retrieve_by_id(&self, id: &Id<Session>) -> RepoResult<Option<Session>>;
}

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("user doesn't exists")]
    UserDoesNotExists,
    #[error("password provided is incorrect")]
    IncorrectPassword,
    #[error("token hash is invalid")]
    InvalidTokenHash(#[from] TokenHashError),
    #[error("invalid session token")]
    InvalidSession,
}

impl From<TokenHashError> for RecipioError {
    fn from(err: TokenHashError) -> Self {
        RecipioError::Session(SessionError::InvalidTokenHash(err))
    }
}
