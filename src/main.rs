use reqwest::{Client, Version};

pub mod http3_client;
pub mod profiler;
pub mod utils;

#[tokio::main]
async fn main() {
    let client = Client::new();

    const ITERATIONS: u64 = 20;
    let http_10 = profiler::profile_http(&client, Version::HTTP_10, ITERATIONS).await;
    let http_11 = profiler::profile_http(&client, Version::HTTP_11, ITERATIONS).await;
    let http_2 = profiler::profile_http(&client, Version::HTTP_2, ITERATIONS).await;

    println!("HTTP/1.0: {}ms", http_10);
    println!("HTTP/1.1: {}ms", http_11);
    println!("HTTP/2:   {}ms", http_2);
}
