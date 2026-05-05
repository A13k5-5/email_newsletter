use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct Parameters {
    subscription_token: String,
}

#[tracing::instrument(name = "Confirm a pending subscriber.", skip(parameters))]
pub async fn confirm(
    parameters: web::Query<Parameters>,
    db_pool: web::Data<PgPool>,
) -> HttpResponse {
    // Run the query - throws error if query fails
    let id = match get_subscriber_id_from_token(&parameters.subscription_token, &db_pool).await {
        Ok(id) => id,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // if such id is not found in the database - throw an error
    if id.is_none() {
        return HttpResponse::InternalServerError().finish();
    }
    let id = id.unwrap(); // can be done safely after the check

    // Database error might occur
    match confirm_subscriber(id, &db_pool).await {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
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
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query {:?}", e);
        e
    })?;

    // If found or not found, return Ok - if found, unwrap the id from the record
    match result {
        Some(record) => Ok(Some(record.subscriber_id)),
        None => Ok(None)
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
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query {:?}", e);
        e
    })?;
    Ok(())
}
