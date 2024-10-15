use std::ops::Index;

use anyhow::{Ok, Result};
use rspotify::{AuthCodeSpotify, scopes};
use rspotify::model::{FullPlaylist, Id, PlaylistId, SearchResult, SearchType, SimplifiedPlaylist};
use rspotify::prelude::BaseClient;

use crate::traits::utilities::SpotifyDefaults;
use crate::utils::client;
use crate::utils::client::setup;

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
// pub async fn query_playlist(playlist_name: String) {
//     let scope = scopes!(
//             "playlist-read-private",
//             "playlist-read-collaborative",
//             "playlist-modify-public",
//             "playlist-modify-private"
//         );
//     let client = setup(Some(scope)).await;
//     let market = Defaults::market();
//     let results = client.search(&playlist_name, SearchType::Playlist, Some(market), None, None, None).await.unwrap();
//     println!("Results: {:?}", results);
// }
pub struct PlaylistQuery {
    pub client: AuthCodeSpotify,
}

impl SpotifyDefaults for PlaylistQuery {}
impl PlaylistQuery {
    pub async fn new() -> Self {
        let scope = scopes!(
            "playlist-read-private",
            "playlist-read-collaborative",
            "playlist-modify-public",
            "playlist-modify-private"
        );
        PlaylistQuery {
            client: setup(Some(scope)).await,
        }
    }
    fn construct_pattern(&self, words: Vec<&str>) -> String {
        // Create the base pattern with case-insensitive flag
        let mut pattern = String::from("(?i).*");
        
        // Iterate over the words and construct the pattern
        for (i, word) in words.iter().enumerate() {
            if i > 0 {
                pattern.push_str(".*"); // Match any characters between words
            }
            pattern.push_str(r"\b");
            pattern.push_str(word);
            pattern.push_str(r"\b");
        }
        pattern.push_str(".*");
        
        pattern
    }
    pub async fn query_playlist(&self, playlist_name: String) -> Result<FullPlaylist> {
        let market = <PlaylistQuery as SpotifyDefaults>::market();
        let results = self.client.search(&playlist_name, SearchType::Playlist, Some(market), None, Some(50), None).await.unwrap();
        let pl_name_vec = playlist_name.split(" ").collect::<Vec<&str>>();
        let regex_pattern = self.construct_pattern(pl_name_vec);
        let regex_match = regex::Regex::new(regex_pattern.as_str()).unwrap();
        match results {
            SearchResult::Playlists(paginator) => {
                let oop = paginator.clone().items.into_iter().filter(|pl| {
                    regex_match.is_match(pl.name.as_str()) 
                }).collect::<Vec<SimplifiedPlaylist>>();
                
                let sel = 
                    dialoguer::Select::new()
                        .items(&oop.clone().iter().map(|pl| {
                            if let Some(displayname) = &pl.owner.display_name {
                                format!("{} - {:?}", pl.name.as_str(), displayname)
                            } else {
                                format!("{}", pl.name.as_str())
                            }
                        }).collect::<Vec<String>>())
                        .interact().unwrap();
                
                let selected = oop.index(sel);
                if let Some(displayname) = &selected.owner.display_name {
                    println!("Selection: {:?} by {:?}", selected.name, displayname);
                    println!("Selection ID: {:?}", selected.id);
                } else {
                    println!("Selection: {:?}", selected.name);
                
                };
                Ok(self.client.playlist(oop.index(sel).id.clone(), None, Some(market)).await?)
            }
            _ => {
                println!("Error: {:?}", results);
                // panic!("Error: {:?}", results)
                Ok(self.client.playlist(PlaylistId::from_id("37i9dQZEVXbdINACbjb1qu").unwrap(), None, Some(market)).await?)
            }
        }
    }
}
