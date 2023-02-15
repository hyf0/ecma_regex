#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Error {
    Syntax(String),
    Runtime(String),
}
