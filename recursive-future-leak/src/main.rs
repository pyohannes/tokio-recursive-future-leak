use hyper::{Body, Client, Method, Request};
use std::future::Future;
use std::pin::Pin;

fn nested_request<'a>(w: bool) -> Pin<Box<dyn Future<Output = ()> + 'a>> {
    Box::pin(async move {
        let client = Client::builder().build_http();
        let request = Request::builder()
            .method(Method::GET)
            .uri("http://example.com")
            .body(Body::from(""))
            .unwrap();
        client.request(request).await.unwrap();
        if w {
            nested_request(false).await;
        }
    })
}

#[tokio::main]
async fn main() {
    nested_request(true).await;
}
