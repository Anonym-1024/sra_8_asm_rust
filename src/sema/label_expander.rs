use crate::sema::sema_error::SemaError;
use crate::parser::ast::{AstNode, AstNodeKind};



pub fn expand_labels(ast: AstNode) -> Result<(), SemaError> {
    assert_eq!(ast.kind, AstNodeKind::File);


    let mut stack: Vec<String> = Vec::new();

    let statements = &ast.child(0).children;

    for statement in statements {
        let statement_kind = statement.child(0).kind;
    }

    Ok(())
}

