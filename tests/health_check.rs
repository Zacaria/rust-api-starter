use anyhow::Result;

use helpers::common::TestClient;

mod helpers;

#[tokio::test]
async fn health_check() -> Result<()> {
    let client = TestClient::new().await;
    let (status, _) = client.get("/health", None).await?;

    assert_eq!(status, reqwest::StatusCode::OK);
    Ok(())
}
