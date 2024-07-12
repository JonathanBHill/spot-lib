use spot_lib_general::playlists::update::ReleaseRadar;

#[tokio::main]
async fn main() {
    let rr = ReleaseRadar::new().await;
    rr.update_rr(true).await;
}
