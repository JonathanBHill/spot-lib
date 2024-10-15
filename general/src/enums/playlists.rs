use std::borrow::Cow;
use std::env;
use rspotify::model::PlaylistId;

pub enum PlaylistType {
    StockRR,
    MyRR,
}

impl PlaylistType {
    pub fn get_id(&self) -> PlaylistId<'static> {
        dotenv::dotenv().ok();
        let env_id = match self {
            PlaylistType::StockRR => "RELEASE_RADAR_ID",
            PlaylistType::MyRR => "MY_RELEASE_RADAR_ID",
        };
        let rr_id = env::var(env_id)
            .expect("Error: The MY_RELEASE_RADAR_ID environmental variable was not found");
        let pl_id = PlaylistId::from_id(Cow::from(rr_id))
            .expect("Error: The PlaylistId could not be created from the playlist ID");
        pl_id.into_static()
    }
}

#[cfg(test)]
mod tests {
    use rspotify::prelude::Id;
    use super::*;
    
    #[test]
    fn test_playlist_types() {
        let myrr = PlaylistType::MyRR.get_id();
        assert_eq!(myrr.id().to_string(), "46mIugmIiN2HYVwAwlaBAr".to_string())
    }
}