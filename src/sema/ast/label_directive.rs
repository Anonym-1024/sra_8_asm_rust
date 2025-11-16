use crate::{parser::cst::{CstNode, CstNodeKind}, sema::ast::labels::LabelDefinition};


pub struct LabelDirective {
    label: LabelDefinition
}

impl LabelDirective {
    pub fn from(node: &CstNode) -> Self {
        assert_eq!(node.kind, CstNodeKind::LabelDirective);

        let label_definition_node = node.child(0);
        let label_definition = LabelDefinition::from(label_definition_node);

        Self { label: label_definition }
    }
}