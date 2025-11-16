use crate::{parser::cst::{CstNode, CstNodeKind}, sema::ast::labels::{LabelDefinition, LabelExternal}};


pub struct ImportDirective {
    pub label_intern: LabelDefinition,
    pub label_extern: LabelExternal,
    pub line: u32
}

impl ImportDirective {
    pub fn from(node: &CstNode) -> Self {
        assert_eq!(node.kind, CstNodeKind::ImportDirective);

        let line = node.child(0).terminal.as_ref().unwrap().line;

        let label_definition_node = node.child(1);
        let label_external_node = node.child(2);

        Self { label_intern: LabelDefinition::from(label_definition_node), label_extern: LabelExternal::from(label_external_node), line }

    }
}