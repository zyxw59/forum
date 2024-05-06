use askama_actix::Template;
use entity::forum::Model as Forum;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    pub forums: Vec<Forum>,
}
