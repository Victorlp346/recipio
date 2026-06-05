use async_trait::async_trait;
use bcrypt;
use recipio_core::identity::hasher::PasswordHasher;
use recipio_core::identity::user::{HashedPassword, UnhashedPassword};
use recipio_core::{RecipioError, RecipioResult};

#[derive(Clone)]
pub struct BcryptHasher;

#[async_trait]
impl PasswordHasher for BcryptHasher {
    async fn hash(&self, password: &UnhashedPassword) -> RecipioResult<HashedPassword> {
        let hash_str = bcrypt::hash(password.as_ref(), bcrypt::DEFAULT_COST)
            .map_err(|_| RecipioError::HashingError)?;
        Ok(HashedPassword::from(hash_str))
    }

    async fn verify(
        &self,
        password: &UnhashedPassword,
        hashed: &HashedPassword,
    ) -> RecipioResult<bool> {
        bcrypt::verify(password.as_ref(), hashed.as_ref()).map_err(|_| RecipioError::HashingError)
    }
}
