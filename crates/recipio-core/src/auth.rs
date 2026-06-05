use crate::{
    Id,
    user::{Role, User},
};

#[derive(Clone, Debug)]
pub struct UserClaims {
    id: Id<User>,
    role: Role,
}

impl UserClaims {
    pub fn id(&self) -> &Id<User> {
        &self.id
    }

    pub fn role(&self) -> &Role {
        &self.role
    }
}

impl From<User> for UserClaims {
    fn from(user: User) -> Self {
        Self {
            id: user.id(),
            role: user.role().to_owned(),
        }
    }
}
