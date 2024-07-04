use rspotify::model::{AlbumId, FullTrack, Id, Page, PlayableItem, PlaylistItem};

// use general::enums::tracks::TrackType;
use crate::enums::tracks::TrackType;

pub struct StoredTrack {
    pub name: String,
    pub id: String,
    pub artist_id: Vec<String>,
    pub artist_name: Vec<String>,
    pub album_id: Option<AlbumId<'static>>,
    pub album_name: String,
    pub album_artists: Vec<String>,
    pub duration: chrono::Duration,
    pub explicit: bool,
    pub popularity: u32,
    pub track_number: u32,
    pub preview_url: Option<String>,
    pub is_local: bool,
}
impl StoredTrack {
    pub fn new(track: PlayableItem) -> Self {
        // let test = track::FullTrack;
        if let PlayableItem::Track(track) = track {
            StoredTrack {
                name: track.name,
                id: track.id.unwrap().to_string(),
                artist_id: track
                    .artists
                    .iter()
                    .map(|artist| artist.id.clone().unwrap().id().to_string())
                    .collect::<Vec<String>>(),
                artist_name: track
                    .artists
                    .iter()
                    .map(|artist| artist.name.clone())
                    .collect::<Vec<String>>(),
                album_id: track.album.id.clone(),
                album_name: track.album.name,
                album_artists: track
                    .album
                    .artists
                    .iter()
                    .map(|artist| artist.name.clone())
                    .collect::<Vec<String>>(),
                duration: track.duration,
                explicit: track.explicit,
                popularity: track.popularity,
                track_number: track.track_number,
                preview_url: track.preview_url,
                is_local: track.is_local,
            }
        } else {
            panic!("Error: {:?}", track);
        }
    }
    pub fn from_track(track: FullTrack) -> Self {
        StoredTrack {
            name: track.name,
            id: track.id.unwrap().id().to_string(),
            artist_id: track
                .artists
                .iter()
                .map(|artist| artist.id.clone().unwrap().id().to_string())
                .collect::<Vec<String>>(),
            artist_name: track
                .artists
                .iter()
                .map(|artist| artist.name.clone())
                .collect::<Vec<String>>(),
            album_id: track.album.id.clone(),
            album_name: track.album.name,
            album_artists: track
                .album
                .artists
                .iter()
                .map(|artist| artist.name.clone())
                .collect::<Vec<String>>(),
            duration: track.duration,
            explicit: track.explicit,
            popularity: track.popularity,
            track_number: track.track_number,
            preview_url: track.preview_url,
            is_local: track.is_local,
        }
    }
}

pub struct StoredTracks {
    pub tracks: Vec<StoredTrack>,
}
impl StoredTracks {
    pub fn from_vector(playlist_tracks: Vec<StoredTrack>) -> Self {
        StoredTracks {
            tracks: playlist_tracks,
        }
    }
    pub fn from_playlist(tracks: Page<PlaylistItem>) -> Self {
        let mut track_vec = Vec::new();
        tracks.items.iter().for_each(|item| {
            if let Some(PlayableItem::Track(ref item)) = item.track {
                track_vec.push(StoredTrack::from_track(item.clone()));
            }
        });
        StoredTracks { tracks: track_vec }
    }
    pub fn new() -> Self {
        StoredTracks { tracks: Vec::new() }
    }
    pub fn add(&mut self, tracks: TrackType) {
        match tracks {
            TrackType::Track(track) => self.tracks.push(track),
            TrackType::Tracks(mut tracks) => self.tracks.append(&mut tracks),
        }
    }
    // fn add_single(&mut self, track: StoredTrack) {
    //     self.tracks.push(track);
    // }
    // fn add_multiple(&mut self, mut tracks: Vec<StoredTrack>) {
    //     self.tracks.append(&mut tracks);
    // }
    #[allow(dead_code)]
    fn get_tracks(&self) -> &Vec<StoredTrack> {
        &self.tracks
    }
    pub fn get_track(&self, index: usize) -> &StoredTrack {
        &self.tracks[index]
    }
    pub fn get_track_mut(&mut self, index: usize) -> &mut StoredTrack {
        &mut self.tracks[index]
    }
    pub fn get_track_by_id(&self, id: &str) -> Option<&StoredTrack> {
        self.tracks.iter().find(|track| track.id == id)
    }
    pub fn get_track_mut_by_id(&mut self, id: &str) -> Option<&mut StoredTrack> {
        self.tracks.iter_mut().find(|track| track.id == id)
    }
    pub fn get_track_by_name(&self, name: &str) -> Option<&StoredTrack> {
        self.tracks.iter().find(|track| track.name == name)
    }
    pub fn get_track_mut_by_name(&mut self, name: &str) -> Option<&mut StoredTrack> {
        self.tracks.iter_mut().find(|track| track.name == name)
    }
    pub fn get_track_by_artist(&self, artist: &str) -> Option<&StoredTrack> {
        self.tracks
            .iter()
            .find(|track| track.artist_name.contains(&artist.to_string()))
    }
    pub fn get_track_mut_by_artist(&mut self, artist: &str) -> Option<&mut StoredTrack> {
        self.tracks
            .iter_mut()
            .find(|track| track.artist_name.contains(&artist.to_string()))
    }
    pub fn get_track_by_album_artist(&self, artist: &str) -> Option<&StoredTrack> {
        self.tracks
            .iter()
            .find(|track| track.album_artists.contains(&artist.to_string()))
    }
    pub fn get_track_mut_by_album_artist(&mut self, artist: &str) -> Option<&mut StoredTrack> {
        self.tracks
            .iter_mut()
            .find(|track| track.album_artists.contains(&artist.to_string()))
    }
}
