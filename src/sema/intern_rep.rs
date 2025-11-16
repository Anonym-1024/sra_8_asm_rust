use crate::{parser::ast::{AstNode, AstNodeKind}, sema::intern_rep::{export_directive::ExportDirective, file::File, import_directive::ImportDirective, instruction::Instruction, label_directive::LabelDirective, r#macro::Macro, res_directive::ResDirective, statement::Statement}};



pub mod statement;
pub mod res_directive;
pub mod export_directive;
pub mod import_directive;
pub mod instruction;
pub mod r#macro;
pub mod label_directive;
pub mod file;
pub mod data_type;
pub mod assignment;
pub mod labels;



pub fn build_intern_rep(root: &AstNode) -> File {
    File::from(root)
}