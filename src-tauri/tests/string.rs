use app::ext::string::*;

#[test]
fn test_template() {
    let str = r#"
        ID: 1234
        NAME: test

        #TAG# test #TAG#
    "#.to_string();

    assert_eq!(str.extract("ID:", Some("\n")).unwrap(), "1234");
    assert_eq!(str.extract("NAME:", Some("\n")).unwrap(), "test");
    assert_eq!(str.extract("#TAG#", None).unwrap(), "test");
}

#[test]
fn test_remove_ruby() {
    let s = "頭[あたま]がいっぱい".to_string();
    assert_eq!("頭がいっぱい", s.remove_ruby());
    let s = "女[おんな]の子[こ]".to_string();
    assert_eq!("女の子", s.remove_ruby());
}

#[test]
fn test_format_ruby() {
    let s = "女「おんな」の子「こ」".to_string();
    assert_eq!("女[おんな]の 子[こ]", s.format_ruby());
    let s = "いい加減「かげん」".to_string();
    assert_eq!("いい 加減[かげん]", s.format_ruby());
    let s = "偉[えら]い".to_string();
    assert_eq!(s, s.format_ruby());
    let s = "頭「あたま」がいい".to_string();
    assert_eq!("頭[あたま]がいい", s.format_ruby());
}