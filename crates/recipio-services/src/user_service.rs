use bcrypt::{DEFAULT_COST, hash};
use recipio_core::{
    CreateUserDTO, Id, RecipioError, RecipioResult, RetrieveUserDTO, User, UserRepository,
};

#[derive(Clone)]
pub struct UserService<R> {
    repo: R,
}

impl<R> UserService<R>
where
    R: UserRepository + Send + Sync,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn register(&self, data: CreateUserDTO) -> RecipioResult<RetrieveUserDTO> {
        let new_id: Id<User> = Id::new();

        let Ok(hashed_password) = hash(data.password.as_ref(), DEFAULT_COST) else {
            return Err(RecipioError::HashingError);
        };

        let new_user = User::new(new_id, data.username, data.email, hashed_password.into());

        let user = self.repo.add(new_user).await?;

        Ok(RetrieveUserDTO::from(user))
    }
}
