
/// A parse tree has nodes corresponding directly to the rules of a grammar.
#[derive(Debug, Clone)]
pub enum ParseTree <'a> {
    /// equivalent to ""
    Empty,
    /// terminal symbol
    Terminal(&'a str),
    /// identifier, in this case it is non-terminal symbol
    // NonTerminal(&'a str),
    NonTerminalDefinition(&'a str, Box<ParseTree<'a>>),
    /// ordered choice
    Choice(Vec<ParseTree<'a>>),
    /// optional choice
    Optional(Box<ParseTree<'a>>),
    /// repetition
    Many(Box<ParseTree<'a>>),
}
