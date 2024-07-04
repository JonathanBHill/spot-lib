use rspotify::model::{Followers, FullPlaylist, Id, PlaylistId};

use crate::models::tracks::StoredTracks;
use crate::traits::utilities::Printables;

pub struct StoredPlaylist {
    pub complete: FullPlaylist,
    pub name: String,
    pub id: PlaylistId<'static>,
    pub owner: String,
    pub collaborative: bool,
    pub public: Option<bool>,
    pub followers: Followers,
    pub description: Option<String>,
    pub tracks: StoredTracks,
}

impl Printables for StoredPlaylist {
    fn properties(&self) {
        println!("Playlist properties:\n");
        println!("Name: {}\tID: {}", self.name, self.id.id());
        println!(
            "Owner: {}\tCollaborative: {}",
            self.owner, self.collaborative
        );
        println!(
            "Public: {:?}\tFollowers: {}",
            self.public, self.followers.total
        );
        println!("Description: {:?}", self.description);
    }
    fn info(&self) {
        println!("Playlist info:\n");
        println!("Name: {}", self.name);
        println!("ID: {}", self.id.id());
        println!("Owner: {}", self.owner);
        println!("Collaborative: {}", self.collaborative);
    }
    fn item_properties(&self) {
        self.tracks
            .tracks
            .iter()
            .enumerate()
            .for_each(|(index, track)| {
                println!("Track {}: {:?}", index + 1, track.name);
            });
    }
}
impl StoredPlaylist {
    pub fn from_playlist(playlist: FullPlaylist) -> Self {
        StoredPlaylist {
            complete: playlist.clone(),
            name: playlist.name,
            id: playlist.id,
            owner: playlist.owner.display_name.unwrap(),
            collaborative: playlist.collaborative,
            public: playlist.public,
            followers: playlist.followers,
            description: playlist.description,
            tracks: StoredTracks::from_playlist(playlist.tracks),
        }
    }
}
