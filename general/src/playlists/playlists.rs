use dotenv::dotenv;
use prettytable::{row, Table};
use rspotify::{AuthCodeSpotify, scopes};
use rspotify::model::FullPlaylist;

// use crate::enums::playlists::PlaylistParam::String;
use crate::models::tracks::StoredTracks;
use crate::utils::client::setup;

pub enum Playlist {
    ReleaseRadar(FullPlaylist),
    // DiscoverWeekly,
    // DailyMix,
    // TopArtists,
    // TopTracks,
    // RecentlyPlayed,
    // LikedSongs,
    // Custom,
}

// #[allow(dead_code)]
// impl PlaylistActions for Playlist {
//     // fn replace_tracks(&self, track_ids: Vec<TrackId>) {
//     //     todo!()
//     // }
//
//     // fn get_playlist_tracks(&self) {
//     //     match self { Playlist::ReleaseRadar(playlist) => {playlist.clone().tracks.items} }
//     //     todo!()
//     // }
// }
// trait PlaylistActions {
//     // fn add_tracks(&self, track_ids: Vec<TrackId>);
//     // fn remove_tracks(&self, track_ids: Vec<TrackId>);
//     // fn reorder_tracks(&self, track_ids: Vec<TrackId>);
//     // fn replace_tracks(&self, track_ids: Vec<TrackId>);
//     // fn get_playlist_tracks(&self) {
//     //     self.clone().tracks.items
//     // };
//     // fn get_playlist_tracks_info(&self);
//     // fn get_playlist_tracks_properties(&self);
//     // fn get_playlist_tracks_audio_features(&self);
//     // fn get_playlist_tracks_audio_analysis(&self);
//
//     // fn get_playlist(&self);
//     // fn delete_playlist(&self);
//     // fn create_playlist(&self);
//     // fn upload_cover_image(&self);
//     // fn get_playlist_cover_image(&self);
//     // fn upload_playlist_cover_image(&self);
//     // fn get_playlist_tracks_recommendations(&self);
//     // fn get_playlist_tracks_recommendations_properties(&self);
//     // fn get_playlist_tracks_recommendations_audio_features(&self);
//     // fn get_playlist_tracks_recommendations_audio_analysis(&self);
// }

// #[derive(PartialEq)]
#[allow(unused_macros)]
macro_rules! zip_to_longest {
    ($vectors:expr, $length:literal, $headers:expr, $default:expr) => {{
        let longest_vector_len = $vectors.iter().map(|v| v.len()).max().unwrap_or(0);
        let mut zipped = Vec::with_capacity(longest_vector_len);
        zipped.push($headers);
        // for i in 0..longest_vector_len {
        //     let tuple = $vectors.into_iter().map(|v| {
        //     // let tuple = $vectors.iter().map(|v| {
        //         if i < v.len() {
        //             v[i].clone()
        //         } else {
        //             $default.clone()
        //         }
        //     }).collect::<[String; $length]>();
        //     // }).collect::<(String, String, String)>();
        //     // }).collect::<Vec<String>>();
        //     // let mut tuple = Vec::with_capacity($vectors.len());
        //     // for v in &$vectors {
        //     //     tuple.push(v.get(i).cloned());
        //     // }
        //     zipped.push(tuple);
            for i in 0..longest_vector_len {
            let tuple: (String, String, String) = $vectors.iter().map(|v| {
                if i < v.len() {
                    v[i].clone()
                } else {
                    $default.clone()
                }
            }).collect::<Vec<String>>().try_into().unwrap_or_else(|v: Vec<String>| {
                panic!("Expected a Vec of length {} but it was {}", $length, v.len())
            });
            zipped.push(tuple);
        }
        zipped
    }};
}
#[allow(dead_code)]
pub struct ComparePlaylists {
    client: AuthCodeSpotify,
    playlist: FullPlaylist,
    pub stored_tracks: StoredTracks,
}

impl PartialEq for ComparePlaylists {
    fn eq(&self, other: &Self) -> bool {
        let pl_ids = self.playlist.id == other.playlist.id;
        pl_ids
    }
}
#[allow(dead_code)]
impl ComparePlaylists {
    pub async fn new(playlist: FullPlaylist) -> Self {
        dotenv().ok();
        let scopes = scopes!(
            "playlist-read-private",
            "playlist-read-collaborative",
            "playlist-modify-public",
            "playlist-modify-private"
        );
        
        ComparePlaylists {
            client: setup(Some(scopes)).await,
            playlist: playlist.clone(),
            stored_tracks: StoredTracks::from_playlist(playlist.tracks.clone()),
        }
    }
    pub fn eq_len(&self, other: &Self) -> bool {
        println!("Playlist lengths are equal: {:?}", self.playlist.tracks.total == other.playlist.tracks.total);
        println!("Playlist 1 length: {:?}", self.playlist.tracks.total);
        println!("Playlist 2 length: {:?}", other.playlist.tracks.total);
        self.playlist.tracks.total == other.playlist.tracks.total
    }
    pub fn comp_metadata(&self, other: &Self) -> bool {
        let eq_id = self.playlist.id == other.playlist.id;
        if eq_id {
            println!("Playlists are equal.");
            println!("Playlist ID: {:?}", self.playlist.id);
            println!("Playlist name: {:?}", self.playlist.name);
            println!("Playlist owner ID: {:?}", self.playlist.owner.id);
        } else {
            println!("Playlists are not equal.");
            println!("Playlist 1 ID: {:?}", self.playlist.id);
            println!("Playlist 2 ID: {:?}", other.playlist.id);
            println!("Playlist 1 name: {:?}", self.playlist.name);
            println!("Playlist 2 name: {:?}", other.playlist.name);
            println!("Playlist 1 owner ID: {:?}", self.playlist.owner.id);
            println!("Playlist 2 owner ID: {:?}", other.playlist.owner.id);
        }
        eq_id
    }
    // fn eq_
    pub fn comp_tracks(&self, other: &Self) -> Vec<(String, String, String)> {
        let self_ids = self.stored_tracks.tracks.clone().iter().map(|track| format!("{} -\n{:?}", track.name.clone(), track.artist_name.clone())).collect::<Vec<String>>();
        let other_ids = other.stored_tracks.tracks.iter().map(|track| format!("{} -\n{:?}", track.name.clone(), track.artist_name.clone())).collect::<Vec<String>>();
        let self_uniques = self_ids
            .iter()
            .filter(|&id| !other_ids.contains(id))
            .cloned()
            .enumerate()
            .map(|(index, id)| format!("{index}. {id}"))
            .collect::<Vec<String>>();
        let other_uniques = other_ids
            .iter()
            .filter(|id| !self_ids.contains(id))
            .cloned()
            .enumerate()
            .map(|(index, id)| format!("{index}. {id}"))
            .collect::<Vec<String>>();
        let shared = self_ids
            .iter()
            .filter(|id| other_ids.contains(id))
            .cloned()
            .enumerate()
            .map(|(index, id)| format!("{index}. {id}"))
            .collect::<Vec<String>>();
        let heads = (format!("{} Unique Tracks", self.playlist.name.to_string()), "Shared Tracks".to_string(), format!("{} Unique Tracks", other.playlist.name.to_string()));
        let final_vec = Self::combine_vectors(self_uniques, shared, other_uniques, heads, "".to_string());
        final_vec
    }
    pub fn print_comp(&self, f: Vec<(String, String, String)>) {
        let mut table = Table::new();
        f.iter().for_each(|(self_uniques, shared, other_uniques)| {
            if shared == "Shared Tracks" {
                table.set_titles(row![FCbc => self_uniques, shared, other_uniques]);
            } else {
                table.add_row(row![self_uniques, shared, other_uniques]);
            }
        });
        table.printstd();
        // table.print_tty(true).expect("No table rendered");
    }
    fn combine_vectors<T: Clone>(v1: Vec<T>, v2: Vec<T>, v3: Vec<T>, headers: (T, T, T), default: T) -> Vec<(T, T, T)> {
        let first_len = std::cmp::max(v1.len(), v2.len());
        let len = std::cmp::max(first_len, v3.len());
        let mut combined = Vec::with_capacity(len);
        combined.push(headers);
        
        for i in 0..len {
            let elem1 = v1.get(i).cloned().unwrap_or_else(|| default.clone());
            let elem2 = v2.get(i).cloned().unwrap_or_else(|| default.clone());
            let elem3 = v3.get(i).cloned().unwrap_or_else(|| default.clone());
            combined.push((elem1, elem2, elem3));
        }
        combined
    }
}

// REMOVED FROM UPDATE FILE - RELEASE RADAR STRUCT

// pub async fn query_release_radar(&self, full: bool) -> PlaylistId {
//     let pl_id = self.get_rr_id(full);
//
//     let playlist = self
//         .client
//         .playlist(pl_id, None, Some(self.market))
//         .await
//         .unwrap();
//
//     println!("Playlist name: {:?}", playlist.name);
//     println!("Playlist ID: {:?}", playlist.id);
//     println!("Playlist owner: {:?}", playlist.owner.display_name.unwrap());
//     playlist
//         .tracks
//         .items
//         .iter()
//         .enumerate()
//         .for_each(|(index, track)| {
//             if let Some(PlayableItem::Track(ref track)) = track.track {
//                 let artist_names = track
//                     .artists
//                     .iter()
//                     .map(|artist| artist.name.clone())
//                     .collect::<Vec<String>>();
//                 let artist_ids = track
//                     .artists
//                     .iter()
//                     .map(|artist| artist.id.clone().unwrap().id().to_string())
//                     .collect::<Vec<String>>();
//                 let album_artist_names = track
//                     .album
//                     .artists
//                     .iter()
//                     .map(|artist| artist.name.clone())
//                     .collect::<Vec<String>>();
//                 let album_artist_ids = track
//                     .album
//                     .artists
//                     .iter()
//                     .map(|artist| artist.id.clone().unwrap().id().to_string())
//                     .collect::<Vec<String>>();
//
//                 println!("\nTrack Index: {}", index + 1);
//                 println!("Track Name: {:?}", track.name);
//                 println!("Track ID: {:?}", track.id.clone().unwrap().id());
//                 println!("Track Artist Names: {:?}", artist_names);
//                 println!("Track Artist IDs: {:?}", artist_ids);
//                 println!("Track Album: {:?}", track.album.name);
//                 println!("Track Album ID: {:?}", track.album.id.clone().unwrap().id());
//                 println!("Track Album Artist Names: {:?}", album_artist_names);
//                 println!("Track Album Artist IDs: {:?}", album_artist_ids);
//                 println!("Track Album Groups: {:?}", track.album.album_group);
//                 println!(
//                     "Track Album Type: {:?}",
//                     track.album.album_type.clone().unwrap()
//                 );
//                 println!(
//                     "Track Album Release Date: {:?}",
//                     track.album.release_date.clone().unwrap()
//                 );
//                 println!(
//                     "Track Album Release Date Precision: {:?}",
//                     track.album.release_date_precision.clone().unwrap()
//                 );
//                 println!("Track Disk Number: {:?}", track.disc_number);
//                 println!("Track Duration: {:?}", track.duration);
//                 println!("Track Explicit: {:?}", track.explicit);
//                 println!("Track Stored Locally: {:?}", track.is_local);
//                 println!("Track Popularity: {:?}", track.popularity);
//                 println!("Track Playable: {:?}", track.is_playable.clone().unwrap());
//                 println!("Track Number: {:?}", track.track_number);
//             }
//         });
//     playlist.id
// }
//
// pub async fn get_rr_track_albums(&self) -> Vec<Vec<String>> {
//     // let pl_id = PlaylistId::from_id(self.release_radar_id.id()).unwrap();
//     let album_ids = self.get_rr_track_album_ids().await;
//     println!("Number of albums: {}", album_ids.len());
//     let mut album_track_ids = Vec::new();
//     for chunk in album_ids.chunks(20) {
//         let albums = self
//             .client
//             .albums(chunk.to_vec(), Some(self.market))
//             .await
//             .expect("TODO: panic message");
//         // let albums = self.client.albums(album_ids, Some(market)).await.unwrap();
//         // let album = self.client.album(album_ids[1].clone(), Some(market)).await.unwrap();
//
//         albums.iter().enumerate().for_each(|(index, album)| {
//             println!("\nAlbum Index: {}", index + 1);
//             println!("Album Name: {:?}", album.name);
//             println!("Album ID: {:?}", album.id.id());
//             println!("Album Type: {:?}", album.album_type);
//             println!(
//                 "Album Artist Names: {:?}",
//                 album
//                     .artists
//                     .iter()
//                     .map(|artist| artist.name.clone())
//                     .collect::<Vec<String>>()
//             );
//             println!(
//                 "Album Artist IDs: {:?}",
//                 album
//                     .artists
//                     .iter()
//                     .map(|artist| artist.id.clone().unwrap().id().to_string())
//                     .collect::<Vec<String>>()
//             );
//             println!("Album Genres: {:?}", album.genres);
//             println!("Album Popularity: {:?}", album.popularity);
//             println!("Album Release Date: {:?}", album.release_date);
//             println!(
//                 "Album Release Date Precision: {:?}",
//                 album.release_date_precision
//             );
//             println!("Album Total Tracks: {:?}", album.tracks.total);
//             println!("Album Label: {:?}", album.label.clone().unwrap());
//
//             println!(
//                 "Album Tracks: {:?}",
//                 album
//                     .tracks
//                     .items
//                     .iter()
//                     .map(|track| track.name.clone())
//                     .collect::<Vec<String>>()
//             );
//             println!(
//                 "Album Tracks IDs: {:?}",
//                 album
//                     .tracks
//                     .items
//                     .iter()
//                     .map(|track| track.id.clone().unwrap().id().to_string())
//                     .collect::<Vec<String>>()
//             );
//             album_track_ids.push(
//                 album
//                     .tracks
//                     .items
//                     .iter()
//                     .map(|track| track.id.clone().unwrap().id().to_string())
//                     .collect::<Vec<String>>(),
//             );
//         });
//     }
//     return album_track_ids;
// }
//
// pub async fn playlists(&self) {
//     let playlists = self.client.current_user_playlists();
//     playlists
//         .try_for_each_concurrent(10, |playlist| async move {
//             println!("Playlist: {:?}, {:?}", playlist.name, playlist.id);
//             Ok(())
//         })
//         .await
//         .unwrap();
// }
// fn print_all_album_track_ids(album_track_ids: &Vec<Vec<TrackId>>) {
//     album_track_ids
//         .iter()
//         .enumerate()
//         .for_each(|(outer_index, album)| {
//             album
//                 .iter()
//                 .enumerate()
//                 .for_each(|(inner_index, track_id)| {
//                     println!(
//                         "Album {:?} - Track {:?}:\t{:?}",
//                         outer_index + 1,
//                         inner_index + 1,
//                         track_id
//                     );
//                 });
//             print_separator();
//         });
// }
// pub async fn get_full_rr(&self) -> StoredPlaylist {
//     // let pl_id = PlaylistId::from_id(Cow::from(self.my_release_radar_id.clone())).unwrap();
//     let market = Market::Country(UnitedStates);
//
//     let playlist = self
//         .client
//         .playlist(self.my_release_radar_id.clone(), None, Some(market))
//         .await
//         .unwrap();
//     return StoredPlaylist::from_playlist(playlist);
// }
// #[tokio::test]
// async fn test_query_release_radar() {
//     let rr = ReleaseRadar::new().await;
//     let rr_id = rr.query_release_radar(false).await;
//     assert_eq!(rr_id.id(), env::var("RELEASE_RADAR_ID").unwrap());
//     println!("ReleaseRadar successfully queried the Release Radar playlist.");
// }
