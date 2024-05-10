use std::fmt;

use sea_orm_newtype::DeriveNewType;
use serde::{Deserialize, Serialize};

pub mod raw;

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

impl From<raw::forum::Model> for Forum {
    fn from(value: raw::forum::Model) -> Self {
        Self {
            id: value.id.into(),
            title: value.title,
            parent: value.parent.map(Into::into),
        }
    }
}
