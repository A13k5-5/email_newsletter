use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, web};
use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;
use crate::session_state::TypedSession;

/// Return an opaque 500 while preserving the error's root cause for logging
fn e500<T>(e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorInternalServerError(e)
}

pub async fn admin_dashboard(
    session: TypedSession,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let username = if let Some(user_id) = session.get_user_id().map_err(e500)? {
        get_username(user_id, &db_pool).await.map_err(e500)?
    } else {
        todo!()
        // return Ok(HttpResponse::SeeOther()
        //     .insert_header(("LOCATION", "/login"))
        //     .finish());
    };
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Admin Dashboard</title>
</head>
<body>
    <h1>Welcome {username}!</h1>
</body>
</html>
    "#
        )))
}

#[tracing::instrument(name = "Get username", skip(db_pool))]
async fn get_username(user_id: Uuid, db_pool: &PgPool) -> Result<String, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        SELECT username FROM users WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_one(db_pool)
    .await
    .context("Failed to fetch username from database")?;
    Ok(row.username)
}
