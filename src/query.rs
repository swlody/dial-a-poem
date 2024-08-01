use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse as _, Response},
    routing::get,
    Json, Router,
};
use maud::{html, Markup, DOCTYPE};
use sqlx::SqlitePool;

use crate::{errors::Result, poem::Poem};

fn render_body(content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang = "en" {
            head {
                meta charset="utf-8";
                link rel = "stylesheet" href="/static/style.css";
                script {
                    "0"
                }
            }
            body {
                (content)
            }
        }
    }
}

async fn api_random(State(db): State<SqlitePool>) -> Result<Response> {
    let poem = Poem::get_random(db).await?;
    Ok((StatusCode::OK, Json(poem)).into_response())
}

async fn html_random(State(db): State<SqlitePool>) -> Result<Markup> {
    let poem = Poem::get_random(db).await?;
    let body = render_body(poem.into_html());
    Ok(body)
}

async fn api_random_by_author(
    Path(author): Path<String>,
    State(db): State<SqlitePool>,
) -> Result<Response> {
    let poem = Poem::get_random_by_author(&author, db).await?;
    Ok((StatusCode::OK, Json(poem)).into_response())
}

async fn html_random_by_author(
    Path(author): Path<String>,
    State(db): State<SqlitePool>,
) -> Result<Markup> {
    let poem = Poem::get_random_by_author(&author, db).await?;
    let body = render_body(poem.into_html());
    Ok(body)
}

async fn api_specific_poem(
    Path((author, title)): Path<(String, String)>,
    State(db): State<SqlitePool>,
) -> Result<Response> {
    let poem = Poem::get_specific_poem(&author, &title, db).await?;
    Ok((StatusCode::OK, Json(poem)).into_response())
}

async fn html_specific_poem(
    Path((author, title)): Path<(String, String)>,
    State(db): State<SqlitePool>,
) -> Result<Markup> {
    let poem = Poem::get_specific_poem(&author, &title, db).await?;
    let body = render_body(poem.into_html());
    Ok(body)
}

pub fn routes() -> Router<SqlitePool> {
    Router::new()
        .route("/author/:author/title/:title", get(html_specific_poem))
        .route("/random", get(html_random))
        .route("/random/:author", get(html_random_by_author))
        .route("/api/author/:author/title/:title", get(api_specific_poem))
        .route("/api/random", get(api_random))
        .route("/api/random/:author", get(api_random_by_author))
}

#[cfg(test)]
mod tests {
    use axum::{body::Body, http::Request};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use super::*;

    #[tokio::test]
    async fn can_get_random_poem() -> anyhow::Result<()> {
        let db = SqlitePool::connect("sqlite://poems.sqlite3").await?;
        let app = routes().with_state(db);
        let response = app
            .oneshot(Request::builder().uri("/api/random").body(Body::empty())?)
            .await?;
        assert_eq!(StatusCode::OK, response.status());
        assert!(!response.into_body().collect().await?.to_bytes().is_empty());
        Ok(())
    }
}
