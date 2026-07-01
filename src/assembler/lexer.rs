use std::str::CharIndices;
use crate::error::Result; 

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind<'a> {
    Instruction(&'a str), 
    Register(&'a str),    
    LabelDef(&'a str),    
    LabelRef(&'a str),    
    Integer(i64),         
    HexInteger(i64),      
    StringLiteral(&'a str),
    Comma,                
    NewLine,              
    EOF,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub lexeme: &'a str,
    pub loc: Location,
}

pub struct Lexer<'a> {
    source: &'a str,
    chars: CharIndices<'a>,
    line: usize,
    col_offset: usize,
    current_idx: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.char_indices(),
            line: 1,
            col_offset: 0,
            current_idx: 0,
        }
    }

    fn current_location(&self) -> Location {
        Location {
            line: self.line,
            column: self.current_idx - self.col_offset + 1,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.clone().next().map(|(_, c)| c)
    }

    fn advance(&mut self) -> Option<char> {
        if let Some((idx, c)) = self.chars.next() {
            self.current_idx = idx + c.len_utf8();
            if c == '\n' {
                self.line += 1;
                self.col_offset = self.current_idx;
            }
            Some(c)
        } else {
            self.current_idx = self.source.len();
            None
        }
    }

    pub fn next_token(&mut self) -> std::result::Result<Token<'a>, String> {
        self.skip_whitespace_and_comments();

        let start_idx = self.current_idx;
        let loc = self.current_location();

        let c = match self.advance() {
            None => {
                return Ok(Token {
                    kind: TokenKind::EOF,
                    lexeme: "",
                    loc,
                })
            }
            Some(c) => c,
        };

        if c == ',' {
            return Ok(Token { kind: TokenKind::Comma, lexeme: &self.source[start_idx..self.current_idx], loc });
        }
        if c == '\n' {
            return Ok(Token { kind: TokenKind::NewLine, lexeme: "\n", loc });
        }

        if c == '"' {
            while let Some(peeked) = self.peek() {
                if peeked == '"' {
                    self.advance();
                    let lexeme = &self.source[start_idx..self.current_idx];
                    let inner = &self.source[start_idx + 1..self.current_idx - 1];
                    return Ok(Token { kind: TokenKind::StringLiteral(inner), lexeme, loc });
                }
                if peeked == '\n' {
                    return Err(format!("Unterminated string literal at line {}, col {}", loc.line, loc.column));
                }
                self.advance();
            }
            return Err(format!("Unterminated string literal at end of file"));
        }

        if c.is_ascii_digit() || (c == '-' && self.peek().map_or(false, |p| p.is_ascii_digit())) {
            let is_hex = c == '0' && self.peek() == Some('x');
            if is_hex {
                self.advance(); // consume 'x'
                while self.peek().map_or(false, |p| p.is_ascii_hexdigit()) {
                    self.advance();
                }
                let lexeme = &self.source[start_idx..self.current_idx];
                let val = i64::from_str_radix(&lexeme[2..], 16)
                    .map_err(|_| format!("Invalid hex literal '{}' at line {}, col {}", lexeme, loc.line, loc.column))?;
                return Ok(Token { kind: TokenKind::HexInteger(val), lexeme, loc });
            } else {
                while self.peek().map_or(false, |p| p.is_ascii_digit()) {
                    self.advance();
                }
                let lexeme = &self.source[start_idx..self.current_idx];
                let val = lexeme.parse::<i64>()
                    .map_err(|_| format!("Invalid integer literal '{}' at line {}, col {}", lexeme, loc.line, loc.column))?;
                return Ok(Token { kind: TokenKind::Integer(val), lexeme, loc });
            }
        }

        if c.is_ascii_alphabetic() || c == '_' || c == '.' {
            while self.peek().map_or(false, |p| p.is_ascii_alphanumeric() || p == '_') {
                self.advance();
            }

            let mut lexeme = &self.source[start_idx..self.current_idx];

            if self.peek() == Some(':') {
                self.advance(); 
                lexeme = &self.source[start_idx..self.current_idx];
                let label_name = &lexeme[..lexeme.len() - 1];
                return Ok(Token { kind: TokenKind::LabelDef(label_name), lexeme, loc });
            }

            if lexeme.starts_with('r') && lexeme[1..].chars().all(|rc| rc.is_ascii_digit()) || lexeme == "sp" || lexeme == "pc" {
                return Ok(Token { kind: TokenKind::Register(lexeme), lexeme, loc });
            }

            return Ok(Token { kind: TokenKind::Instruction(lexeme), lexeme, loc });
        }

        Err(format!("Unexpected character '{}' at line {}, col {}", c, loc.line, loc.column))
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.peek() {
                Some(' ') | Some('\t') | Some('\r') => {
                    self.advance();
                }
                Some(';') => { 
                    while self.peek().map_or(false, |p| p != '\n') {
                        self.advance();
                    }
                }
                _ => break,
            }
        }
    }
}