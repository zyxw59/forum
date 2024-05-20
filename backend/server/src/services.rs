use actix_web::{error, web, Responder, Result};
use entity::{
    raw::{forum, post, thread},
    Forum, ForumKey, ThreadKey,
};
use sea_orm::{prelude::*, ActiveValue, QuerySelect, TransactionTrait};
use serde::Deserialize;

use crate::AppState;

pub async fn index(
    state: web::Data<AppState>,
    forum_id: Option<web::Path<ForumKey>>,
) -> Result<impl Responder> {
    let forum_id = forum_id.map(|id| *id);
    let title;
    let forums_filter;
    let threads;
    if let Some(forum_id) = forum_id {
        forums_filter = forum::Column::Parent.eq(forum_id);
        title = forum::Entity::find()
            .filter(forum::Column::Id.eq(forum_id))
            .one(&state.connection)
            .await
            .map_err(error::ErrorInternalServerError)?
            .ok_or_else(|| error::ErrorNotFound(format!("No forum with id {forum_id} found")))?.title.into();
        threads = thread::Entity::find()
            .filter(thread::Column::Forum.eq(forum_id))
            .all(&state.connection)
            .await
            .map_err(error::ErrorInternalServerError)?;
    } else {
        forums_filter = forum::Column::Parent.is_null();
        title = "All forums".into();
        threads = Vec::new();
    }
    let forums: Vec<_> = forum::Entity::find()
        .filter(forums_filter)
        .into_partial_model()
        .all(&state.connection)
        .await
        .map_err(error::ErrorInternalServerError)?;
    Ok(templates::Index {
        title,
        id: forum_id,
        forums,
        threads,
    })
}

pub async fn new_forum_get(parent: Option<web::Path<ForumKey>>) -> Result<impl Responder> {
    Ok(templates::NewForum { parent: parent.map(web::Path::into_inner) })
}

pub async fn new_forum_post(
    state: web::Data<AppState>,
    parent: Option<web::Path<ForumKey>>,
    web::Form(query): web::Form<NewForum>,
) -> Result<impl Responder> {
    let new_forum = forum::ActiveModel {
        title: ActiveValue::Set(query.forum_name),
        parent: ActiveValue::Set(parent.map(|parent| (*parent).into())),
        ..Default::default()
    }
    .insert(&state.connection)
    .await
    .map_err(error::ErrorInternalServerError)?;
    Ok(web::Redirect::to(format!("/forum/{}/", ForumKey::from(new_forum.id))).see_other())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct NewForum {
    forum_name: String,
}

pub async fn new_thread_get(
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

pub async fn new_thread_post(
    state: web::Data<AppState>,
    id: web::Path<ForumKey>,
    web::Form(query): web::Form<NewThread>,
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
    Ok(web::Redirect::to(format!("/thread/{}/", ThreadKey::from(new_thread.id))).see_other())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct NewThread {
    title: String,
    text: String,
}

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

pub async fn reply_get(
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

pub async fn reply_post(
    state: web::Data<AppState>,
    id: web::Path<ThreadKey>,
    web::Form(query): web::Form<Reply>,
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
    Ok(web::Redirect::to(format!("/thread/{id}/#post-{post_id}")).see_other())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Reply {
    text: String,
}
