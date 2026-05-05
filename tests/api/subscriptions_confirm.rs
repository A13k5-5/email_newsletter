use crate::helpers::spawn_app;

#[tokio::test]
async fn confirmation_without_token_are_rejected_with_400() {
    // Arrange
    let test_app = spawn_app().await;

    // Act
    let response = test_app.get_confirm().await;

    // Assert
    assert_eq!(response.status().as_u16(), 400)
}
