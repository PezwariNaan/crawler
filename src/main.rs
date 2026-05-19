#[tokio::main]
async fn main() {
    let url = "http://localhost:8080";
    let body = reqwest::get(url).await.unwrap().text().await.unwrap();
    println!("{}", body);
}

