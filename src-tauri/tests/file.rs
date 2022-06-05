use app::file::*;
use std::path::Path;

#[test]
fn find_by_extension_test() {
    let file = "tests/test-files/media/test";
    let extensions = vec!["mp4"];
    let ext = find_extension(file, &extensions).unwrap();
    assert!(Path::new(&format!("{}.{}", file, ext)).exists());
}
