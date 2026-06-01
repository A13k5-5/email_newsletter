use actix_web::http::StatusCode;
use crate::idempotency::IdempotencyKey;
use actix_web::HttpResponse;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_saved_response(
    db_pool: &PgPool,
    idempotency_key: IdempotencyKey,
    user_id: Uuid,
) -> Result<Option<HttpResponse>, anyhow::Error> {
    let saved_response = sqlx::query!(
        r#"
        SELECT
            response_status_code,
            response_headers as "response_headers: Vec<HeaderPairRecord>",
            response_body
        FROM idempotency
        WHERE user_id = $1 AND idempotency_key = $2
        "#,
        user_id,
        idempotency_key.as_ref()
    )
    .fetch_optional(db_pool)
    .await?;
    if let Some(saved_response) = saved_response {
        let status_code = StatusCode::from_u16(saved_response.response_status_code.try_into()?)?;
        let mut response = HttpResponse::build(status_code);
        for HeaderPairRecord { name, value } in saved_response.response_headers {
            response.append_header((name, value));
        }
        Ok(Some(response.body(saved_response.response_body)))
    } else {
        Ok(None)
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "header_pair")]
struct HeaderPairRecord {
    name: String,
    value: Vec<u8>,
}
