use rspotify::clients::OAuthClient;
use rspotify::{AuthCodeSpotify, Config, Credentials, OAuth};
use std::collections::HashSet;
use std::path::PathBuf;
pub async fn setup(scopes: Option<HashSet<String>>) -> AuthCodeSpotify {
    let creds = Credentials::from_env().unwrap();
    let config = Config {
        cache_path: PathBuf::from("/home/jonathan/RustroverProjects/Spotify/token_cache"),
        token_cached: true,
        token_refreshing: true,
        ..Default::default()
    };
    if let Some(scopes) = scopes {
        let oath = OAuth::from_env(scopes).unwrap();
        let spotify_client = AuthCodeSpotify::with_config(creds.clone(), oath.clone(), config.clone());
        let url = spotify_client.get_authorize_url(false).unwrap();
        spotify_client.prompt_for_token(&url).await.unwrap();
        return spotify_client;
    } else {
        let oath = OAuth::from_env(HashSet::new()).unwrap();
        let spotify_client = AuthCodeSpotify::with_config(creds.clone(), oath.clone(), config.clone());
        let url = spotify_client.get_authorize_url(false).unwrap();
        spotify_client.prompt_for_token(&url).await.unwrap();
        return spotify_client;
    }
}
