use crate::{parser::cst::CstNode, sema::{ast::{assignment::{Assignment}, data_type::DataType, labels::LabelDefinition}}};

pub struct ResDirective {
    pub label: LabelDefinition,
    pub data_type: DataType,
    pub assignment: Option<Assignment>
}

impl ResDirective {
    pub fn from(node: &CstNode) -> Self {
        let label_definition_node = node.child(1);
        let label = LabelDefinition::from(label_definition_node);

        let data_type_node = node.child(2);
        let data_type = DataType::from(data_type_node);

        if node.children.len() == 2 {
            return ResDirective { label, data_type, assignment: None };
        }

        let assignment_node = node.child(3);
        let assignment = Assignment::from(assignment_node);

        ResDirective { label, data_type, assignment: Some(assignment) }
    }
}