#[tokio::main(flavor = "multi_thread")]
async fn main() {
    app::app::run().await;
}
