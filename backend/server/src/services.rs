use actix_web::{error, web, Responder, Result};
use entity::{
    raw::{forum, post, thread},
    Forum, ForumKey, ThreadKey,
};
use sea_orm::{prelude::*, ActiveValue, QuerySelect, TransactionTrait};
use serde::Deserialize;

use crate::AppState;

#[actix_web::get("/")]
pub async fn toplevel_forums(state: web::Data<AppState>) -> Result<impl Responder> {
    let forums: Vec<_> = forum::Entity::find()
        .filter(forum::Column::Parent.is_null())
        .into_partial_model()
        .all(&state.connection)
        .await
        .map_err(error::ErrorInternalServerError)?;
    Ok(templates::Index {
        title: "All forums".into(),
        id: None,
        forums,
        threads: Vec::new(),
    })
}

#[actix_web::get("/forum/{id}")]
pub async fn get_forum(
    state: web::Data<AppState>,
    id: web::Path<ForumKey>,
) -> Result<impl Responder> {
    let forum = forum::Entity::find()
        .filter(forum::Column::Id.eq(*id))
        .one(&state.connection)
        .await
        .map_err(error::ErrorInternalServerError)?
        .ok_or_else(|| error::ErrorNotFound(format!("No forum with id {id} found")))?;
    let forums: Vec<_> = forum::Entity::find()
        .filter(forum::Column::Parent.eq(*id))
        .into_partial_model()
        .all(&state.connection)
        .await
        .map_err(error::ErrorInternalServerError)?;
    let threads: Vec<_> = thread::Entity::find()
        .filter(thread::Column::Forum.eq(*id))
        .all(&state.connection)
        .await
        .map_err(error::ErrorInternalServerError)?;
    Ok(templates::Index {
        title: forum.title.into(),
        id: Some(*id),
        forums,
        threads,
    })
}

#[actix_web::get("/forum/new")]
pub async fn get_create_forum(query: web::Query<GetCreateForum>) -> Result<impl Responder> {
    Ok(templates::NewForum {
        parent: query.parent,
    })
}

#[derive(Debug, Deserialize)]
struct GetCreateForum {
    parent: Option<ForumKey>,
}

#[actix_web::post("/forum/new")]
pub async fn post_create_forum(
    state: web::Data<AppState>,
    web::Form(query): web::Form<PostCreateForum>,
) -> Result<impl Responder> {
    let new_forum = forum::ActiveModel {
        title: ActiveValue::Set(query.forum_name),
        parent: ActiveValue::Set(query.parent.map(Into::into)),
        ..Default::default()
    }
    .insert(&state.connection)
    .await
    .map_err(error::ErrorInternalServerError)?;
    Ok(web::Redirect::to(format!("/forum/{}", ForumKey::from(new_forum.id))).see_other())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct PostCreateForum {
    parent: Option<ForumKey>,
    forum_name: String,
}

#[actix_web::get("/forum/{id}/new")]
pub async fn get_create_thread(
    state: web::Data<AppState>,
    id: web::Path<ForumKey>,
) -> Result<impl Responder> {
    let forum = forum::Entity::find()
        .filter(forum::Column::Id.eq(*id))
        .into_partial_model::<Forum>()
        .one(&state.connection)
        .await
        .map_err(error::ErrorInternalServerError)?
        .ok_or_else(|| error::ErrorNotFound(format!("No forum with id {id} found")))?;
    Ok(templates::NewPost {
        parent: forum.into(),
    })
}

#[actix_web::post("/forum/{id}/new")]
pub async fn post_create_thread(
    state: web::Data<AppState>,
    id: web::Path<ForumKey>,
    web::Form(query): web::Form<PostCreateThread>,
) -> Result<impl Responder> {
    let new_thread = state
        .connection
        .transaction::<_, _, sea_orm::DbErr>(|txn| {
            Box::pin(async move {
                let new_thread = thread::ActiveModel {
                    id: ActiveValue::Set(ThreadKey::new_random().into()),
                    forum: ActiveValue::Set((*id).into()),
                    title: ActiveValue::Set(query.title),
                }
                .insert(txn)
                .await?;
                post::ActiveModel {
                    id: ActiveValue::Set(0),
                    thread: ActiveValue::Set(new_thread.id),
                    text: ActiveValue::Set(query.text),
                    date: ActiveValue::NotSet,
                }
                .insert(txn)
                .await?;
                Ok(new_thread)
            })
        })
        .await
        .map_err(error::ErrorInternalServerError)?;
    Ok(web::Redirect::to(format!("/thread/{}", ThreadKey::from(new_thread.id))).see_other())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct PostCreateThread {
    title: String,
    text: String,
}

#[actix_web::get("/thread/{id}")]
pub async fn view_thread(
    state: web::Data<AppState>,
    id: web::Path<ThreadKey>,
) -> Result<impl Responder> {
    let thread = thread::Entity::find()
        .filter(thread::Column::Id.eq(*id))
        .one(&state.connection)
        .await
        .map_err(error::ErrorInternalServerError)?
        .ok_or_else(|| error::ErrorNotFound(format!("No forum with id {id} found")))?;
    let posts: Vec<_> = post::Entity::find()
        .filter(post::Column::Thread.eq(*id))
        .all(&state.connection)
        .await
        .map_err(error::ErrorInternalServerError)?;
    Ok(templates::ViewThread { thread, posts })
}

#[actix_web::get("/thread/{id}/reply")]
pub async fn get_reply_thread(
    state: web::Data<AppState>,
    id: web::Path<ThreadKey>,
) -> Result<impl Responder> {
    let thread = thread::Entity::find()
        .filter(thread::Column::Id.eq(*id))
        .one(&state.connection)
        .await
        .map_err(error::ErrorInternalServerError)?
        .ok_or_else(|| error::ErrorNotFound(format!("No forum with id {id} found")))?;
    Ok(templates::NewPost {
        parent: thread.into(),
    })
}

#[actix_web::post("/thread/{id}/reply")]
pub async fn post_reply_thread(
    state: web::Data<AppState>,
    id: web::Path<ThreadKey>,
    web::Form(query): web::Form<PostReplyThread>,
) -> Result<impl Responder> {
    let id = *id;
    let post_id = state
        .connection
        .transaction::<_, _, sea_orm::DbErr>(|txn| {
            Box::pin(async move {
                let last_post_id = post::Entity::find()
                    .filter(post::Column::Thread.eq(id))
                    .select_only()
                    .expr(post::Column::Id.max())
                    .into_tuple::<(i16,)>()
                    .one(txn)
                    .await?;
                let post_id = if let Some((last_post_id,)) = last_post_id {
                    last_post_id + 1
                } else {
                    return Ok(None);
                };
                post::ActiveModel {
                    id: ActiveValue::Set(post_id),
                    thread: ActiveValue::Set(id.into()),
                    text: ActiveValue::Set(query.text),
                    date: ActiveValue::NotSet,
                }
                .insert(txn)
                .await?;
                Ok(Some(post_id))
            })
        })
        .await
        .map_err(error::ErrorInternalServerError)?
        .ok_or_else(|| error::ErrorNotFound(format!("No thread with id {id} found")))?;
    Ok(web::Redirect::to(format!("/thread/{id}#post-{post_id}")).see_other())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct PostReplyThread {
    text: String,
}
