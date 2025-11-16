use crate::{parser::cst::{CstNode, CstNodeKind}, sema::ast::labels::{LabelAccess, LabelExternal}};


pub struct ExportDirective {
    pub label_intern: LabelAccess,
    pub label_extern: LabelExternal
}

impl ExportDirective {
    pub fn from(node: &CstNode) -> Self {
        assert_eq!(node.kind, CstNodeKind::ExportDirective);

        let label_access_node = node.child(1);
        let label_external_node = node.child(2);

        Self { label_intern: LabelAccess::from(label_access_node), label_extern: LabelExternal::from(label_external_node) }

    }
}