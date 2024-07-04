use crate::utils::client;
use rspotify::model::{Id, PlaylistId};
use rspotify::{scopes, AuthCodeSpotify};

pub struct PlaylistResult {
    pub client: AuthCodeSpotify,
    pub playlist_id: String,
}

impl PlaylistResult {
    pub async fn new(playlist_id: PlaylistId<'_>) -> Self {
        let scope = scopes!(
            "playlist-read-private",
            "playlist-read-collaborative",
            "playlist-modify-public",
            "playlist-modify-private"
        );
        PlaylistResult {
            client: client::setup(Some(scope)).await,
            playlist_id: String::from(playlist_id.id()),
        }
    }
}
