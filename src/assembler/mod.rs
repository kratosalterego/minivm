pub mod lexer;
pub mod parser;
pub mod encoder;
pub mod diagnostics;

use crate::error::Result; 

pub fn compile(source: &str) -> Result<Vec<u8>> {
    let mut lexer = lexer::Lexer::new(source);
    
    let mut tokens = Vec::new();
    loop {
        match lexer.next_token() {
            Ok(token) => {
                let is_eof = token.kind == lexer::TokenKind::EOF;
                tokens.push(token);
                if is_eof {
                    break;
                }
            }
            Err(lex_err) => {
                return Err(format!("Lexical Analysis Error: {}", lex_err).into());
            }
        }
    }

    let mut parser = parser::Parser::new(tokens);
    let program = parser.parse().map_err(|parse_err| {
        format!("Parsing Error: {}", parse_err)
    })?;

    let mut encoder = encoder::Encoder::new();
    let bytecode = encoder.encode(&program).map_err(|encode_err| {
        format!("Encoding Error: {}", encode_err)
    })?;

    Ok(bytecode)
}