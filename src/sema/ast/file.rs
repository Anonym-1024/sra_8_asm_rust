use crate::{parser::cst::{CstNode, CstNodeKind}, sema::{ast::statement::Statement}};



pub struct File {
    pub statements: Vec<Statement>
}

impl File {
    pub fn from(node: &CstNode) -> Self {
        assert_eq!(node.kind, CstNodeKind::File);

        let mut statements: Vec<Statement> = Vec::new();

        let statements_node = node.child(0);
        let statement_nodes = &statements_node.children;


        for statement_node in statement_nodes {
            statements.push(Statement::from(statement_node));
        }

        File { statements }
    }
}