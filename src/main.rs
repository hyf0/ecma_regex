use ecma_regex::{Regex};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() != 3 {
        panic!(r#"Usage: cargo run /hello/g "hello world"#);
    }

    // Usage: cargo run "/hello/g" "hello world"
    let input = &args[1];
    let text = &args[2];

    let regex = Regex::from_ecma_literal(input).unwrap();

    println!("is_match: {}", regex.is_match(text));
}
