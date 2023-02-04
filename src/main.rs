use std::str::FromStr;

use ecma_regex::{Flags, Regex};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() != 3 {
        panic!(r#"Usage: cargo run /hello/g \"hello world\"#);
    }

    // Usage: cargo run "/hello/g" "hello world"
    let input = &args[1];
    let text = &args[2];

    assert!(input.starts_with('/'));

    let (pat, flags) = if input.ends_with('/') {
        (&input[1..input.len() - 1], Flags::empty())
    } else {
        // we already know the input isn't ends with '/'
        let mut end_slash_pos = input.len() - 2;
        while end_slash_pos > 0 {
            if input.as_bytes()[end_slash_pos] == b'/' {
                break;
            }
            end_slash_pos -= 1;
        }
        if end_slash_pos == 0 {
            panic!("Invalid input");
        }
        let flags = Flags::from_str(&input[(end_slash_pos + 1)..]).unwrap();
        (&input[1..end_slash_pos], flags)
    };

    let regex = Regex::with_flags(pat, flags).unwrap();

    println!("is_match: {}", regex.is_match(text));

}