use crate::{lexer::token::TokenKind, parser::cst::{CstNode, CstNodeKind}, sema::ast::{helpers::{num_lit_to_int, str_lit_to_str}, labels::LabelAccess}};


#[derive(Debug)]
pub enum MacroArg {
    Register(String),
    Number(i32),
    LongRegister(String),
    Label(LabelAccess)
}

impl MacroArg {
    pub fn from(node: &CstNode) -> Self {
        assert_eq!(node.kind, CstNodeKind::MacroArgument);

        
        if node.child(0).kind == CstNodeKind::LabelAccess {
            return Self::Label(LabelAccess::from(node.child(0)));
        }

        let token = node.child(0).terminal.as_ref().unwrap();

        match token.kind {
            TokenKind::Register => Self::Register(token.lexeme.clone()),
            TokenKind::Number => Self::Number(num_lit_to_int(token)),
            TokenKind::LongRegister => Self::LongRegister(token.lexeme.clone()),
            _ => unreachable!()
        }
    }
}