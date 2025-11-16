use crate::{parser::ast::{AstNode, AstNodeKind}, sema::intern_rep::{export_directive::ExportDirective, import_directive::ImportDirective, instruction::Instruction, label_directive::LabelDirective, r#macro::Macro, res_directive::ResDirective, statement::Statement}};



pub struct File {
    statements: Vec<Statement>
}

impl File {
    pub fn from(node: &AstNode) -> Self {
        assert_eq!(node.kind, AstNodeKind::File);

        let statements: Vec<Statement> = Vec::new();

        let statements_node = node.child(0);
        let statement_nodes = &statements_node.children;


        for statement_node in statement_nodes {
            match statement_node.kind {
                AstNodeKind::ImportDirective => statements.push(Statement::ImportDirective(ImportDirective::from(statement_node.child(0)))),
                AstNodeKind::ExportDirective => statements.push(Statement::ExportDirective(ExportDirective::from(statement_node.child(0)))),
                AstNodeKind::ResDirective => statements.push(Statement::ResDirective(ResDirective::from(statement_node.child(0)))),
                AstNodeKind::LabelDirective => statements.push(Statement::LabelDirective(LabelDirective::from(statement_node.child(0)))),
                AstNodeKind::Instruction => statements.push(Statement::Instruction(Instruction::from(statement_node.child(0)))),
                AstNodeKind::Macro => statements.push(Statement::Macro(Macro::from(statement_node.child(0)))),
                _ => unreachable!()
            }
        }

        File {statements }
    }
}