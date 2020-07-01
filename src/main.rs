use tracing::{info, instrument, Level};
use tracing_subscriber;

fn main() {
    let subscriber = tracing_subscriber::fmt()
        .json()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("no global subscriber has been set");

    info!("This will be logged to stdout");
    test_func("Nicolas");
}

#[instrument]
fn test_func(name: &str) {
    info!("Got name {}", name);
    println!("Hello, {}!", name);
}
