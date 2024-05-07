use std::path::Path;

use crate::source::SourcePosition;

#[test]
fn source_position_equality_test() {
    let positon_one: SourcePosition<'_> = SourcePosition::new(None, 1, 1);
    let position_two: SourcePosition<'_> = SourcePosition::new(Some(Path::new("foo.bar")), 1, 1);
    let position_three: SourcePosition<'_> = SourcePosition::new(Some(Path::new("foob.ar")), 1, 1);

    assert_eq!(positon_one, position_two);
    assert_ne!(position_two, position_three);
    assert_eq!(positon_one, position_three);
}

#[test]
fn source_position_ordering_test() {
    let position_one: SourcePosition<'_> = SourcePosition::new(None, 1, 1);
    let position_two: SourcePosition<'_> = SourcePosition::new(Some(Path::new("foo.bar")), 2, 1);

    assert!(position_one < position_two);
    assert!(position_two > position_one);
}
