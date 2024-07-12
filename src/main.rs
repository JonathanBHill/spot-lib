use spot_lib_cli::interface::TerminalApp;
#[tokio::main]
async fn main() {
    let app = TerminalApp::new();
    app.run().await;
}
