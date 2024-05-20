use std::fmt;

use sea_orm::DeriveModel;
use sea_orm_newtype::DeriveNewType;
use serde::{Deserialize, Serialize};

pub mod raw;
use raw::{forum::Entity as ForumEntity, thread::Entity as ThreadEntity};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveNewType)]
#[serde(transparent)]
#[sea_orm_newtype(from_into = "i32")]
pub struct ForumKey(u32);

impl fmt::Display for ForumKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl From<i32> for ForumKey {
    fn from(key: i32) -> ForumKey {
        ForumKey(key as _)
    }
}

impl From<ForumKey> for i32 {
    fn from(key: ForumKey) -> i32 {
        key.0 as _
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveNewType)]
#[serde(transparent)]
#[sea_orm_newtype(from_into = "i64")]
pub struct ThreadKey(u64);

impl ThreadKey {
    pub fn new_random() -> Self {
        Self(rand::random())
    }
}

impl fmt::Display for ThreadKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl From<i64> for ThreadKey {
    fn from(key: i64) -> ThreadKey {
        ThreadKey(key as _)
    }
}

impl From<ThreadKey> for i64 {
    fn from(key: ThreadKey) -> i64 {
        key.0 as _
    }
}

#[derive(Clone, Debug, DeriveModel)]
#[sea_orm(entity = ForumEntity)]
pub struct Forum {
    pub id: ForumKey,
    pub title: String,
    pub parent: Option<ForumKey>,
}

impl From<Forum> for raw::forum::Model {
    fn from(value: Forum) -> Self {
        Self {
            id: value.id.into(),
            title: value.title,
            parent: value.parent.map(Into::into),
        }
    }
}

#[derive(Clone, Debug, DeriveModel)]
#[sea_orm(entity = ThreadEntity)]
pub struct Thread {
    pub id: ThreadKey,
    pub forum: ForumKey,
    pub title: String,
}
