use crate::assembler::lexer::Location;
use std::fmt;

#[derive(Debug, Clone)]
pub enum DiagnosticLevel {
    Error,
    Warning,
}

impl fmt::Display for DiagnosticLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiagnosticLevel::Error => write!(f, "\x1b[1;31merror\x1b[0m"),  
            DiagnosticLevel::Warning => write!(f, "\x1b[1;33mwarning\x1b[0m"), 
        }
    }
}

pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
    pub loc: Option<Location>,
    pub details: Option<String>,
}

impl Diagnostic {
    pub fn error(message: impl Into<String>, loc: Location) -> Self {
        Self {
            level: DiagnosticLevel::Error,
            message: message.into(),
            loc: Some(loc),
            details: None,
        }
    }

    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }

    pub fn report(&self, source_code: &str, file_name: &str) {
        println!("{}: {}", self.level, self.message);

        if let Some(loc) = self.loc {
            println!("  \x1b[1;34m-->\x1b[0m {}:{}:{}", file_name, loc.line, loc.column);
            println!("   \x1b[1;34m|\x1b[0m");

            let lines: Vec<&str> = source_code.lines().collect();
            if loc.line > 0 && loc.line <= lines.len() {
                let source_line = lines[loc.line - 1];

                println!("{:3} \x1b[1;34m|\x1b[0m {}", loc.line, source_line);

                let mut caret_line = String::new();
                for (i, c) in source_line.chars().enumerate() {
                    if i + 1 < loc.column {
                        if c == '\t' {
                            caret_line.push('\t');
                        } else {
                            caret_line.push(' ');
                        }
                    } else {
                        break;
                    }
                }
                
                println!("    \x1b[1;34m|\x1b[0m {}\x1b[1;31m^\x1b[0m", caret_line);
            }
        }

        if let Some(ref details) = self.details {
            println!("   \x1b[1;34m=\x1b[0m \x1b[1mhint:\x1b[0m {}", details);
        }
        println!();
    }
}