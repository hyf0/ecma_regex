# ecma_regex

The goal of `ecma_regex` is to provide the same functionality as the [regex](https://github.com/rust-lang/regex) crate in ECMAScript regular expression syntax.

- Reliable regex engine from QuickJS
- Passes nearly 100% of the ECMAScript Test Suite tests

Build on top of [libregexp-sys](https://github.com/hyf0/libregexp-sys).

# Difference to [regex](https://github.com/rust-lang/regex)

# Difference to ECMAScript [Regexp](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp)

- `Regexp` in ECMAScript is [stateful](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/lastIndex) while `ecma_regex` is stateless.

- `Regexp` in ECMAScript need to deal with flags like `g`, `m`, `s`, `u`, `y` while `ecma_regex` doesn't. The corresponding behaviors in different flags are implemented as different methods in `ecma_regex` crate.

It's possible to write a `Regexp` crate having the same behavior as ECMAScript `Regexp`, but it's not the goal of this crate.

# Credit

The API try to align with [regex](https://github.com/rust-lang/regex) crate and reuse it's documents, credit to the [author](https://github.com/BurntSushi) and [contributors](https://github.com/rust-lang/regex/graphs/contributors).
