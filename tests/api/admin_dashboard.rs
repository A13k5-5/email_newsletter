use crate::helpers::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn admin_dashboard_requires_authentication() {
    // Arrange
    let test_app = spawn_app().await;

    // Act
    let response = test_app.get_admin_dashboard().await;

    // Assert
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn logout_clears_session_state() {
    // Arrange
    let test_app = spawn_app().await;

    // Act - part 1 - login
    let response = test_app.login_as_test_user().await;
    assert_is_redirect_to(&response, "/admin/dashboard");

    // Act - part 2 - follow the redirect
    let html_page = test_app.get_admin_dashboard_html().await;
    assert!(html_page.contains(&format!("Welcome {}!", &test_app.test_user.username)));

    // Act - part 3 - logout
    let response = test_app.post_logout().await;
    assert_is_redirect_to(&response, "/login");

    // Act - part 4 - follow the redirect
    let html_page = test_app.get_login_html().await;
    assert!(html_page.contains(r#"<p><i>You have successfully logged out.</i></p>"#));

    // Act - part 5 - try to access the admin dashboard again
    let response = test_app.get_admin_dashboard().await;
    assert_is_redirect_to(&response, "/login");
}
