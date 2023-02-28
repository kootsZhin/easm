use crate::lexer::Token;

/// Compiler compile tokens and returns a binary string
/// that can be interpreted by the EVM.
pub struct Compiler {
    source: Vec<Token>,
}

impl Compiler {
    pub fn compile(&self) {}
}
