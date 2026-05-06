mod common;

use obsidian_tidy_config::{Config, ConfigLoader, ConfigSaver};
use obsidian_tidy_core::rule::Rules;
use std::io::Cursor;

#[test]
fn save_and_load() {
    let config = Config {
        rules: common::test_rules(),
    };

    let mut buffer = Vec::new();
    ConfigSaver::new(&config)
        .save(&mut buffer)
        .expect("failed save");

    let registry = common::test_registry_fabric();
    let loaded_config = ConfigLoader::new(&registry)
        .load(Cursor::new(&buffer))
        .expect("failed load");

    let json = String::from_utf8(buffer).unwrap();

    insta::assert_json_snapshot!(loaded_config);
    insta::assert_snapshot!(json);
}

#[test]
fn save_and_load_empty_config() {
    let original_config = Config {
        rules: Rules::new(),
    };

    let mut buffer = Vec::new();
    ConfigSaver::new(&original_config)
        .save(&mut buffer)
        .expect("failed to save");

    let registry = common::test_registry_fabric();
    let loaded_config = ConfigLoader::new(&registry)
        .load(Cursor::new(&buffer))
        .expect("failed to load");

    insta::assert_json_snapshot!(loaded_config);
}
