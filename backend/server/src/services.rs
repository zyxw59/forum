use actix_web::{error, web, Responder, Result};
use entity::{forum, thread};
use sea_orm::{prelude::*, ActiveValue};
use serde::Deserialize;

use crate::AppState;

#[actix_web::get("/")]
pub async fn toplevel_forums(state: web::Data<AppState>) -> Result<impl Responder> {
    let forums: Vec<_> = forum::Entity::find()
        .filter(forum::Column::Parent.is_null())
        .all(&state.connection)
        .await
        .map_err(error::ErrorInternalServerError)?;
    Ok(templates::Index {
        forums,
        threads: Vec::new(),
    })
}

#[actix_web::get("/forum/{id}")]
pub async fn get_forum(state: web::Data<AppState>, id: web::Path<u32>) -> Result<impl Responder> {
    let id = *id as i32;
    let forums: Vec<_> = forum::Entity::find()
        .filter(forum::Column::Parent.eq(id))
        .all(&state.connection)
        .await
        .map_err(error::ErrorInternalServerError)?;
    let threads: Vec<_> = thread::Entity::find()
        .filter(thread::Column::Forum.eq(id))
        .all(&state.connection)
        .await
        .map_err(error::ErrorInternalServerError)?;
    Ok(templates::Index { forums, threads })
}

#[actix_web::get("/forum/new")]
pub async fn get_create_forum(query: web::Query<GetCreateForum>) -> Result<impl Responder> {
    Ok(templates::NewForum {
        parent: query.parent,
    })
}

#[derive(Debug, Deserialize)]
struct GetCreateForum {
    parent: Option<u32>,
}

#[actix_web::post("/forum/new")]
pub async fn post_create_forum(
    state: web::Data<AppState>,
    web::Form(query): web::Form<PostCreateForum>,
) -> Result<impl Responder> {
    let new_forum = forum::ActiveModel {
        id: ActiveValue::Set(rand::random()),
        title: ActiveValue::Set(query.forum_name),
        parent: ActiveValue::Set(query.parent.map(|id| id as i32)),
    }
    .insert(&state.connection)
    .await
    .map_err(error::ErrorInternalServerError)?;
    Ok(web::Redirect::to(format!("/forum/{}", new_forum.id as u32)).see_other())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct PostCreateForum {
    parent: Option<u32>,
    forum_name: String,
}
