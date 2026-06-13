mod common;

use crate::common::setup;

#[tokio::test]
async fn server_starts() {
    let server = setup().await;
    let response = server.get("/").await;
    response.assert_status_ok();
}
