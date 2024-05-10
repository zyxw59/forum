use askama_actix::Template;
use entity::{
    Forum, ForumKey,
    raw::thread::Model as Thread,
};

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    pub id: Option<ForumKey>,
    pub forums: Vec<Forum>,
    pub threads: Vec<Thread>,
}

#[derive(Template)]
#[template(path = "new-forum.html")]
pub struct NewForum {
    pub parent: Option<ForumKey>,
}
