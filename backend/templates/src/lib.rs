use askama_actix::Template;
use entity::{
    forum::Model as Forum,
    thread::Model as Thread,
};

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    pub forums: Vec<Forum>,
    pub threads: Vec<Thread>,
}

#[derive(Template)]
#[template(path = "new-forum.html")]
pub struct NewForum {
    pub parent: Option<u32>,
}
