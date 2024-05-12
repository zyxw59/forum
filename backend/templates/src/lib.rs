use std::borrow::Cow;

use askama_actix::Template;
use entity::{raw::thread::Model as Thread, Forum, ForumKey};

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
