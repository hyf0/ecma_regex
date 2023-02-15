use ecma_regex::Regex;

#[test]
fn supports_look_ahead() {
    // From https://stackoverflow.com/questions/71809435/javascript-regex-into-rust-regex
    let re = Regex::new(r"^(?=.*?[A-Z])(?=.*?[a-z])(?=.*?[0-9]).{6,10}$");
    assert!(re.is_ok());
}
