use reqwest::{Client, Version};
use tokio::time::{sleep, Duration};

use crate::{http3_client::Http3Client, utils::timestamp_now_nanos};

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

pub async fn profile_http3(iterations: u64) -> f32 {
    let mut total_duration: u64 = 0;
    let mut client = Http3Client::default();

    for _ in 0..iterations {
        let before = timestamp_now_nanos();
        client.request().unwrap();
        sleep(Duration::from_millis(100)).await;
        let after = timestamp_now_nanos();
        total_duration += after - before;
    }

    (total_duration as f32) / (iterations as f32)
}
