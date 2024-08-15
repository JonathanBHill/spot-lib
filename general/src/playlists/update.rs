use dotenv::dotenv;
use rspotify::clients::{BaseClient, OAuthClient};
use rspotify::model::{AlbumId, FullPlaylist, Market, PlayableItem, PlaylistId, TrackId};
use rspotify::{prelude::*, scopes, AuthCodeSpotify};
use std::borrow::Cow;
use std::env;

use crate::traits::utilities::SpotifyDefaults;
use crate::utils::client::setup;
use crate::utils::misc::print_separator;

#[derive(Clone, Debug)]
pub struct ReleaseRadar {
    pub client: AuthCodeSpotify,
    release_radar_id: PlaylistId<'static>,
    my_release_radar_id: PlaylistId<'static>,
    market: Market,
}
impl SpotifyDefaults for ReleaseRadar {}
impl ReleaseRadar {
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
            release_radar_id: PlaylistId::from_id(Cow::from(env::var("RELEASE_RADAR_ID").unwrap()))
                .unwrap(),
            my_release_radar_id: PlaylistId::from_id(Cow::from(
                env::var("MY_RELEASE_RADAR_ID").unwrap(),
            ))
            .unwrap(),
            market: <ReleaseRadar as SpotifyDefaults>::market(),
        }
    }
    fn get_rr_id(&self, rr_type: bool) -> PlaylistId {
        let playlist_id = match rr_type {
            true => match PlaylistId::from_id(self.my_release_radar_id.id()) {
                Ok(id) => id,
                Err(e) => panic!("Error: {:?}", e),
            },
            false => match PlaylistId::from_id(self.release_radar_id.id()) {
                Ok(id) => id,
                Err(e) => panic!("Error: {:?}", e),
            },
        };
        playlist_id
    }
    pub async fn get_rr(&self, rr_type: bool) -> FullPlaylist {
        let pl_id = self.get_rr_id(rr_type);
        let playlist = self
            .client
            .playlist(pl_id.clone(), None, Some(self.market))
            .await
            .unwrap();
        return playlist;
    }

    pub async fn get_rr_track_album_ids(&self) -> Vec<AlbumId> {
        // let pl_id = PlaylistId::from_id(Cow::from(self.release_radar_id.clone())).unwrap();

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

    pub async fn query_rr(&self, rr_type: bool) {
        let pl_id = self.get_rr_id(rr_type);
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
    // pub async fn get_album_tracks_from_rr(&self, print: bool) -> HashSet<TrackId> {
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
    async fn change_description(&self) {
        let pl_id = PlaylistId::from_id(self.my_release_radar_id.id()).unwrap();
        let date = chrono::Local::now().format("%m/%d/%Y");
        let description = format!(
            "Release Radar playlist with songs from albums included. \
            Created on 11/02/2023. Updated on {}.", date
        );
        self.client
            .playlist_change_detail(pl_id, None, None, Some(&*description), None)
            .await
            .expect("Description should be assigned to description as type &str");
    }
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
                self.change_description().await;
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
    pub async fn get_last_updated(&self) -> String {
        let pl_id = PlaylistId::from_id(self.my_release_radar_id.id()).unwrap();
        let playlist = self
            .client
            .playlist(pl_id.clone(), None, Some(self.market))
            .await
            .unwrap();
        let description = playlist.description.unwrap_or("".to_string());
        let last_updated_date = description.split("Updated on ").collect::<Vec<&str>>()[1];
        format!("Last Updated on: {:?}", last_updated_date)
    }
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
    #[tokio::test]
    async fn test_get_rr_id() {
        let rr = ReleaseRadar::new();
        let await_rr = rr.await;
        let rr_id = await_rr.get_rr_id(true);
        assert_eq!(rr_id.id(), env::var("MY_RELEASE_RADAR_ID").unwrap());
        println!("ReleaseRadar successfully retrieved the correct Release Radar ID.");
    }
}
