use std::sync::Arc;

use recipio_core::hasher::PasswordHasher;
use recipio_core::user::{Email, UnhashedPassword, User, UserRepository, Username};
use recipio_core::{Id, RecipioResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterUserDto {
    pub username: String,
    pub email: String,
    pub password: String,
}

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

    pub async fn register(&self, data: RegisterUserDto) -> RecipioResult<UserResponseDto> {
        let username: Username = data.username.try_into()?;
        let email: Email = data.email.try_into()?;
        let unhashed_password: UnhashedPassword = data.password.try_into()?;

        let new_id: Id<User> = Id::new();

        let hashed_password = self.password_hasher.hash(&unhashed_password).await?;
        let new_user = User::new(new_id, username, email, hashed_password);

        let user = self.repo.add(new_user).await?;

        Ok(UserResponseDto::from(user))
    }

    pub async fn get_by_id(&self, id: &Id<User>) -> RecipioResult<Option<User>> {
        Ok(self.repo.retrieve_by_id(id).await?)
    }
}
