use crate::routes::error_chain_fmt;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError, web};
use anyhow::Context;
use sqlx::PgPool;
use std::fmt::{Debug, Formatter};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct Parameters {
    subscription_token: String,
}

#[derive(thiserror::Error)]
pub enum ConfirmError {
    #[error("There is now subscriber associated with the provided token.")]
    UnknownToken,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl Debug for ConfirmError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for ConfirmError {
    fn status_code(&self) -> StatusCode {
        match self {
            ConfirmError::UnknownToken => StatusCode::UNAUTHORIZED,
            ConfirmError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[tracing::instrument(name = "Confirm a pending subscriber.", skip(parameters))]
pub async fn confirm(
    parameters: web::Query<Parameters>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, ConfirmError> {
    let id = get_subscriber_id_from_token(&parameters.subscription_token, &db_pool)
        .await
        // if query fails
        .context("Failed to retrieve subscriber id associated with the provided token.")?
        // if id not found
        .ok_or(ConfirmError::UnknownToken)?;

    // Database error might occur
    confirm_subscriber(id, &db_pool)
        .await
        .context("Failed to update status to `confirmed`.")?;
    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(
    name = "Get subscriber_id from token",
    skip(subscription_token, db_pool)
)]
async fn get_subscriber_id_from_token(
    subscription_token: &str,
    db_pool: &PgPool,
) -> Result<Option<Uuid>, sqlx::Error> {
    let result = sqlx::query!(
        r#"
    SELECT subscriber_id FROM subscription_tokens
                         WHERE subscription_token = $1
"#,
        subscription_token
    )
    .fetch_optional(db_pool)
    .await?;

    // If found or not found, return Ok - if found, unwrap the id from the record
    match result {
        Some(record) => Ok(Some(record.subscriber_id)),
        None => Ok(None),
    }
}

#[tracing::instrument(name = "Mark subscriber as confirmed", skip(subscriber_id, db_pool))]
async fn confirm_subscriber(subscriber_id: Uuid, db_pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    UPDATE subscriptions
        SET status = 'confirmed' WHERE id = $1
"#,
        subscriber_id
    )
    .execute(db_pool)
    .await?;
    Ok(())
}
