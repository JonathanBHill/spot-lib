#[allow(dead_code)]
trait Comparisons {
    fn full_compare(&self, other: &Self) -> bool;
    fn partial_compare(&self, other: &Self) -> bool;
    fn metadata_compare(&self, other: &Self) -> bool;
    fn track_compare(&self, other: &Self) -> bool;
    fn artist_compare(&self, other: &Self) -> bool;
}