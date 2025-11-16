
use crate::lexer::token;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone, Copy)]
pub enum AstNodeKind {
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
pub struct AstNode {
    pub kind: AstNodeKind,
    pub children: Vec<AstNode>,
    pub terminal: Option<token::Token>
}

impl AstNode {
    pub fn nonterminal(kind: AstNodeKind, children: Vec<AstNode>) -> AstNode {
        AstNode { kind, children, terminal: None }
    }

    pub fn terminal(terminal: token::Token) -> AstNode {
        AstNode { kind: AstNodeKind::Terminal, children: Vec::new(), terminal: Some(terminal) }
    }

    pub fn kind_desc(&self) -> String {
        match self.kind {
            AstNodeKind::Instruction => "instruction statement".to_string(),
            AstNodeKind::ResDirective => "reserve directive".to_string(),
            AstNodeKind::ImportDirective => "import directive".to_string(),
            AstNodeKind::ExportDirective => "export directive".to_string(),
            AstNodeKind::StartDirective => "start directive".to_string(),
            AstNodeKind::LabelDirective => "lebel directive".to_string(),
            AstNodeKind::Macro => "macro directive".to_string(),
            _ => "".to_string()
        }
    }

    pub fn child<'a>(&'a self, index: usize) -> &'a AstNode {
        
        &self.children[index]
    }

    pub fn child_mut<'a>(&'a mut self, index: usize) -> &'a mut AstNode {
        
        &mut self.children[index]
    }
}