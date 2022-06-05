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
