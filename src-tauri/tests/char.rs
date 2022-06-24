use app::ext::char::CharExt;

#[test]
fn test_is_kanji() {
    let s = "漢字".to_string();
    for c in s.chars() {
        assert!(c.is_kanji());
    }
    let s = "ひらがな・カタカナ";
    for c in s.chars() {
        assert!(!c.is_kanji());
    }
}