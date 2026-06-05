use std::sync::Arc;

use rand::distr::{Alphanumeric, SampleString};
use recipio_core::identity::hasher::PasswordHasher;
use recipio_core::identity::session::{Session, SessionError, SessionRepository, TokenHash};
use recipio_core::identity::user::{UnhashedPassword, UserRepository, Username};
use recipio_core::{Id, RecipioError, RecipioResult};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub type DynUserRepository = Arc<dyn UserRepository + Send + Sync>;
pub type DynSessionRepository = Arc<dyn SessionRepository + Send + Sync>;
pub type DynPasswordHasher = Arc<dyn PasswordHasher + Send + Sync>;

#[derive(Clone)]
pub struct SessionService {
    user_repo: DynUserRepository,
    session_repo: DynSessionRepository,
    password_hasher: DynPasswordHasher,
}

impl SessionService {
    pub fn new(
        user_repo: DynUserRepository,
        session_repo: DynSessionRepository,
        password_hasher: DynPasswordHasher,
    ) -> Self {
        Self {
            user_repo,
            session_repo,
            password_hasher,
        }
    }

    pub async fn create_session(&self, data: LoginDto) -> RecipioResult<SessionCreatedDTO> {
        let username: Username = data.username.try_into()?;
        let unhashed_password: UnhashedPassword = data.password.try_into()?;

        let Some(user) = self.user_repo.retrieve_by_username(&username).await? else {
            let dummy_hash = "$2b$12$somevalidlookingdummyhashstringhere............."
                .try_into()
                .unwrap();
            let _ = self
                .password_hasher
                .verify(&unhashed_password, &dummy_hash)
                .await;
            return Err(RecipioError::Session(SessionError::UserDoesNotExists));
        };

        let is_valid = self
            .password_hasher
            .verify(&unhashed_password, user.password())
            .await?;

        if !is_valid {
            return Err(RecipioError::Session(SessionError::IncorrectPassword));
        }

        let unhashed_token = Alphanumeric.sample_string(&mut rand::rng(), 128);
        let mut hasher = Sha256::new();
        hasher.update(unhashed_token.as_bytes());
        let hashed_token: TokenHash = hex::encode(hasher.finalize()).try_into()?;

        let session = Session::new(user.id(), hashed_token);
        let stored_session = self.session_repo.add(session).await?;
        Ok(SessionCreatedDTO {
            session_id: stored_session.id(),
            token: unhashed_token,
        })
    }

    pub async fn validate_session(
        &self,
        id: &Id<Session>,
        unhashed_token: &str,
    ) -> RecipioResult<Session> {
        let Some(session) = self.session_repo.retrieve_by_id(id).await? else {
            return Err(RecipioError::Session(SessionError::InvalidSession));
        };
        let mut hasher = Sha256::new();
        hasher.update(unhashed_token);
        let hashed_provided_token: TokenHash = hex::encode(hasher.finalize()).try_into()?;

        if hashed_provided_token == session.token().to_owned() {
            return Ok(session);
        }
        Err(RecipioError::Session(SessionError::InvalidSession))
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginDto {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct SessionCreatedDTO {
    pub session_id: Id<Session>,
    pub token: String,
}
