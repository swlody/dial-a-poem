use axum::{
    extract::{Path, State},
    response::{IntoResponse as _, Redirect, Response},
    routing::get,
    Router,
};
use maud::Markup;
use sqlx::SqlitePool;

use crate::{errors::Result, poem::Poem, render::wrap_body};

async fn html_random(State(db): State<SqlitePool>) -> Result<Response> {
    let Poem { author, title, .. } = Poem::get_random(db).await?;
    Ok(Redirect::to(&format!("/poem/{author}/{title}")).into_response())
}

async fn html_random_by_author(
    Path(author): Path<String>,
    State(db): State<SqlitePool>,
) -> Result<Response> {
    let Poem { author, title, .. } = Poem::get_random_by_author(&author, db).await?;
    Ok(Redirect::to(&format!("/poem/{author}/{title}")).into_response())
}

async fn html_specific_poem(
    Path((author, title)): Path<(String, String)>,
    State(db): State<SqlitePool>,
) -> Result<Markup> {
    let poem = Poem::get_specific_poem(&author, &title, db).await?;
    let body = wrap_body(&poem.into_html());
    Ok(body)
}

pub fn routes() -> Router<SqlitePool> {
    Router::new()
        .route("/poem/:author/:title", get(html_specific_poem))
        .route("/poem/random", get(html_random))
        .route("/poem/:author/random", get(html_random_by_author))
}