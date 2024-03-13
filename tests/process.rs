use anyhow::Result;

use helpers::common::TestClient;
use serde_json::json;

mod helpers;

#[tokio::test]
async fn health_check() -> Result<()> {
    let client = TestClient::new().await;
    let (status, _) = client
        .post("/api/process", None, Some(json!("toto")))
        .await?;

    assert_eq!(status, reqwest::StatusCode::OK);
    Ok(())
}
