use crate::parser::ast::{AstNode, AstNodeKind};



pub struct LabelDefinition {
    prefix_count: u32,
    label: String,

    str: Option<String>
}

impl LabelDefinition {
    pub fn from(node: &AstNode) -> Self {
        assert_eq!(node.kind, AstNodeKind::LabelDefinition);

        let auto_scope_prefix_node = node.child(0);
        let prefix_count: u32 = auto_scope_prefix_node.children.len() as u32;

        let identifier_token = node.child(1).terminal.as_ref().unwrap();
        let label = identifier_token.lexeme.clone();

        Self { prefix_count, label, str: None }
    }
}

pub struct LabelAccess {
    prefix_count: u32,
    scopes: Vec<String>,
    label: String,

    str: Option<String>
}

impl LabelAccess {
    pub fn from(node: &AstNode) -> Self {
        assert_eq!(node.kind, AstNodeKind::LabelAccess);

        let auto_scope_prefix_node = node.child(0);
        let prefix_count: u32 = auto_scope_prefix_node.children.len() as u32;

        let scopes_node = node.child(1);

        let mut scopes = Vec::new();
        for scope_node in &scopes_node.children {
            scopes.push(scope_node.child(0).terminal.as_ref().unwrap().lexeme.clone());
        }

        let identifier_token = node.child(2).terminal.as_ref().unwrap();
        let label = identifier_token.lexeme.clone();

        Self { prefix_count, scopes, label, str: None }
    }
}

pub struct LabelExternal {
    scopes: Vec<String>,
    label: String,

    str: Option<String>
}

impl LabelExternal {
    pub fn from(node: &AstNode) -> Self {
        assert_eq!(node.kind, AstNodeKind::LabelExternal);

        let scopes_node = node.child(0);

        let mut scopes = Vec::new();
        for scope_node in &scopes_node.children {
            scopes.push(scope_node.child(0).terminal.as_ref().unwrap().lexeme.clone());
        }

        let identifier_token = node.child(1).terminal.as_ref().unwrap();
        let label = identifier_token.lexeme.clone();

        Self { scopes, label, str: None }
    }
}