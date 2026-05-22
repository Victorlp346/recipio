use recipio_core::hasher::PasswordHasher;
use recipio_core::session::{Session, SessionError, SessionRepository};
use recipio_core::user::{UnhashedPassword, UserRepository, Username};
use recipio_core::{Id, RecipioError, RecipioResult};
use serde::Deserialize;

#[derive(Clone)]
pub struct SessionService<UR, SR, H> {
    user_repo: UR,
    session_repo: SR,
    password_hasher: H,
}

impl<UR, SR, H> SessionService<UR, SR, H>
where
    UR: UserRepository + Send + Sync,
    SR: SessionRepository + Send + Sync,
    H: PasswordHasher + Send + Sync,
{
    pub fn new(user_repo: UR, session_repo: SR, password_hasher: H) -> Self {
        Self {
            user_repo,
            session_repo,
            password_hasher,
        }
    }

    pub async fn create_session(&self, data: LoginDto) -> RecipioResult<Id<Session>> {
        let username: Username = data.username.try_into()?;
        let unhashed_password: UnhashedPassword = data.password.try_into()?;

        let Some(user) = self.user_repo.retrieve_by_username(&username).await? else {
            return Err(RecipioError::Session(SessionError::UserDoesNotExists));
        };

        let is_valid = self
            .password_hasher
            .verify(&unhashed_password, user.password())
            .await?;

        if !is_valid {
            return Err(RecipioError::Session(SessionError::IncorrectPassword));
        }

        let session = Session::new(user.id());
        let stored_session = self.session_repo.add(session).await?;
        Ok(stored_session.id())
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginDto {
    pub username: String,
    pub password: String,
}
