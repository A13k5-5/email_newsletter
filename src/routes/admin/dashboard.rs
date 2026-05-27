use crate::session_state::TypedSession;
use crate::utils::e500;
use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, web};
use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn admin_dashboard(
    session: TypedSession,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let username = if let Some(user_id) = session.get_user_id().map_err(e500)? {
        get_username(user_id, &db_pool).await.map_err(e500)?
    } else {
        return Ok(HttpResponse::SeeOther()
            .insert_header(("LOCATION", "/login"))
            .finish());
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
    <p>Available actions:</p>
    <ol>
        <li><a href="/admin/password">Change password</a></li>
        <li>
            <form action="/admin/logout" method="post">
                <button type="submit">Logout</button>
            </form>
        </li>
    </ol>
</body>
</html>
    "#
        )))
}

/// Get the username from the database given the user ID.
#[tracing::instrument(name = "Get username", skip(db_pool))]
pub async fn get_username(user_id: Uuid, db_pool: &PgPool) -> Result<String, anyhow::Error> {
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
