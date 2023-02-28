/// Lexer parse the source code and turn them into tokens.
/// Tokens are interpreted by the compiler.
#[derive(Debug)]
pub struct Lexer {
    input: String,
    tokens: Vec<Token>,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lineno: usize,
    text: String,
}

#[derive(Debug)]
pub enum TokenType {
    ///Opcode
    OpCode,
    /// Value
    Value,
    /// Comment
    Comment,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut tokens = Vec::<Token>::new();
        for (i, line) in input.lines().enumerate() {
            if line.starts_with('#') {
                tokens.push(Token {
                    token_type: TokenType::Comment,
                    lineno: i,
                    text: line.to_string(),
                });
                continue;
            }
            for w in line.split_whitespace() {
                // TODO change another approach to check for hex value
                if w.starts_with("0x") {
                    tokens.push(Token {
                        token_type: TokenType::Value,
                        lineno: i,
                        text: w.to_string(),
                    });
                    continue;
                }
                tokens.push(Token {
                    token_type: TokenType::OpCode,
                    lineno: i,
                    text: w.to_string(),
                })
            }
        }
        Lexer { input, tokens }
    }
}

#[cfg(test)]
mod lexer_tests {
    use super::*;
    use eyre::Result;

    #[test]
    fn test_lex() -> Result<()> {
        // use `let test_easm std::fs::read("test.easm");` in CLI
        //
        // File: test.easm
        //
        // PUSH 0x2
        // PUSH 0x2
        // MUL
        // STOP
        let test_easm = "# this line is comment\nPUSH 0x2\nPUSH 0x2\nMUL\nSTOP";
        let lexer = Lexer::new(test_easm.to_string());
        dbg!(lexer);

        Ok(())
    }
}
