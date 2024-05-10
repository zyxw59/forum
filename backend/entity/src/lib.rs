use std::fmt;

use serde::{Deserialize, Serialize};
use sea_orm_newtype::DeriveNewType;

pub mod raw;

use internal::Wrapper;

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

pub type Forum = internal::ForumModel<internal::Id>;
pub type ActiveForum = internal::ForumModel<internal::Active>;

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

impl ActiveForum {
    pub fn into_raw(self) -> raw::forum::ActiveModel {
        self.into()
    }
}

impl From<ActiveForum> for raw::forum::ActiveModel {
    fn from(value: ActiveForum) -> Self {
        Self {
            id: internal::Active::map(value.id, Into::into),
            title: value.title,
            parent: internal::Active::map(value.parent, |parent| parent.map(Into::into)),
        }
    }
}

mod internal {
    use sea_orm::{ActiveValue, Value};

    use super::ForumKey;

    pub struct ForumModel<W: Wrapper> {
        pub id: W::Wrapped<ForumKey>,
        pub title: W::Wrapped<String>,
        pub parent: W::Wrapped<Option<ForumKey>>,
    }

    impl<W: Wrapper> Default for ForumModel<W>
    where
        W::Wrapped<ForumKey>: Default,
        W::Wrapped<String>: Default,
        W::Wrapped<Option<ForumKey>>: Default,
    {
        fn default() -> Self {
            Self {
                id: Default::default(),
                title: Default::default(),
                parent: Default::default(),
            }
        }
    }

    pub trait Wrapper {
        type Wrapped<T>
        where
            Value: From<T>;

        fn map<T, U>(this: Self::Wrapped<T>, f: impl FnOnce(T) -> U) -> Self::Wrapped<U>
        where
            Value: From<T> + From<U>;
    }

    pub struct Id;

    impl Wrapper for Id {
        type Wrapped<T> = T where Value: From<T>;

        fn map<T, U>(this: Self::Wrapped<T>, f: impl FnOnce(T) -> U) -> Self::Wrapped<U>
        where
            Value: From<T> + From<U>,
        {
            f(this)
        }
    }

    pub struct Active;

    impl Wrapper for Active {
        type Wrapped<T> = ActiveValue<T> where Value: From<T>;

        fn map<T, U>(this: Self::Wrapped<T>, f: impl FnOnce(T) -> U) -> Self::Wrapped<U>
        where
            Value: From<T> + From<U>,
        {
            match this {
                ActiveValue::Set(val) => ActiveValue::Set(f(val)),
                ActiveValue::Unchanged(val) => ActiveValue::Unchanged(f(val)),
                ActiveValue::NotSet => ActiveValue::NotSet,
            }
        }
    }
}
