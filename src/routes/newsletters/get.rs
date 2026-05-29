use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

#[tracing::instrument(name = "Display the newsletter publication form", skip(flash_messages))]
pub async fn publish_newsletter_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let mut error_html = String::new();
    for m in flash_messages.iter() {
        writeln!(error_html, "<p><i>{}</i></p>", m.content()).unwrap()
    }
    HttpResponse::Ok().body(
        r#"
        <form action="/newsletters" method="post">
            <label>Title
                <input type="text" name="title">
            </label>
            <br>
            <label>HTML content
                <textarea name="html_content"></textarea>
            </label>
            <br>
            <label>Text content
                <textarea name="text_content"></textarea>
            </label>
            <br>
            <button type="submit">Publish</button>
        </form>
        "#,
    )
}
