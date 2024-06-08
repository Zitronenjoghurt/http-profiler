use reqwest::{ClientBuilder, Version};
use std::time::Duration;

pub mod http3_client;
pub mod profiler;
pub mod utils;

#[tokio::main]
async fn main() {
    let client_http_10 = ClientBuilder::new()
        .http1_title_case_headers()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    let client_http_11 = ClientBuilder::new()
        .http1_title_case_headers()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    let client_http_2 = ClientBuilder::new()
        .http2_prior_knowledge()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    const ITERATIONS: u64 = 20;
    let http_10 = profiler::profile_http(&client_http_10, Version::HTTP_10, ITERATIONS).await;
    let http_11 = profiler::profile_http(&client_http_11, Version::HTTP_11, ITERATIONS).await;
    let http_2 = profiler::profile_http(&client_http_2, Version::HTTP_2, ITERATIONS).await;

    println!("HTTP/1.0: {}ms", http_10);
    println!("HTTP/1.1: {}ms", http_11);
    println!("HTTP/2:   {}ms", http_2);
}
