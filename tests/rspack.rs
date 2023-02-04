use ecma_regex::Regex;

#[test]
fn test_basic() {
    // should not panic
    assert!(Regex::new("test\\\\").is_ok());
}

#[test]
fn test_js_regex() {
    assert!(Regex::new("test.*").unwrap().is_match("testaaaaaaa"));
    assert!(!Regex::new("test.*").unwrap().is_match("tesaaaaaaa"));
}
