mod common;

#[cfg(test)]
#[cfg_attr(debug_assertions, allow(unused_imports))]
mod tests {
    use crate::common;
    use rspotify::model::PlaylistId;
    use std::borrow::Cow;

    fn test_setup() {
        let setup_vars = common::setup();
        assert_eq!(setup_vars.len(), 1);
    }
}
