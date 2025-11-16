
use crate::lexer::token;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone, Copy)]
pub enum CstNodeKind {
    Terminal,
    File,
    Statements,
    Statement,
    ResDirective,
    ImportDirective,
    ExportDirective,
    TypeDirective,
    ByteDirective,
    BytesDirective,
    ArrDirective,
    Assignment,
    AssignmentRepetition,
    AssignmentValues,
    AssignmentValue,
    StartDirective,
    LabelDirective,
    LabelDefinition,
    AutoScopePrefix,
    LabelAccess,
    LabelExternal, 
    Scopes,
    Scope,
    Instruction,
    ConditionCode,
    InstructionArguments,
    InstructionArgument,
    Macro,
    MacroArguments,
    MacroArgument,
}
#[derive(Debug)]
pub struct CstNode {
    pub kind: CstNodeKind,
    pub children: Vec<CstNode>,
    pub terminal: Option<token::Token>
}

impl CstNode {
    pub fn nonterminal(kind: CstNodeKind, children: Vec<CstNode>) -> CstNode {
        CstNode { kind, children, terminal: None }
    }

    pub fn terminal(terminal: token::Token) -> CstNode {
        CstNode { kind: CstNodeKind::Terminal, children: Vec::new(), terminal: Some(terminal) }
    }

    pub fn kind_desc(&self) -> String {
        match self.kind {
            CstNodeKind::Instruction => "instruction statement".to_string(),
            CstNodeKind::ResDirective => "reserve directive".to_string(),
            CstNodeKind::ImportDirective => "import directive".to_string(),
            CstNodeKind::ExportDirective => "export directive".to_string(),
            CstNodeKind::StartDirective => "start directive".to_string(),
            CstNodeKind::LabelDirective => "lebel directive".to_string(),
            CstNodeKind::Macro => "macro directive".to_string(),
            _ => "".to_string()
        }
    }

    pub fn child<'a>(&'a self, index: usize) -> &'a CstNode {
        
        &self.children[index]
    }

    pub fn child_mut<'a>(&'a mut self, index: usize) -> &'a mut CstNode {
        
        &mut self.children[index]
    }
}