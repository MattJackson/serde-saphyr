//! `Options::quoted_scalars_are_strings`: an untagged quoted scalar is a string only, as
//! `serde_yaml` reads it.
#![cfg(feature = "deserialize")]

use serde::Deserialize;
use serde_saphyr::{Options, from_str_with_options};

fn strict() -> Options {
    serde_saphyr::options! { quoted_scalars_are_strings: true }
}

#[derive(Debug, Deserialize, PartialEq)]
struct Typed {
    n: Option<u64>,
}

#[test]
fn default_reads_quoted_numbers_into_numeric_targets() {
    assert!(!Options::default().quoted_scalars_are_strings);
    let read: u64 = from_str_with_options("'1024'", Options::default()).unwrap();
    assert_eq!(read, 1024);
}

#[test]
fn quoted_scalars_are_refused_for_numeric_and_boolean_targets() {
    for yaml in ["'1024'", "\"1024\""] {
        assert!(
            from_str_with_options::<u64>(yaml, strict()).is_err(),
            "{yaml}"
        );
        assert!(
            from_str_with_options::<i32>(yaml, strict()).is_err(),
            "{yaml}"
        );
        assert!(
            from_str_with_options::<f64>(yaml, strict()).is_err(),
            "{yaml}"
        );
        assert!(
            from_str_with_options::<Typed>(&format!("n: {yaml}"), strict()).is_err(),
            "{yaml}"
        );
    }
    assert!(from_str_with_options::<bool>("'true'", strict()).is_err());
}

#[test]
fn plain_and_tagged_scalars_and_string_targets_are_unaffected() {
    assert_eq!(
        from_str_with_options::<u64>("1024", strict()).unwrap(),
        1024
    );
    assert_eq!(
        from_str_with_options::<u64>("!!int '1024'", strict()).unwrap(),
        1024
    );
    assert!(from_str_with_options::<bool>("true", strict()).unwrap());
    assert_eq!(
        from_str_with_options::<String>("'1024'", strict()).unwrap(),
        "1024"
    );
    assert_eq!(
        from_str_with_options::<Typed>("n: 7", strict()).unwrap(),
        Typed { n: Some(7) }
    );
}
