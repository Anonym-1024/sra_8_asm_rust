


use crate::{parser::cst::{CstNode, CstNodeKind}, sema::ast::{instruction_arg::InstructionArg, macro_arg::MacroArg}};




pub struct Macro {
    pub mnemonic: String,
    pub condition: Option<String>,
    pub args: Vec<MacroArg>,
    pub line: u32
}

impl Macro {
    pub fn from(node: &CstNode) -> Self {
        assert_eq!(node.kind, CstNodeKind::Macro);

        let line = node.child(0).terminal.as_ref().unwrap().line;

        let mnemonic = node.child(0).terminal.as_ref().unwrap().lexeme.clone();

        let mut args_i = 1;
        let mut condition_code = None;
        if node.children.len() == 3 {
            args_i = 2;
            let condition_node = node.child(1);
            condition_code = Some(condition_node.child(1).terminal.as_ref().unwrap().lexeme.clone());
        }

        let mut args = Vec::new();

        for arg_node in &node.child(args_i).children {
            args.push(MacroArg::from(arg_node));
        }


        Self { mnemonic, condition: condition_code, args, line }
    }
}