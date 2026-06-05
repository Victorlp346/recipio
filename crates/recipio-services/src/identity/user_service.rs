use std::sync::Arc;

use recipio_core::identity::auth::UserClaims;
use recipio_core::identity::hasher::PasswordHasher;
use recipio_core::identity::user::{Email, Role, UnhashedPassword, User, UserRepository, Username};
use recipio_core::{Id, RecipioError, RecipioResult};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct UserResponseDto {
    pub id: Id<User>,
    pub username: String,
    pub email: String,
}

impl From<User> for UserResponseDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id(),
            username: user.username().to_string(),
            email: user.email().to_string(),
        }
    }
}

pub type DynUserRepository = Arc<dyn UserRepository + Send + Sync>;
pub type DynPasswordHasher = Arc<dyn PasswordHasher + Send + Sync>;

#[derive(Clone)]
pub struct UserService {
    repo: DynUserRepository,
    password_hasher: DynPasswordHasher,
}

impl UserService {
    pub fn new(repo: DynUserRepository, password_hasher: DynPasswordHasher) -> Self {
        Self {
            repo,
            password_hasher,
        }
    }

    pub async fn register(
        &self,
        username: Username,
        email: Email,
        unhashed_password: UnhashedPassword,
    ) -> RecipioResult<UserResponseDto> {
        let new_id: Id<User> = Id::new();

        let hashed_password = self.password_hasher.hash(&unhashed_password).await?;
        let new_user = User::new(new_id, username, email, hashed_password);

        let user = self.repo.add(new_user).await?;

        Ok(UserResponseDto::from(user))
    }

    pub async fn get_claims_by_id(&self, id: &Id<User>) -> RecipioResult<Option<UserClaims>> {
        Ok(self.repo.retrieve_by_id(id).await?.map(Into::into))
    }

    pub async fn get_by_id(
        &self,
        id: &Id<User>,
        requester: &UserClaims,
    ) -> RecipioResult<Option<UserResponseDto>> {
        if requester.id() != id {
            if *requester.role() < Role::Admin {
                return Err(RecipioError::Unauthorized);
            }
        }
        Ok(self
            .repo
            .retrieve_by_id(id)
            .await?
            .map(UserResponseDto::from))
    }
}
