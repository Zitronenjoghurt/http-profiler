use reqwest::{Client, Version};

pub mod http3_client;
pub mod profiler;
pub mod utils;

#[tokio::main]
async fn main() {
    let client = Client::new();

    let http_10 = profiler::profile_http(&client, Version::HTTP_10, 1).await;
    let http_11 = profiler::profile_http(&client, Version::HTTP_11, 1).await;
    let http_2 = profiler::profile_http(&client, Version::HTTP_2, 1).await;
    let http_3 = profiler::profile_http3(1).await;

    println!("HTTP/1.0: {}", http_10);
    println!("HTTP/1.1: {}", http_11);
    println!("HTTP/2:   {}", http_2);
    println!("HTTP/3:   {}", http_3);
}
