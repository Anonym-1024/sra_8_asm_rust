use crate::{parser::cst::{CstNode, CstNodeKind}, sema::ast::{export_directive::ExportDirective, import_directive::ImportDirective, instruction::Instruction, label_directive::LabelDirective, r#macro::Macro, res_directive::ResDirective}};


pub enum Statement {
    ImportDirective(ImportDirective),
    ExportDirective(ExportDirective),
    ResDirective(ResDirective),
    LabelDirective(LabelDirective),
    Instruction(Instruction),
    Macro(Macro)
}

impl Statement {
    pub fn from(node: &CstNode) -> Self {
        assert_eq!(node.kind, CstNodeKind::Statement);

        match node.child(0).kind {
            CstNodeKind::ImportDirective => Statement::ImportDirective(ImportDirective::from(node.child(0))),
            CstNodeKind::ExportDirective => Statement::ExportDirective(ExportDirective::from(node.child(0))),
            CstNodeKind::ResDirective => Statement::ResDirective(ResDirective::from(node.child(0))),
            CstNodeKind::LabelDirective => Statement::LabelDirective(LabelDirective::from(node.child(0))),
            CstNodeKind::Instruction => Statement::Instruction(Instruction::from(node.child(0))),
            CstNodeKind::Macro => Statement::Macro(Macro::from(node.child(0))),
            _ => unreachable!()
        }
        
    }
}