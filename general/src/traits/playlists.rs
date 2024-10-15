use async_trait::async_trait;
use rspotify::model::{AlbumId, PlayableItem, PlaylistId};
use rspotify::prelude::{BaseClient, Id};

#[allow(dead_code)]
trait Comparisons {
    fn full_compare(&self, other: &Self) -> bool;
    fn partial_compare(&self, other: &Self) -> bool;
    fn metadata_compare(&self, other: &Self) -> bool;
    fn track_compare(&self, other: &Self) -> bool;
    fn artist_compare(&self, other: &Self) -> bool;
}
#[async_trait]
pub trait PlaylistBehavior {
    type ClientType: BaseClient + Send + Sync;
    
    fn client(&self) -> &Self::ClientType;
    fn playlist_id(&self) -> &PlaylistId<'static>;
    fn market(&self) -> rspotify::model::Market;
    
    async fn get_playlist_album_ids (&self) -> Vec<AlbumId> {
        let playlist = self
            .client()
            .playlist(self.playlist_id().clone(), None, Some(self.market()))
            .await
            .unwrap();
        let rr_track_album_ids = playlist
            .tracks
            .items
            .iter()
            .filter_map(|track| match track.track {
                Some(PlayableItem::Track(ref track)) => Some(
                    AlbumId::from_id(track.album.id.clone().unwrap().id().to_string()).unwrap(),
                ),
                _ => None,
            })
            .collect();
        return rr_track_album_ids;
    }
    fn test (&self) -> bool {
        true
    }
}