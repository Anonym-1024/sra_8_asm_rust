use crate::{lexer::token::TokenKind, parser::cst::{CstNode, CstNodeKind}, sema::ast::helpers::{num_lit_to_int, str_lit_to_str}};


#[derive(Debug)]
pub enum AssignmentValue {
    Number(i32),
    String(Vec<char>),
    Assignment(Assignment)
}
#[derive(Debug)]
pub struct Assignment {
    pub values: Vec<AssignmentValue>,
    pub repetition: u32
}

impl Assignment {
    pub fn from(node: &CstNode) -> Self {
        assert_eq!(node.kind, CstNodeKind::Assignment);

        let assignment_values_node = node.child(1);
        let values = make_values(assignment_values_node);

        if node.children.len() == 3 {
            return Self { values, repetition: 1 };
        }

        let repetition_node = node.child(3);
        let repetition = get_repetition(repetition_node);

        Self { values, repetition }
    }
}

fn make_values(node: &CstNode) -> Vec<AssignmentValue> {
    assert_eq!(node.kind, CstNodeKind::AssignmentValues);

    let mut result = Vec::new();
    for value_node in &node.children {
        result.push(get_value(value_node));
    }

    result

}

fn get_repetition(node: &CstNode) -> u32 {
    assert_eq!(node.kind, CstNodeKind::AssignmentRepetition);

    if node.children.len() == 1 {
        0
    } else {
        num_lit_to_int(node.child(1).terminal.as_ref().unwrap()) as u32
    }
}

fn get_value(node: &CstNode) -> AssignmentValue {
    assert_eq!(node.kind, CstNodeKind::AssignmentValue);

    if node.child(0).kind == CstNodeKind::Assignment {
        AssignmentValue::Assignment(Assignment::from(node.child(0)))
    } else if node.child(0).terminal.as_ref().unwrap().kind == TokenKind::Number {
        AssignmentValue::Number(num_lit_to_int(node.child(0).terminal.as_ref().unwrap()))
    } else {
        AssignmentValue::String(str_lit_to_str(node.child(0).terminal.as_ref().unwrap()))
    }
}