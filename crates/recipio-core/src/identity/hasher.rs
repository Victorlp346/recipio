use async_trait::async_trait;

use crate::{
    RecipioResult,
    identity::user::{HashedPassword, UnhashedPassword},
};

#[async_trait]
pub trait PasswordHasher {
    async fn hash(&self, password: &UnhashedPassword) -> RecipioResult<HashedPassword>;
    async fn verify(
        &self,
        password: &UnhashedPassword,
        hashed_password: &HashedPassword,
    ) -> RecipioResult<bool>;
}
