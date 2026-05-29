use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use bon::bon;
use recipio_core::{
    Id, RepoResult,
    session::{Session, SessionRepository},
};
use tokio::sync::RwLock;

#[derive(Clone, Debug)]
pub struct SessionInMemoryRepo {
    sessions: Arc<RwLock<HashMap<Id<Session>, Session>>>,
}

#[bon]
impl SessionInMemoryRepo {
    #[builder]
    pub fn new(sessions: Option<Arc<RwLock<HashMap<Id<Session>, Session>>>>) -> Self {
        Self {
            sessions: sessions.unwrap_or(Arc::new(RwLock::new(HashMap::new()))),
        }
    }
}

#[async_trait]
impl SessionRepository for SessionInMemoryRepo {
    async fn add(&self, session: Session) -> RepoResult<Session> {
        self.sessions
            .write()
            .await
            .insert(session.id(), session.clone());
        dbg!(self);
        Ok(session)
    }

    async fn retrieve_by_id(&self, id: &Id<Session>) -> RepoResult<Option<Session>> {
        Ok(self.sessions.read().await.get(id).cloned())
    }
}
