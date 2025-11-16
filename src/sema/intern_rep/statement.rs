use crate::sema::intern_rep::{export_directive::ExportDirective, import_directive::ImportDirective, instruction::Instruction, label_directive::{LabelDirective}, r#macro::Macro, res_directive::ResDirective};


pub enum Statement {
    ImportDirective(ImportDirective),
    ExportDirective(ExportDirective),
    ResDirective(ResDirective),
    LabelDirective(LabelDirective),
    Instruction(Instruction),
    Macro(Macro)
}