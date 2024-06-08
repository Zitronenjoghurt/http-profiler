use reqwest::{Client, Version};
use tokio::time::{sleep, Duration};

use crate::{http3_client::Http3Client, utils::timestamp_now_millis};

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
    let before = timestamp_now_millis();
    let response = client
        .get("https://profiling.lemon.industries/")
        .version(version)
        .send()
        .await
        .unwrap();
    let _body = response.text().await.unwrap();
    let after = timestamp_now_millis();
    let latency = after - before;
    println!("{} GET / took {}ms", version_to_string(version), latency);
    latency
}

pub async fn profile_http3(iterations: u64) -> f32 {
    let mut total_duration: u64 = 0;
    let mut client = Http3Client::default();

    for _ in 0..iterations {
        let before = timestamp_now_millis();
        client.request().unwrap();
        sleep(Duration::from_millis(100)).await;
        let after = timestamp_now_millis();
        total_duration += after - before;
    }

    (total_duration as f32) / (iterations as f32)
}

pub fn version_to_string(version: Version) -> &'static str {
    match version {
        Version::HTTP_09 => "HTTP/0.9",
        Version::HTTP_10 => "HTTP/1.0",
        Version::HTTP_11 => "HTTP/1.1",
        Version::HTTP_2 => "HTTP/2",
        Version::HTTP_3 => "HTTP/3",
        _ => "",
    }
}
