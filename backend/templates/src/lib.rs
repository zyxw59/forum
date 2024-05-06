use askama_actix::Template;
use entity::forum::Model as Forum;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    pub forums: Vec<Forum>,
}

#[derive(Template)]
#[template(path = "new-forum.html")]
pub struct NewForum {
    pub parent: Option<i64>,
}
