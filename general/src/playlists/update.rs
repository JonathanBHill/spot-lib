use dotenv::dotenv;
use rspotify::clients::{BaseClient, OAuthClient};
use rspotify::model::{AlbumId, FullPlaylist, Market, PlayableItem, PlaylistId, TrackId};
use rspotify::{prelude::*, scopes, AuthCodeSpotify};
use crate::enums::playlists::PlaylistType;
// use crate::traits::playlists::PlaylistBehavior;

use crate::traits::utilities::SpotifyDefaults;
use crate::utils::client::setup;
use crate::utils::misc::print_separator;

pub struct ReleaseRadar {
    pub client: AuthCodeSpotify,
    release_radar_id: PlaylistId<'static>,
    my_release_radar_id: PlaylistId<'static>,
    market: Market,
}
impl SpotifyDefaults for ReleaseRadar {}
// impl PlaylistBehavior for ReleaseRadar {
//
//     type ClientType = AuthCodeSpotify;
//
//     fn client(&self) -> &Self::ClientType {
//         &self.client
//     }
//
//     fn playlist_id(&self) -> &PlaylistId<'static> {
//         &self.release_radar_id
//     }
//
//     fn market(&self) -> Market {
//         self.market
//     }
// }
impl ReleaseRadar {
    /// Creates a new `Release` instance.
    ///
    /// This function initializes the Spotify client with the necessary scopes and retrieves
    /// the playlist IDs from environment variables.
    ///
    /// # Returns
    /// A new `Release` instance.
    pub async fn new() -> Self {
        dotenv().ok();
        let scopes = scopes!(
            "playlist-read-private",
            "playlist-read-collaborative",
            "playlist-modify-public",
            "playlist-modify-private"
        );

        ReleaseRadar {
            client: setup(Some(scopes)).await,
            release_radar_id: PlaylistType::StockRR.get_id(),
            my_release_radar_id: PlaylistType::MyRR.get_id(),
            market: <ReleaseRadar as SpotifyDefaults>::market(),
        }
    }
    /// Retrieves the last updated date of the user's Release Radar playlist.
    ///
    /// This function fetches the playlist details from Spotify and extracts the last updated
    /// date from the playlist description.
    ///
    /// # Returns
    /// A string representing the last updated date of the playlist.
    pub async fn get_rr(&self, rr_type: bool) -> FullPlaylist {
        let pl_id = match rr_type {
            true => self.my_release_radar_id.clone(),
            false => self.release_radar_id.clone(),
        };
        let playlist = self
            .client
            .playlist(pl_id.clone(), None, Some(self.market))
            .await
            .unwrap();
        return playlist;
    }
    /// Retrieves the album IDs of tracks in the Release Radar playlist.
    ///
    /// This function fetches the Release Radar playlist and extracts the album IDs of the tracks.
    ///
    /// # Returns
    /// A vector of `AlbumId` representing the album IDs of the tracks in the Release Radar playlist.
    pub async fn get_rr_track_album_ids(&self) -> Vec<AlbumId> {
        let playlist = self
            .client
            .playlist(self.release_radar_id.clone(), None, Some(self.market))
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
    /// Queries the Release Radar playlist and prints track details.
    ///
    /// This function fetches the Release Radar playlist based on the `rr_type` and prints
    /// the track name, album name, and artists for each track.
    ///
    /// # Arguments
    /// * `rr_type` - A boolean indicating which Release Radar playlist to query.
    ///               `true` for the user's Release Radar, `false` for the stock Release Radar.
    pub async fn query_rr(&self, rr_type: bool) {
        let pl_id = match rr_type {
            true => self.my_release_radar_id.clone(),
            false => self.release_radar_id.clone(),
        };
        let playlist = self
            .client
            .playlist(pl_id.clone(), None, Some(self.market))
            .await
            .unwrap();
        let tracks = playlist.tracks.items;
        for track in tracks {
            match track.track {
                Some(PlayableItem::Track(ref track)) => {
                    println!("Track: {:?}", track.name);
                    println!("Album: {:?}", track.album.name);
                    println!("Artists: {:?}", track.artists);
                    print_separator();
                }
                _ => (),
            }
        }
    }
    /// Retrieves the track IDs of all tracks in the albums from the Release Radar playlist.
    ///
    /// This function fetches the album IDs from the Release Radar playlist, retrieves the tracks
    /// from each album, and returns a vector of unique track IDs. Optionally prints the track IDs.
    ///
    /// # Arguments
    /// * `print` - A boolean indicating whether to print the track IDs.
    ///
    /// # Returns
    /// A vector of `TrackId` representing the track IDs of all tracks in the albums from the Release Radar playlist.
    pub async fn get_album_tracks_from_rr(&self, print: bool) -> Vec<TrackId> {
        let album_ids = self.get_rr_track_album_ids().await;
        println!("Number of albums: {}", album_ids.len());
        let mut return_vector = Vec::new();
        let mut album_track_ids = Vec::new();
        for chunk in album_ids.chunks(20) {
            let albums = self
                .client
                .albums(chunk.to_vec(), Some(self.market))
                .await
                .expect("TODO: panic message");

            albums.iter().for_each(|album| {
                // let hst = HashSet::from(album.tracks.items.iter().map(|track| track.id.clone().unwrap()).collect::<Vec<TrackId>>());
                return_vector = Self::append_uniques(
                    &return_vector,
                    &album
                        .tracks
                        .items
                        .iter()
                        .map(|track| track.id.clone().unwrap())
                        .collect::<Vec<TrackId>>(),
                );
                // album.tracks.items.iter().
                album_track_ids.push(
                    album
                        .tracks
                        .items
                        .iter()
                        .map(|track| track.id.clone().unwrap())
                        .collect::<Vec<TrackId>>(),
                );
            });
        }
        if print {
            Self::print_all_album_track_ids(&album_track_ids);
        };
        return return_vector;
    }
    /// Appends unique track IDs from the new vector to the existing vector.
    ///
    /// This function takes two vectors of `TrackId` and appends only the unique track IDs
    /// from the new vector to the existing vector.
    ///
    /// # Arguments
    /// * `existing` - A vector of existing `TrackId`s.
    /// * `new` - A vector of new `TrackId`s to be appended.
    ///
    /// # Returns
    /// A vector of `TrackId` containing unique track IDs from both vectors.
    fn append_uniques<'a>(existing: &Vec<TrackId<'a>>, new: &Vec<TrackId<'a>>) -> Vec<TrackId<'a>> {
        let mut extended = existing.clone();
        let intersection: Vec<_> = existing
            .iter()
            .filter(|x| new.contains(x))
            .cloned()
            .collect();
        extended.extend(new.iter().filter(|x| !intersection.contains(x)).cloned());
        return extended;
    }
    /// Updates the Release Radar playlist with new tracks.
    ///
    /// This function fetches the track IDs from the albums in the Release Radar playlist,
    /// and updates the playlist with these tracks. Optionally prints the track IDs.
    ///
    /// # Arguments
    /// * `print` - A boolean indicating whether to print the track IDs.
    pub async fn update_rr(&self, print: bool) {
        let ids = self.get_album_tracks_from_rr(false).await;
        let pl_id = PlaylistId::from_id(self.my_release_radar_id.id()).unwrap();
        let chunks = ids.chunks(20);
        let mut replace = true;
        for chunk in chunks {
            let chunk_iterated = chunk
                .into_iter()
                .map(|track| track.id().to_string())
                .collect::<Vec<String>>();

            if replace {
                if print {
                    println!("Replacing playlist with the first {:?} tracks", chunk.len());
                }
                self.client
                    .playlist_replace_items(
                        pl_id.clone(),
                        chunk_iterated
                            .into_iter()
                            .map(|track_id| PlayableId::Track(TrackId::from_id(track_id).unwrap())),
                    )
                    .await
                    .expect("Track IDs should be assigned to chunk_iterated as type TrackID");
                replace = false;
            } else {
                if print {
                    println!("Adding {:?} tracks to the playlist", chunk.len());
                }
                self.client
                    .playlist_add_items(
                        pl_id.clone(),
                        chunk_iterated
                            .into_iter()
                            .map(|track_id| PlayableId::Track(TrackId::from_id(track_id).unwrap())),
                        Option::None,
                    )
                    .await
                    .expect("Track IDs should be assigned to chunk_iterated as type TrackID");
            }
        }
    }
    
    /// Prints all album track IDs.
    ///
    /// This function takes a vector of vectors containing `TrackId`s and prints each track ID
    /// along with its album and track index.
    ///
    /// # Arguments
    /// * `album_track_ids` - A vector of vectors containing `TrackId`s.
    fn print_all_album_track_ids(album_track_ids: &Vec<Vec<TrackId>>) {
        album_track_ids
            .iter()
            .enumerate()
            .for_each(|(outer_index, album)| {
                album
                    .iter()
                    .enumerate()
                    .for_each(|(inner_index, track_id)| {
                        println!(
                            "Album {:?} - Track {:?}:\t{:?}",
                            outer_index + 1,
                            inner_index + 1,
                            track_id
                        );
                    });
                print_separator();
            });
    }
}

#[cfg(test)]
#[cfg_attr(debug_assertions, allow(unused_variables, dead_code))]
mod tests {
    use rspotify::model::Country::UnitedStates;
    use super::*;
    use crate::utils::misc::get_type;
    use rspotify::model::Market;

    #[tokio::test]
    async fn test_release_radar_instantiation() {
        let rr = ReleaseRadar::new().await;

        let playlist_scopes = scopes!(
            "playlist-read-private",
            "playlist-modify-private",
            "playlist-modify-public",
            "playlist-read-collaborative"
        );
        assert_eq!(rr.client.get_oauth().scopes, playlist_scopes);
        assert_eq!(
            get_type(&rr.market, true),
            get_type(&Market::Country(UnitedStates), true)
        );
        println!("ReleaseRadar was successfully instantiated with the correct market country and correct scopes.");
    }
    #[test]
    fn test_append_uniques() {
        let a = vec![
            TrackId::from_id("1").unwrap(),
            TrackId::from_id("2").unwrap(),
            TrackId::from_id("3").unwrap(),
        ];
        let b = vec![
            TrackId::from_id("3").unwrap(),
            TrackId::from_id("4").unwrap(),
            TrackId::from_id("5").unwrap(),
        ];
        let c = vec![
            TrackId::from_id("1").unwrap(),
            TrackId::from_id("2").unwrap(),
            TrackId::from_id("3").unwrap(),
            TrackId::from_id("4").unwrap(),
            TrackId::from_id("5").unwrap(),
        ];
        let appended = ReleaseRadar::append_uniques(&a, &b);
        assert_eq!(appended, c);
        println!("ReleaseRadar successfully appended only unique tracks.");
    }
    // #[tokio::test]
    // async fn test_get_rr_id() {
    //     let rr = ReleaseRadar::new();
    //     let await_rr = rr.await;
    //     let rr_id = await_rr.get_rr_id(true);
    //     assert_eq!(rr_id.id(), env::var("MY_RELEASE_RADAR_ID").unwrap());
    //     println!("ReleaseRadar successfully retrieved the correct Release Radar ID.");
    // }
}
