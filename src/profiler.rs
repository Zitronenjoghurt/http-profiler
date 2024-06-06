use reqwest::{Client, Version};
use tokio::time::{sleep, Duration};

use crate::utils::timestamp_now_nanos;

pub async fn profile_http(client: &Client, version: Version, iterations: u64) -> f32 {
    let mut total_duration: u64 = 0;

    for _ in 0..iterations {
        let duration = ping_api(client, version).await;
        total_duration += duration;
        sleep(Duration::from_millis(100)).await;
    }

    (total_duration as f32) / (iterations as f32)
}

pub async fn ping_api(client: &Client, version: Version) -> u64 {
    let before = timestamp_now_nanos();
    let response = client
        .get("https://profiling.lemon.industries/")
        .version(version)
        .send()
        .await
        .unwrap();
    let _body = response.text().await.unwrap();
    let after = timestamp_now_nanos();
    after - before
}
