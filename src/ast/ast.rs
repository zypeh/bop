
/// A parse tree has nodes corresponding directly to the rules of a grammar.
#[derive(Debug)]
pub enum ParseTree {
    /// equivalent to ""
    Empty,
    /// terminal symbol
    Terminal(String),
    /// identifier, in this case it is non-terminal symbol
    NonTerminal(String),
    /// ordered choice
    Choice(Vec<ParseTree>),
    /// optional choice
    Optional(Box<ParseTree>),
    /// repetition
    Many(Box<ParseTree>),
}
