use crate::session_state::TypedSession;
use crate::utils::{e500, see_other};
use actix_web::HttpResponse;
use actix_web::http::header::ContentType;
use actix_web_flash_messages::IncomingFlashMessages;

pub async fn change_password_form(
    session: TypedSession,
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(e500)?.is_none() {
        return Ok(see_other("/login"));
    }
    let mut error_html = String::new();
    for m in flash_messages.iter() {
        error_html.push_str(&format!(r#"<p><i>{}</i></p>"#, m.content()));
    }

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
            <html lang="en">
            <head>
                <meta http-equiv="content-type" content="text/html; charset=utf-8">
                <title>Change password</title>
            </head>
            <body>
                {error_html}
                <h1>Change password</h1>
                <form action="/admin/password" method="post">
                    <label>Current password
                        <input type="password" placeholder="Enter current password" name="current_password">
                    </label>
                    <br>
                    <label>New password
                        <input type="password" placeholder="Enter new password" name="new_password">
                    </label>
                    <br>
                    <label>Confirm new password
                        <input type="password" placeholder="Type the new password again" name="new_password_check">
                    </label>
                    <button type="submit">Change password</button>
                </form>
                <p><a href="/admin/dashboard">&lt;- Back</a></p>
            </body>
            </html>
        "#,
        )))
}
