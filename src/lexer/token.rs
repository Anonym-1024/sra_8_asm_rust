
#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(Clone, Copy)]
pub enum TokenKind {
    Instruction,
    Macro,
    Directive,
    Register,
    LongRegister,
    Port,
    SystemRegister,
    ConditionCode,
    Identifier,
    Number,
    String,
    Punctuation,
    Eof
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: u32,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, line: u32) -> Self {
        Self  {kind: kind, lexeme: lexeme, line: line}
    }

    pub fn eof_token(line: u32) -> Self {
        Self { kind: TokenKind::Eof, lexeme: String::new(), line }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line: {}, type: {:?}, lexeme: {}", self.line, self.kind, self.lexeme)
    }
}


