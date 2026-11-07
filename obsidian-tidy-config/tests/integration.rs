//! Integration tests for configuration round-trip.
//!
//! These tests verify that `Config` can be saved to JSON and loaded back
//! without loss of information, using the public API only.

use obsidian_tidy_config::{Config, template::Template};
use obsidian_tidy_rules::ALL_RULES_FABRICS;
use std::io::Cursor;

#[test]
fn save_and_load() {
    let config = Config::new(Template::Standard);

    let mut buffer = Vec::new();
    config
        .saver()
        .pretty(true)
        .save(&mut buffer)
        .expect("failed save");

    let loaded_config = Config::loader(&ALL_RULES_FABRICS)
        .load(Cursor::new(&buffer))
        .expect("failed load");

    insta::assert_json_snapshot!(loaded_config);
}
