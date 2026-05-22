use crate::helpers::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn an_error_flash_message_is_set_on_failure() {
    // Arrange
    let test_app = spawn_app().await;

    // Act
    let login_body = serde_json::json!({
        "username": "random-user",
        "password": "random-password"
    });
    let response = test_app.post_login(&login_body).await;

    // Assert
    assert_is_redirect_to(&response, "/login");
    // check the cookies
    let flash_cookies = response.cookies().find(|c| c.name() == "_flash").unwrap();
    assert_eq!(flash_cookies.value(), "Authentication failed");

    // Act - part 2
    let html_page = test_app.get_login_html().await;
    println!("{}", html_page);
    assert!(html_page.contains(r#"<p><i>Authentication failed</i></p>"#))
}
