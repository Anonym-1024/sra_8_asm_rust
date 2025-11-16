use crate::sema::sema_error::SemaError;
use crate::parser::cst::{CstNode, CstNodeKind};



pub fn expand_labels(ast: CstNode) -> Result<(), SemaError> {
    assert_eq!(ast.kind, CstNodeKind::File);


    let mut stack: Vec<String> = Vec::new();

    let statements = &ast.child(0).children;

    for statement in statements {
        let statement_kind = statement.child(0).kind;
    }

    Ok(())
}

