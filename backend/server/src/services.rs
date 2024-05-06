use actix_web::{error, web, Responder, Result};
use entity::forum;
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
    Ok(templates::Index { forums })
}

#[actix_web::get("/forum/new")]
pub async fn get_create_forum(query: web::Query<GetCreateForum>) -> Result<impl Responder> {
    Ok(templates::NewForum {
        parent: query.parent,
    })
}

#[derive(Debug, Deserialize)]
struct GetCreateForum {
    parent: Option<i64>,
}

#[actix_web::post("/forum/new")]
pub async fn post_create_forum(
    state: web::Data<AppState>,
    web::Form(query): web::Form<PostCreateForum>,
) -> Result<impl Responder> {
    let new_forum = forum::ActiveModel {
        id: ActiveValue::Set(rand::random()),
        name: ActiveValue::Set(query.forum_name),
        parent: ActiveValue::Set(query.parent),
    }
    .insert(&state.connection)
    .await
    .map_err(error::ErrorInternalServerError)?;
    Ok(web::Redirect::to(format!("/forum/{}", new_forum.id)).see_other())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct PostCreateForum {
    parent: Option<i64>,
    forum_name: String,
}
