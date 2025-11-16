use crate::sema::ast::export_directive::ExportDirective;
use crate::sema::ast::file::File;
use crate::sema::ast::import_directive::ImportDirective;
use crate::sema::ast::label_directive::LabelDirective;
use crate::sema::ast::r#macro::Macro;
use crate::sema::ast::macro_arg::MacroArg;
use crate::sema::ast::res_directive::ResDirective;
use crate::sema::ast::statement::Statement;
use crate::sema::sema_error::SemaError;
use crate::parser::cst::{CstNode, CstNodeKind};



pub fn expand_labels(file: &mut File) -> Result<(), SemaError> {
    
    let mut stack: Vec<String> = Vec::new();


    for stmt in &mut file.statements {
        match stmt {
            Statement::ExportDirective(node) => expand_export_directive(node, &mut stack)?,
            Statement::ImportDirective(node) => expand_import_directive(node, &mut stack)?,
            Statement::ResDirective(node) => expand_res_directive(node, &mut stack)?,
            Statement::LabelDirective(node) => expand_label_directive(node, &mut stack)?,
            Statement::Instruction(node) => {},
            Statement::Macro(node) => expand_macro(node, &mut stack)?,
        }
    }

    Ok(())
}

fn expand_export_directive(node: &mut ExportDirective, stack: &mut Vec<String>) -> Result<(), SemaError>{
    let label_intern = &node.label_intern;

    let mut str = String::new();

    if label_intern.prefix_count as usize > stack.len() {
        return Err(SemaError::new("Auto nesting too deep.", node.line))
    }

    for i in 0..label_intern.prefix_count as usize {
        str.push_str(&stack[i]);
        str.push('>');
    }

    for scope in &label_intern.scopes {
        str.push_str(&scope);
        str.push('>');
    }

    str.push_str(&label_intern.label);

    node.label_intern.str = Some(str);



    let mut str = String::new();

    let label_extern = &node.label_extern;

    for scope in &label_extern.scopes {
        str.push_str(&scope);
        str.push('>');
    }

    str.push_str(&label_extern.label);

    node.label_extern.str = Some(str);




    Ok(())
}


fn expand_import_directive(node: &mut ImportDirective, stack: &mut Vec<String>) -> Result<(), SemaError>{
    let label_intern = &node.label_intern;

    let mut str = String::new();

    if label_intern.prefix_count as usize > stack.len() {
        return Err(SemaError::new("Auto nesting too deep.", node.line))
    }

    for i in 0..label_intern.prefix_count as usize {
        str.push_str(&stack[i]);
        str.push('>');
    }

    str.push_str(&label_intern.label);

    node.label_intern.str = Some(str);

    
    let mut str = String::new();

    let label_extern = &node.label_extern;

    for scope in &label_extern.scopes {
        str.push_str(&scope);
        str.push('>');
    }

    str.push_str(&label_extern.label);

    node.label_extern.str = Some(str);

    Ok(())
}


fn expand_res_directive(node: &mut ResDirective, stack: &mut Vec<String>) -> Result<(), SemaError> {

    let label = &node.label;

    let mut str = String::new();

    if label.prefix_count as usize > stack.len() {
        return Err(SemaError::new("Auto nesting too deep.", node.line))
    }

    for i in 0..label.prefix_count as usize {
        str.push_str(&stack[i]);
        str.push('>');
    }

    str.push_str(&label.label);

    node.label.str = Some(str);




    Ok(())
}


fn expand_label_directive(node: &mut LabelDirective, stack: &mut Vec<String>) -> Result<(), SemaError> {
    
    let label = &node.label;

    let mut str = String::new();

    if label.prefix_count as usize > stack.len() {
        return Err(SemaError::new("Auto nesting too deep.", node.line))
    }

    for i in 0..label.prefix_count as usize {
        str.push_str(&stack[i]);
        str.push('>');
    }

    str.push_str(&label.label);

    node.label.str = Some(str);


    
    for _ in 0..(stack.len() - node.label.prefix_count as usize) {
        stack.pop();
    }
    stack.push(node.label.label.clone());
    




    Ok(())

}


fn expand_macro(node: &mut Macro, stack: &mut Vec<String>) -> Result<(), SemaError> {
    for arg in &mut node.args {
        if let MacroArg::Label(label) = arg {

            let mut str = String::new();

            if label.prefix_count as usize > stack.len() {
                return Err(SemaError::new("Auto nesting too deep.", node.line))
            }

            for i in 0..label.prefix_count as usize {
                str.push_str(&stack[i]);
                str.push('>');
            }

            for scope in &label.scopes {
                str.push_str(&scope);
                str.push('>');
            }

            str.push_str(&label.label);

            label.str = Some(str);
        }
    }

    Ok(())

}