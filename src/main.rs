use reqwest::{Client, Version};

pub mod profiler;

#[tokio::main]
async fn main() {
    let client = Client::new();

    profiler::profile_http(&client, Version::HTTP_3).await;
}
