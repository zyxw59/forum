use std::borrow::Cow;

use askama_actix::Template;
use entity::{raw::post::Model as Post, raw::thread::Model as Thread, Forum, ForumKey};

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    pub title: Cow<'static, str>,
    pub id: Option<ForumKey>,
    pub forums: Vec<Forum>,
    pub threads: Vec<Thread>,
}

#[derive(Template)]
#[template(path = "new-forum.html")]
pub struct NewForum {
    pub parent: Option<ForumKey>,
}

#[derive(Template)]
#[template(path = "post.html")]
pub struct NewPost {
    pub parent: ThreadOrForum,
}

pub enum ThreadOrForum {
    Thread(Thread),
    Forum(Forum),
}

impl ThreadOrForum {
    pub fn title(&self) -> &str {
        match self {
            Self::Thread(thread) => &thread.title,
            Self::Forum(forum) => &forum.title,
        }
    }
}

impl From<Forum> for ThreadOrForum {
    fn from(forum: Forum) -> Self {
        Self::Forum(forum)
    }
}

impl From<Thread> for ThreadOrForum {
    fn from(thread: Thread) -> Self {
        Self::Thread(thread)
    }
}

#[derive(Template)]
#[template(path = "thread.html")]
pub struct ViewThread {
    pub thread: Thread,
    pub posts: Vec<Post>,
}
