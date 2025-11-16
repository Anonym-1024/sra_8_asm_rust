use crate::parser::cst::{CstNode, CstNodeKind};


#[derive(Debug)]
pub struct LabelDefinition {
    pub prefix_count: u32,
    pub label: String,

    pub str: Option<String>
}

impl LabelDefinition {
    pub fn from(node: &CstNode) -> Self {
        assert_eq!(node.kind, CstNodeKind::LabelDefinition);

        let auto_scope_prefix_node = node.child(0);
        let prefix_count: u32 = auto_scope_prefix_node.children.len() as u32;

        let identifier_token = node.child(1).terminal.as_ref().unwrap();
        let label = identifier_token.lexeme.clone();

        Self { prefix_count, label, str: None }
    }
}
#[derive(Debug)]
pub struct LabelAccess {
    pub prefix_count: u32,
    pub scopes: Vec<String>,
    pub label: String,

    pub str: Option<String>
}

impl LabelAccess {
    pub fn from(node: &CstNode) -> Self {
        assert_eq!(node.kind, CstNodeKind::LabelAccess);

        let auto_scope_prefix_node = node.child(1);
        let prefix_count: u32 = auto_scope_prefix_node.children.len() as u32;

        let scopes_node = node.child(2);

        let mut scopes = Vec::new();
        for scope_node in &scopes_node.children {
            scopes.push(scope_node.child(0).terminal.as_ref().unwrap().lexeme.clone());
        }

        let identifier_token = node.child(3).terminal.as_ref().unwrap();
        let label = identifier_token.lexeme.clone();

        Self { prefix_count, scopes, label, str: None }
    }
}
#[derive(Debug)]
pub struct LabelExternal {
    pub scopes: Vec<String>,
    pub label: String,

    pub str: Option<String>
}

impl LabelExternal {
    pub fn from(node: &CstNode) -> Self {
        assert_eq!(node.kind, CstNodeKind::LabelExternal);

        let scopes_node = node.child(1);

        let mut scopes = Vec::new();
        for scope_node in &scopes_node.children {
            scopes.push(scope_node.child(0).terminal.as_ref().unwrap().lexeme.clone());
        }

        let identifier_token = node.child(2).terminal.as_ref().unwrap();
        let label = identifier_token.lexeme.clone();

        Self { scopes, label, str: None }
    }
}