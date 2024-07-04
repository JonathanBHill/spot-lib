use crate::models::tracks::StoredTrack;

pub enum TrackType {
    Track(StoredTrack),
    Tracks(Vec<StoredTrack>),
}
