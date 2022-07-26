use app::kanji::{generate_furigana, rubify};

#[test]
fn test_generate_furigana() {
    let pairs = generate_furigana(
        "侍は強かった",
        "さむらいはつよかった"
    ).unwrap();
    assert_eq!("侍", pairs[0][0]);
    assert_eq!("さむらい", pairs[0][1]);
    assert_eq!("は", pairs[1][0]);
    assert_eq!("", pairs[1][1]);
    assert_eq!("強", pairs[2][0]);
    assert_eq!("つよ", pairs[2][1]);
    assert_eq!("かった", pairs[3][0]);
    assert_eq!("", pairs[3][1]);

    let pairs = generate_furigana("破壊", "はかい").unwrap();
    assert_eq!("破壊", pairs[0][0]);
    assert_eq!("はかい", pairs[0][1]);
}

#[test]
fn test_generate_ruby_string() {
    let s = rubify(
        "今日は本当に暑いね",
        "きょうはほんとうにあついね"
    ).unwrap();
    assert_eq!("今日[きょう]は 本当[ほんとう]に 暑[あつ]いね", s);
}