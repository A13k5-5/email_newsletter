use crate::helpers::assert_is_redirect_to;

#[tokio::test]
async fn admin_dashboard_requires_authentication() {
    // Arrange
    let test_app = crate::helpers::spawn_app().await;

    // Act
    let response = test_app.get_admin_dashboard().await;

    // Assert
    assert_is_redirect_to(&response, "/login");
}
