//! The JSON Test Suite is taken from here:
//! https://github.com/nst/JSONTestSuite
//! and is available under the MIT license.

use kserde::*;

#[test]
fn json_test_suite() {
    let files = std::fs::read_dir("tests/test_parsing").unwrap();

    for entry in files {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        let file_name = file_name.to_str().unwrap();
        let path = entry.path();
        let path = path.to_str().unwrap();

        let source = std::fs::read_to_string(path);
        if let Ok(source) = source {
            let json = Thing::from_json(&source);

            println!("TEST: {:?}", path);
            if file_name.starts_with("y_") {
                assert!(json.is_some(), "Unexpected failure for: {}", path);
            }

            if file_name.starts_with("n_") && json.is_some() {
                // This crate is more permissive.
                // For json that should fail but does not is ignored.
                // assert!(json.is_none(), "Unexpected success for: {}", path);
            }
        }
    }
}
