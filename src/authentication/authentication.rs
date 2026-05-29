use crate::telemetry::spawn_blocking_with_tracing;
use anyhow::Context;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use secrecy::{ExposeSecret, SecretString};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

pub struct Credentials {
    pub username: String,
    pub password: SecretString,
}

#[tracing::instrument(name = "Validate credentials", skip(credentials, db_pool))]
pub async fn validate_credentials(
    credentials: Credentials,
    db_pool: &PgPool,
) -> Result<uuid::Uuid, AuthError> {
    let mut user_id = None;
    // We need to use a constant time comparison algorithm to prevent timing attacks, even if the user doesn't exist. Hence, a default password hash.
    let mut expected_password_hash = SecretString::new(
        "$argon2id$v=19$m=15000,t=2,p=1$\
        gZiV/M1gPc22ElAH/Jh1Hw$\
        CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno"
            .into(),
    );

    if let Some((stored_user_id, stored_password_hash)) =
        get_stored_credentials(&credentials.username, &db_pool)
            .await
            .map_err(AuthError::UnexpectedError)?
    {
        user_id = Some(stored_user_id);
        expected_password_hash = stored_password_hash;
    };

    spawn_blocking_with_tracing(move || {
        verify_password_hash(expected_password_hash, credentials.password)
    })
    .await
    .context("Failed to spawn blocking task")
    .map_err(AuthError::UnexpectedError)??;

    // Even if the default password matches (highly unlikely), unknown user is not authenticated
    user_id
        .ok_or_else(|| anyhow::anyhow!("Unknown username."))
        .map_err(AuthError::InvalidCredentials)
}

#[tracing::instrument(name = "Verify password hash", skip())]
fn verify_password_hash(
    expected_password_hash: SecretString,
    password: SecretString,
) -> Result<(), AuthError> {
    let expected_password_hash = PasswordHash::new(expected_password_hash.expose_secret())
        .context("Failed to parse hash in PHC string format")
        .map_err(AuthError::UnexpectedError)?;

    Argon2::default()
        .verify_password(password.expose_secret().as_bytes(), &expected_password_hash)
        .context("Invalid password")
        .map_err(AuthError::InvalidCredentials)
}

#[tracing::instrument(name = "Get stored credentials.", skip(username, db_pool))]
async fn get_stored_credentials(
    username: &str,
    db_pool: &PgPool,
) -> Result<Option<(uuid::Uuid, SecretString)>, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        SELECT user_id, password_hash FROM users
                   WHERE username = $1
        "#,
        username,
    )
    .fetch_optional(db_pool)
    .await
    .context("Failed to perform a query to retrieve stored credentials.")?
    .map(|row| (row.user_id, SecretString::new(row.password_hash.into())));

    Ok(row)
}

#[tracing::instrument(name = "Change password", skip(db_pool, new_password))]
pub async fn change_password(
    db_pool: &PgPool,
    user_id: Uuid,
    new_password: SecretString,
) -> Result<(), anyhow::Error> {
    let password_hash = spawn_blocking_with_tracing(move || compute_password_hash(new_password))
        .await
        .context("Failed to spawn blocking task to compute password hash.")??;
    sqlx::query!(
        r#"
        UPDATE users
        SET password_hash = $1
        WHERE user_id = $2
        "#,
        password_hash.expose_secret(),
        user_id,
    )
    .execute(db_pool)
    .await?;
    Ok(())
}

fn compute_password_hash(password: SecretString) -> Result<SecretString, anyhow::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(password.expose_secret().as_bytes(), salt.as_salt())?
        .to_string();
    Ok(SecretString::new(password_hash.into()))
}
