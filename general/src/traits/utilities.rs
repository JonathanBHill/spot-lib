use rspotify::model::Market;
use rspotify::scopes;
use std::collections::HashSet;
use std::io::Write;

pub trait Printables {
    fn properties(&self);
    fn info(&self);
    fn item_properties(&self);
}
pub trait SpotifyDefaults {
    fn market() -> Market {
        Market::Country(rspotify::model::Country::UnitedStates)
    }
    fn scopes_rr() -> HashSet<String> {
        scopes!(
            "playlist-read-private",
            "playlist-read-collaborative",
            "playlist-modify-public",
            "playlist-modify-private"
        )
    }
}
pub trait JsonConfig {
    fn to_json(&self) -> String where Self: serde::Serialize {
        if let Ok(json) = json5::to_string(&self) {
            json
        } else {
            String::from("Unable to convert properties into a json5 string.")
        }
    }
    fn write_to_file(&self, path: &str) -> std::io::Result<()> where Self: serde::Serialize {
        let json = self.to_json();
        if let Ok(mut file) = std::fs::File::create(path) {
            if let Ok(_) = file.write_all(json.as_bytes()) {
                return Ok(());
            }
        }
        Ok(())
    }
}
