use crate::{lexer::token::TokenKind, parser::cst::{CstNode, CstNodeKind}, sema::ast::helpers::{num_lit_to_int, str_lit_to_str}};


#[derive(Debug)]
pub enum InstructionArg {
    Register(String),
    SystemRegister(String),
    Port(String),
    Number(i32),
    String(Vec<char>),
    LongRegister(String)
}

impl InstructionArg {
    pub fn from(node: &CstNode) -> Self {
        assert_eq!(node.kind, CstNodeKind::InstructionArgument);

        let token = node.child(0).terminal.as_ref().unwrap();

        match token.kind {
            TokenKind::Register => Self::Register(token.lexeme.clone()),
            TokenKind::SystemRegister => Self::SystemRegister(token.lexeme.clone()),
            TokenKind::Port => Self::Port(token.lexeme.clone()),
            TokenKind::Number => Self::Number(num_lit_to_int(token)),
            TokenKind::String => Self::String(str_lit_to_str(token)),
            TokenKind::LongRegister => Self::LongRegister(token.lexeme.clone()),
            _ => unreachable!()
        }
    }
}