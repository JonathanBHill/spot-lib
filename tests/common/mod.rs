use rspotify::model::PlaylistId;
use std::collections::HashMap;

pub fn setup<'a>() -> HashMap<&'static str, PlaylistId<'a>> {
    println!("Setting up tests");
    let mut setup_vars = HashMap::new();
    setup_vars.insert(
        "playlist_id",
        PlaylistId::from_id("37i9dQZF1DXcBWIGoYBM5M").unwrap(),
    );
    return setup_vars;
}
