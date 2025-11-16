use crate::{parser::ast::AstNode, sema::intern_rep::{data_type::DataType, labels::LabelDefinition}};

pub struct ResDirective {
    label: LabelDefinition,
    data_type: DataType,
}

impl ResDirective {
    pub fn from(node: AstNode) -> Self {

    }
}