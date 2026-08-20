
use launcher::Launcher;

#[tokio::main]
async fn main() {
    let app = Launcher::new();
    app.run().await;
}