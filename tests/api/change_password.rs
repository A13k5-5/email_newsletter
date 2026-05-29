use crate::helpers::{assert_is_redirect_to, spawn_app};
use uuid::Uuid;

#[tokio::test]
async fn you_must_be_logged_in_to_see_the_change_password_form() {
    // Arrange
    let test_app = spawn_app().await;

    // Act
    let response = test_app.get_change_password().await;

    // Assert
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn you_must_be_logged_in_to_change_your_password() {
    // Arrange
    let test_app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();

    // Act
    let response = test_app
        .post_change_password(&serde_json::json!({
            "current_password": Uuid::new_v4().to_string(),
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;

    // Assert
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn new_password_fields_must_match() {
    // Arrange
    let test_app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();

    // Act - part 1 - login
    test_app
        .post_login(&serde_json::json!({
            "username": &test_app.test_user.username,
            "password": &test_app.test_user.password,
        }))
        .await;

    // Act - part 2 - change password
    let response = test_app
        .post_change_password(&serde_json::json!({
            "current_password": &test_app.test_user.password,
            "new_password": &new_password,
            // the confirmation field doesn't match the new password
            "new_password_check": Uuid::new_v4().to_string(),
        }))
        .await;
    assert_is_redirect_to(&response, "/admin/password");

    // Act - part 3 - follow the redirect
    let html_page = test_app.get_change_password_html().await;
    assert!(html_page.contains(
        "<p><i>You entered two different new passwords - the field values must match.</i></p>",
    ));
}

#[tokio::test]
async fn current_password_must_be_valid() {
    // Arrange
    let test_app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();

    // Act - part 1 - login
    test_app
        .post_login(&serde_json::json!({
            "username": &test_app.test_user.username,
            "password": &test_app.test_user.password,
        }))
        .await;

    // Act - part 2 - change password
    let response = test_app
        .post_change_password(&serde_json::json!({
            "current_password": Uuid::new_v4().to_string(), // invalid current password
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;
    assert_is_redirect_to(&response, "/admin/password");

    // Act - part 3 - follow the redirect
    let html_page = test_app.get_change_password_html().await;
    assert!(html_page.contains("<p><i>The current password is incorrect.</i></p>",));
}

#[tokio::test]
async fn new_password_must_be_in_the_length_range() {
    // Arrange
    let test_app = spawn_app().await;
    let passwords = ["a".repeat(12), "a".repeat(129)];

    // Act - part 1 - login
    test_app
        .post_login(&serde_json::json!({
            "username": &test_app.test_user.username,
            "password": &test_app.test_user.password,
        }))
        .await;

    for new_password in passwords {
        // Act - part 2 - change password
        let response = test_app
            .post_change_password(&serde_json::json!({
                "current_password": &test_app.test_user.password,
                "new_password": &new_password,
                "new_password_check": &new_password,
            }))
            .await;
        assert_is_redirect_to(&response, "/admin/password");

        // Act - part 3 - follow the redirect
        let html_page = test_app.get_change_password_html().await;
        assert!(html_page.contains(
            "<p><i>The new password must be longer than 12 and shorter than 129 characters.</i></p>",
        ));
    }
}

#[tokio::test]
async fn changing_password_works() {
    // Arrange
    let test_app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();

    // Act - part 1 - login
    test_app
        .post_login(&serde_json::json!({
            "username": &test_app.test_user.username,
            "password": &test_app.test_user.password,
        }))
        .await;

    // Act - part 2 - change password
    let response = test_app
        .post_change_password(&serde_json::json!({
            "current_password": &test_app.test_user.password,
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;
    assert_is_redirect_to(&response, "/admin/password");

    // Act - part 3 - follow the redirect
    let html_page = test_app.get_change_password_html().await;
    assert!(html_page.contains(
        "<p><i>Your password has been changed.</i></p>",
    ));

    // Act - part 4 - log out
    let response = test_app.post_logout().await;
    assert_is_redirect_to(&response, "/login");

    // Act - part 5 - follow the redirect
    let html_page = test_app.get_login_html().await;
    assert!(html_page.contains(r#"<p><i>You have successfully logged out.</i></p>"#));

    // Act - part 6 - log in with the new password
    let response = test_app
        .post_login(&serde_json::json!({
            "username": &test_app.test_user.username,
            "password": &new_password,
        }))
        .await;
    assert_is_redirect_to(&response, "/admin/dashboard");

}
