use rspotify::model::Market;
use rspotify::scopes;
use std::collections::HashSet;

pub trait Printables {
    fn properties(&self);
    fn info(&self);
    fn item_properties(&self);
}
pub trait Defaults {
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
