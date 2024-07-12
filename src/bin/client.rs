use std::collections::HashSet;
use std::path::PathBuf;

use rspotify::{AuthCodeSpotify, Config, Credentials, OAuth};
use rspotify::clients::OAuthClient;

pub async fn setup(scopes: HashSet<String>) -> AuthCodeSpotify {
    let creds = Credentials::from_env().unwrap();
    let config = Config {
        cache_path: PathBuf::from("/home/jonathan/RustroverProjects/Spotify/token_cache"),
        token_cached: true,
        token_refreshing: true,
        ..Default::default()
    };
    let oath = OAuth::from_env(scopes).unwrap();

    let spotify_client = AuthCodeSpotify::with_config(creds.clone(), oath.clone(), config.clone());
    let url = spotify_client.get_authorize_url(false).unwrap();
    spotify_client.prompt_for_token(&url).await.unwrap();
    spotify_client
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_client_macro() {
        // debug_assert!(client_macro!(scopes!["user-library-read"]));
        // let client = client_macro!(vec!["user-library-read"]);
        // assert_eq!(client.config.token_cached, true);
        // assert_eq!(client.config.token_refreshing, true);
        // assert_eq!(client.oath.scopes, vec!["user-library-read"]);
    }
}

fn main() {}
