use reqwest::{Client, Version};

#[tokio::main]
async fn main() {
    let client = Client::new();
    let res = client
        .get("https://profiling.lemon.industries/")
        .version(Version::HTTP_11)
        .send()
        .await
        .unwrap();
    println!("HTTP/1.0 response: {:?}", res);
}
