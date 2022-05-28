
use snaplink::service;
use tokio;
use tracing_subscriber;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
    .with(tracing_subscriber::EnvFilter::new(
        std::env::var("RUST_LOG").unwrap_or_else(|_| "example_sessions=debug".into()),
    ))
    .with(tracing_subscriber::fmt::layer())
    .init();

    println!("We are here!");
}
