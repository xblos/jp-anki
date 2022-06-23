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
fn test_ignore_between() {
    let s = "頭[あたま]がいっぱい".to_string();
    assert_eq!("頭がいっぱい", s.ignore_ruby());
    let s = "女[おんな]の子[こ]".to_string();
    assert_eq!("女の子", s.ignore_ruby());
}

#[test]
fn test_replace_ruby_parenthesis() {
    let s = "男「おとこ」の子「こ」".to_string();
    assert_eq!("男[おとこ]の子[こ]", s.replace_ruby_parentheses());
}