use ecma_regex::Regex;

#[test]
fn find() {
    let re = Regex::unstable_from_ecma_literal(r"/\w+/g").unwrap();
    let text = "foo bar baz";
    let mut matches = re.find(text);
    println!("matches: {:?}", matches);
    let mut matches = re.find(text);
    println!("matches2: {:?}", matches);
}