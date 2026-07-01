use crate::assembler::lexer::{Token, TokenKind, Location};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand<'a> {
    Register(&'a str),
    Integer(i64),
    LabelRef(&'a str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedInstruction<'a> {
    pub opcode: &'a str,
    pub operands: Vec<Operand<'a>>,
    pub loc: Location,
}

#[derive(Debug, Default)]
pub struct AssembledProgram<'a> {
    pub instructions: Vec<ParsedInstruction<'a>>,
    pub labels: HashMap<&'a str, usize>, 
}

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    cursor: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self { tokens, cursor: 0 }
    }
    pub fn parse(&mut self) -> Result<AssembledProgram<'a>, String> {
        let mut program = AssembledProgram::default();

        while !self.is_at_end() {
            if self.match_token(TokenKind::NewLine) {
                continue;
            }

            if self.check(TokenKind::EOF) {
                break;
            }

            if let Some(TokenKind::LabelDef(name)) = self.peek_kind() {
                let name = *name; 
                self.advance(); 
                
                if program.labels.contains_key(name) {
                    return Err(format!("Duplicate label definition found: '{}'", name));
                }
                program.labels.insert(name, program.instructions.len());
                
                continue;
            }

            if let Some(TokenKind::Instruction(opcode)) = self.peek_kind() {
                let opcode = *opcode;
                let loc = self.peek().unwrap().loc;
                self.advance(); 

                let mut operands = Vec::new();

                if !self.check(TokenKind::NewLine) && !self.check(TokenKind::EOF) {
                    loop {
                        operands.push(self.parse_operand()?);
                        if self.match_token(TokenKind::Comma) {
                            continue;
                        }
                        break;
                    }
                }
                if !self.check(TokenKind::EOF) {
                    self.consume(TokenKind::NewLine, "Expected newline after instruction components")?;
                }

                program.instructions.push(ParsedInstruction {
                    opcode,
                    operands,
                    loc,
                });
                continue;
            }

            let bad_token = self.peek().unwrap();
            return Err(format!(
                "Unexpected token '{:?}' at line {}, col {}",
                bad_token.kind, bad_token.loc.line, bad_token.loc.column
            ));
        }

        Ok(program)
    }

    fn parse_operand(&mut self) -> Result<Operand<'a>, String> {
        let token = self.peek().ok_or_else(|| "Unexpected end of input while parsing operand".to_string())?;

        match token.kind {
            TokenKind::Register(reg) => {
                self.advance();
                Ok(Operand::Register(reg))
            }
            TokenKind::Integer(val) | TokenKind::HexInteger(val) => {
                self.advance();
                Ok(Operand::Integer(val))
            }
            TokenKind::Instruction(label) => {
                self.advance();
                Ok(Operand::LabelRef(label))
            }
            _ => Err(format!(
                "Expected register, literal integer, or label reference, found '{:?}' at line {}, col {}",
                token.kind, token.loc.line, token.loc.column
            )),
        }
    }


    fn peek(&self) -> Option<&Token<'a>> {
        self.tokens.get(self.cursor)
    }

    fn peek_kind(&self) -> Option<&TokenKind<'a>> {
        self.peek().map(|t| &t.kind)
    }

    fn advance(&mut self) -> Option<&Token<'a>> {
        if !self.is_at_end() {
            self.cursor += 1;
        }
        self.tokens.get(self.cursor - 1)
    }

    fn check(&self, kind: TokenKind<'a>) -> bool {
        if self.is_at_end() {
            return kind == TokenKind::EOF;
        }
        self.peek_kind() == Some(&kind)
    }

    fn match_token(&mut self, kind: TokenKind<'a>) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn consume(&mut self, kind: TokenKind<'a>, error_msg: &str) -> Result<&Token<'a>, String> {
        if self.check(kind) {
            Ok(self.advance().unwrap())
        } else {
            let loc = self.peek().map(|t| t.loc).unwrap_or(Location { line: 0, column: 0 });
            Err(format!("{} at line {}, col {}", error_msg, loc.line, loc.column))
        }
    }

    fn is_at_end(&self) -> bool {
        self.cursor >= self.tokens.len() || matches!(self.peek_kind(), Some(TokenKind::EOF))
    }
}