use crate::authentication::middleware::UserId;
use crate::authentication::{AuthError, Credentials, validate_credentials};
use crate::routes::admin::dashboard::get_username;
use crate::utils::{e500, see_other};
use actix_web::{HttpResponse, web};
use actix_web_flash_messages::FlashMessage;
use secrecy::{ExposeSecret, SecretString};

#[derive(serde::Deserialize)]
pub struct FormData {
    current_password: SecretString,
    new_password: SecretString,
    new_password_check: SecretString,
}

pub async fn change_password(
    form: web::Form<FormData>,
    db_pool: web::Data<sqlx::PgPool>,
    user_id: web::ReqData<UserId>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();
    if form.new_password.expose_secret() != form.new_password_check.expose_secret() {
        FlashMessage::error(
            "You entered two different new passwords - the field values must match.",
        )
        .send();
        return Ok(see_other("/admin/password"));
    }
    let username = get_username(*user_id, &db_pool).await.map_err(e500)?;
    let credentials = Credentials {
        username,
        password: form.0.current_password.clone(),
    };
    // Check if current password is correct
    if let Err(e) = validate_credentials(credentials, &db_pool).await {
        return match e {
            AuthError::InvalidCredentials(_) => {
                FlashMessage::error("The current password is incorrect.").send();
                return Ok(see_other("/admin/password"));
            }
            AuthError::UnexpectedError(_) => Err(e500(e)),
        };
    }
    // Check if new password is in the allowed length range (> 12 and < 129 characters)
    // let password_length = form.new_password.expose_secret().len();
    // if password_length <= 12 || password_length >= 129 {
    //     FlashMessage::error(
    //         "The new password must be longer than 12 and shorter than 129 characters.",
    //     )
    //     .send();
    //     return Ok(see_other("/admin/password"));
    // }

    crate::authentication::change_password(&db_pool, *user_id, form.new_password.clone())
        .await
        .map_err(e500)?;
    FlashMessage::info("Your password has been changed.").send();
    Ok(see_other("/admin/password"))
}

/// TODO: For refactoring
async fn _check_current_password() -> Result<(), actix_web::Error> {
    todo!()
}
