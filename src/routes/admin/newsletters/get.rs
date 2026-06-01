use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

#[tracing::instrument(name = "Display the newsletter publication form", skip(flash_messages))]
pub async fn publish_newsletter_form(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut error_html = String::new();
    for m in flash_messages.iter() {
        writeln!(error_html, "<p><i>{}</i></p>", m.content()).unwrap()
    }
    let idempotency_key = uuid::Uuid::new_v4();
    Ok(HttpResponse::Ok().body(format!(
        r#"
        <html lang="en">
        <head>
            <meta http-equiv="content-type" content="text/html; charset=utf-8">
            <title>Publish newsletter issue</title>
        </head>
        <body>
            {error_html}
            <h1>Publish newsletter issue</h1>
            <form action="/newsletters" method="post">
                <label>Title
                    <input type="text" name="title">
                    <placeholder="Title of the newsletter issue"></input>
                </label>
                <br>
                <label>HTML content
                    <textarea name="html_content"></textarea>
                    <placeholder="HTML content of the newsletter issue"></textarea>
                </label>
                <br>
                <label>Text content
                    <textarea name="text_content"></textarea>
                    <placeholder="Text content of the newsletter issue"></textarea>
                </label>
                <br>
                <input type="hidden" name="idempotency_key" value="{idempotency_key}">
                <button type="submit">Publish</button>
            </form>
            <p><a href="/admin/dashboard">&lt;- Back</a></p>
        </body>
        </html>
        "#,
    )))
}
