use crate::{parser::ast::{AstNode, AstNodeKind}, sema::intern_rep::labels::{LabelAccess, LabelExternal}};


pub struct ExportDirective {
    label_intern: LabelAccess,
    label_extern: LabelExternal
}

impl ExportDirective {
    pub fn from(node: &AstNode) -> Self {
        assert_eq!(node.kind, AstNodeKind::ExportDirective);

        let label_access_node = node.child(1);
        let label_external_node = node.child(2);

        Self { label_intern: LabelAccess::from(label_access_node), label_extern: LabelExternal::from(label_external_node) }

    }
}