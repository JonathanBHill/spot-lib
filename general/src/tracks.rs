use rspotify::{AuthCodeSpotify};
use rspotify::clients::BaseClient;
use rspotify::model::{Id, TrackId};
use crate::utils::client::setup;

pub struct TrackUtil {
    client: AuthCodeSpotify,
    // track_ids: Vec<TrackId<'a>>,
}
impl TrackUtil {
    pub async fn new() -> Self {
        // let static_track_ids: Vec<TrackId<'static>> = track_ids.iter().map(|id| TrackId::from(id.clone())).collect::<Vec<TrackId<'static>>>();
        TrackUtil { client: setup(None).await }
    }
    
    pub async fn get_track_names(&self, track_ids: Vec<TrackId<'_>>) {
        // let mut names = Vec::new();
        let mut counter = 1;
        for chunk in track_ids.chunks(20) {
            let tracks = self.client.tracks(chunk.to_vec(), None).await.unwrap();
            for track in tracks {
                println!("{:?}. {:?} - {:?}", counter, track.artists.iter().map(|artist| artist.name.clone()).collect::<Vec<String>>(), track.name);
                // names.push(track.name);
                counter += 1;
            }
        }
        // names
    }
}
// pub fn get_track_names(ids: Vec<String>) -> Vec<String> {
//     let mut names = Vec::new();
//     for id in ids {
//         let track = get_track_by_id(&id);
//         names.push(track.name.clone());
//     }
//     names
// }
