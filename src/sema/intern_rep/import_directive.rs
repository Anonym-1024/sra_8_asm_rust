use crate::{parser::ast::{AstNode, AstNodeKind}, sema::intern_rep::labels::{LabelDefinition, LabelExternal}};


pub struct ImportDirective {
    label_intern: LabelDefinition,
    label_extern: LabelExternal
}

impl ImportDirective {
    pub fn from(node: &AstNode) -> Self {
        assert_eq!(node.kind, AstNodeKind::ExportDirective);

        let label_definition_node = node.child(1);
        let label_external_node = node.child(2);

        Self { label_intern: LabelDefinition::from(label_definition_node), label_extern: LabelExternal::from(label_external_node) }

    }
}