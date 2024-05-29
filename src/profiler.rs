use reqwest::{Client, Version};

pub async fn profile_http(client: &Client, version: Version) -> u64 {
    let res = client
        .get("https://profiling.lemon.industries/")
        .version(version)
        .send()
        .await
        .unwrap();
    0
}
