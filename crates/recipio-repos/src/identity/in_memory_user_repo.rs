use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use bon::bon;
use recipio_core::identity::user::{User, UserRepository, Username};
use recipio_core::{Id, RepoResult};
use tokio::sync::RwLock;

#[derive(Clone, Debug)]
pub struct UserInMemoryRepo {
    users: Arc<RwLock<HashMap<Id<User>, User>>>,
}

#[bon]
impl UserInMemoryRepo {
    #[builder]
    pub fn new(users: Option<Arc<RwLock<HashMap<Id<User>, User>>>>) -> Self {
        Self {
            users: users.unwrap_or(Arc::new(RwLock::new(HashMap::new()))),
        }
    }
}

#[async_trait]
impl UserRepository for UserInMemoryRepo {
    async fn add(&self, user: User) -> RepoResult<User> {
        self.users.write().await.insert(user.id(), user.clone());
        dbg!(self);
        Ok(user)
    }

    async fn retrieve_by_id(&self, id: &Id<User>) -> RepoResult<Option<User>> {
        Ok(self.users.read().await.get(id).cloned())
    }

    async fn retrieve_by_username(&self, username: &Username) -> RepoResult<Option<User>> {
        Ok(self
            .users
            .read()
            .await
            .values()
            .find(|user| user.username() == username)
            .cloned())
    }
}
