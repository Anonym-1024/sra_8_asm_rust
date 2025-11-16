use core::panic;

use crate::lexer::token::{self, Token, TokenKind};
use cst::{CstNode, CstNodeKind};
use parser_error::ParserError;
use result::ParserResult;


pub mod cst;
pub mod parser_error;
pub mod result;



pub struct Parser<'a> {
    tokens: &'a [Token],
    index: usize,
    line: u32
}

impl Parser<'_> {
    pub fn parse(tokens: &[Token]) -> ParserResult<CstNode, ParserError> {
        let mut parser = Parser { tokens: tokens, index: 0, line: 1 };

        parser.parse_file()
    }
}


impl<'a> Parser<'a> {
    fn pop_token_if_kind(&mut self, kind: TokenKind) -> Option<Token> {
        if self.index >= self.tokens.len() {
            return None;
        }
        let token = &self.tokens[self.index];


        if token.kind == kind {
            self.line = token.line;
            self.index += 1;
            Some(Token::new(kind, String::from(&token.lexeme), self.line))
        } else {
            None
        }
    }

    fn pop_token_if_lexeme(&mut self, lexeme: &str) -> Option<Token> {
        if self.index >= self.tokens.len() {
            return None;
        }
        let token = &self.tokens[self.index];


        if token.lexeme == lexeme {
            self.line = token.line;
            self.index += 1;
            Some(Token { kind: token.kind, lexeme: String::from(lexeme), line: self.line })
        } else {
            None
        }
    }



    fn lookahead(&self, k: usize) -> Option<&'a Token> {
        if self.index + k < self.tokens.len() { Some(&self.tokens[self.index + k]) } else { None }
    }



    fn parse_file(&mut self) -> ParserResult<CstNode, ParserError> {
        let mut children: Vec<CstNode> = Vec::new();

        let statements_node = self.parse_statements();
        match statements_node {
            ParserResult::Some(node) => children.push(node),
            ParserResult::Err(error) => return ParserResult::Err(error),
            ParserResult::None => panic!("INVALID"),
        }


        if self.pop_token_if_kind(TokenKind::Eof).is_some() {
            ParserResult::Some(CstNode::nonterminal(CstNodeKind::File, children))
        } else {

            ParserResult::Err(ParserError::new("Expected end of file", self.line))
        }
    }


    fn parse_statements(&mut self) -> ParserResult<CstNode, ParserError> {
        let mut children: Vec<CstNode> = Vec::new();

        loop {
            let statement_node = self.parse_statement();

            match statement_node {
                ParserResult::Some(node) => { children.push(node); },
                ParserResult::None => { break; },
                ParserResult::Err(_) => { return statement_node; },
            }
        }

        ParserResult::Some(CstNode::nonterminal(CstNodeKind::Statements, children))
    }


    fn parse_statement(&mut self) -> ParserResult<CstNode, ParserError> {
        let mut children: Vec<CstNode> = Vec::new();

        while self.pop_token_if_lexeme("\n").is_some() { }


        let child_node = self.parse_res_directive()
            .or(|| self.parse_start_directive())
            .or(|| self.parse_import_directive())
            .or(|| self.parse_export_directive())
            .or(|| self.parse_label_directive())
            .or(|| self.parse_instruction())
            .or(|| self.parse_macro());

        let mut child_node_desc = String::new();
        
        match child_node {
            ParserResult::Some(node) => { 
                child_node_desc.push_str(node.kind_desc().as_str());
                children.push(node); 
            },
            ParserResult::None => { return child_node; },
            ParserResult::Err(_) => { return child_node; },
        }
        
        let new_line_token = self.pop_token_if_lexeme("\n");
        
        if new_line_token.is_none() && let Some(t) = self.lookahead(0) && t.kind != TokenKind::Eof {
            return ParserResult::Err(ParserError::new(format!("Expected a new line after a statement, not {:?} \"{}\"; {} unterminated.", t.kind, t.lexeme, child_node_desc).as_str(), self.line));
        }



        ParserResult::Some(CstNode::nonterminal(CstNodeKind::Statement, children))

    }



    fn parse_label_definition(&mut self) -> ParserResult<CstNode, ParserError> {
        let mut children: Vec<CstNode> = Vec::new();

        let mut auto_scope_prefix_count = 0;
        let auto_scope_prefix_node = self.parse_auto_scope_prefix();
        match auto_scope_prefix_node {
            ParserResult::Some(node) => { auto_scope_prefix_count = node.children.len(); children.push(node) },
            ParserResult::None => { panic!("NEVER") },
            ParserResult::Err(_) => { return auto_scope_prefix_node }
        }

        if let Some(token) = self.pop_token_if_kind(TokenKind::Identifier) {
            children.push(CstNode::terminal(token));
        } else {
            if auto_scope_prefix_count == 0{
                return ParserResult::None;
            }
            return ParserResult::Err(ParserError::new("Expected identifier after auto scope prefix (>).", self.line))
        }

        ParserResult::Some(CstNode::nonterminal(CstNodeKind::LabelDefinition, children))
    }    


    fn parse_auto_scope_prefix(&mut self) -> ParserResult<CstNode, ParserError> {
        let mut children: Vec<CstNode> = Vec::new();

        while let Some(token) = self.pop_token_if_lexeme(">") {
            children.push(CstNode::terminal(token));
        }

        ParserResult::Some(CstNode::nonterminal(CstNodeKind::AutoScopePrefix, children))
    }


    fn parse_label_access(&mut self) -> ParserResult<CstNode, ParserError> {
        let mut children: Vec<CstNode> = Vec::new();

        if let Some(token) = self.pop_token_if_lexeme("$") {
            children.push(CstNode::terminal(token));
        } else {
            return ParserResult::None;
        }

        let auto_scope_prefix_node = self.parse_auto_scope_prefix();
        match auto_scope_prefix_node {
            ParserResult::Some(node) => { children.push(node) },
            ParserResult::None => { panic!("NEVER") },
            ParserResult::Err(_) => { return auto_scope_prefix_node }
        }

        
        let scopes_node = self.parse_scopes();
        match scopes_node {
            ParserResult::Some(node) => { children.push(node) },
            ParserResult::None => { panic!("NEVER") },
            ParserResult::Err(_) => { return scopes_node }
        }


        if let Some(token) = self.pop_token_if_kind(TokenKind::Identifier) {
            children.push(CstNode::terminal(token));
        } else {
            return ParserResult::Err(ParserError::new("Expected identifier after $ (label access).", self.line))
        }


        ParserResult::Some(CstNode::nonterminal(CstNodeKind::LabelAccess, children))
    }



    fn parse_scopes(&mut self) -> ParserResult<CstNode, ParserError> {
        let mut children: Vec<CstNode> = Vec::new();


        loop {
            let scope_node = self.parse_scope();
            match scope_node {
                ParserResult::Some(node) => { children.push(node); },
                ParserResult::None => { break; },
                ParserResult::Err(_) => { return scope_node; },
            }
        }

        ParserResult::Some(CstNode::nonterminal(CstNodeKind::Scopes, children)) 

    }



    fn parse_scope(&mut self) -> ParserResult<CstNode, ParserError> {
        let mut children: Vec<CstNode> = Vec::new();


        if !(self.lookahead(1).is_some_and(|t| t.lexeme == ">")) {
            return ParserResult::None;
        }


        if let Some(token) = self.pop_token_if_kind(TokenKind::Identifier) {
            children.push(CstNode::terminal(token));
        } else {
            return ParserResult::Err(ParserError::new("Expected an identifier before >.", self.line))
        }

        if let Some(token) = self.pop_token_if_lexeme( ">") {
            children.push(CstNode::terminal(token));
        } else {
            return ParserResult::Err(ParserError::new("NEVER", self.line))
        }



        ParserResult::Some(CstNode::nonterminal(CstNodeKind::Scope, children)) 

    }



    fn parse_label_external(&mut self) -> ParserResult<CstNode, ParserError> {
        let mut children: Vec<CstNode> = Vec::new();

        if let Some(token) = self.pop_token_if_lexeme("(") {
            children.push(CstNode::terminal(token));
        } else {
            return ParserResult::None;
        }

        let scopes_node = self.parse_scopes();
        match scopes_node {
            ParserResult::Some(node) => { children.push(node) },
            ParserResult::None => { panic!("NEVER") },
            ParserResult::Err(_) => { return scopes_node }
        }


        if let Some(token) = self.pop_token_if_kind(TokenKind::Identifier) {
            children.push(CstNode::terminal(token));
        } else {
            return ParserResult::Err(ParserError::new("Expected identifier in an external label expression.", self.line))
        }

        if let Some(token) = self.pop_token_if_lexeme(")") {
            children.push(CstNode::terminal(token));
        } else {
            return ParserResult::Err(ParserError::new("Expected ) after an external label expression.", self.line))
        }


        ParserResult::Some(CstNode::nonterminal(CstNodeKind::LabelExternal, children))
    }



    fn parse_res_directive(&mut self) -> ParserResult<CstNode, ParserError> {
        let mut children: Vec<CstNode> = Vec::new();

        if let Some(token) = self.pop_token_if_lexeme(".res") {
            children.push(CstNode::terminal(token));
        } else {
            return ParserResult::None;
        }

        let label_definition_node = self.parse_label_definition();
        match label_definition_node {
            ParserResult::Some(node) => { children.push(node) },
            ParserResult::None => { return ParserResult::Err(ParserError::new("Expected label definition after .res directive", self.line)) },
            ParserResult::Err(_) => { return label_definition_node }
        }

        let type_directive_node = self.parse_type_directive();
        match type_directive_node {
            ParserResult::Some(node) => { children.push(node); },
            ParserResult::None => { return ParserResult::Err(ParserError::new("Expected type in .res directive.", self.line)); },
            ParserResult::Err(_) => { return type_directive_node; },
        }

        let assignment_node = self.parse_assignment();
        match assignment_node {
            ParserResult::Some(node) => { children.push(node); },
            ParserResult::None => {  },
            ParserResult::Err(_) => { return assignment_node; },
        }

        ParserResult::Some(CstNode::nonterminal(CstNodeKind::ResDirective, children))

        
    }


    fn parse_type_directive(&mut self) -> ParserResult<CstNode, ParserError> {
        let mut children: Vec<CstNode> = Vec::new();

        let child_node = self.parse_byte_directive()
            .or(|| self.parse_bytes_directive())
            .or(|| self.parse_arr_directive());


        match child_node {
            ParserResult::Some(node) => { children.push(node); },
            ParserResult::None => { return child_node; },
            ParserResult::Err(_) => { return child_node; },
        }

        ParserResult::Some(CstNode::nonterminal(CstNodeKind::TypeDirective, children))

    }


    fn parse_byte_directive(&mut self) -> ParserResult<CstNode, ParserError> {
        let mut children: Vec<CstNode> = Vec::new();


        if let Some(token) = self.pop_token_if_lexeme( ".byte") {
            children.push(CstNode::terminal(token));
        } else {
            return ParserResult::None;
        }

        ParserResult::Some(CstNode::nonterminal(CstNodeKind::ByteDirective, children))
    }


    fn parse_bytes_directive(&mut self) -> ParserResult<CstNode, ParserError> {
        let mut children: Vec<CstNode> = Vec::new();


        if let Some(token) = self.pop_token_if_lexeme( ".bytes") {
            children.push(CstNode::terminal(token));
        } else {
            return ParserResult::None;
        }

        if let Some(token) = self.pop_token_if_kind(TokenKind::Number) {
            children.push(CstNode::terminal(token));
        } else {
            return ParserResult::Err(ParserError::new("Expected a number (element size) after a .bytes directive", self.line));
        }

        ParserResult::Some(CstNode::nonterminal(CstNodeKind::BytesDirective, children))
    }



    fn parse_arr_directive(&mut self) -> ParserResult<CstNode, ParserError> {
        let mut children: Vec<CstNode> = Vec::new();

        if let Some(token) = self.pop_token_if_lexeme( ".arr") {
            children.push(CstNode::terminal(token));
        } else {
            return ParserResult::None;
        }

        if let Some(token) = self.pop_token_if_kind(TokenKind::Number) {
            children.push(CstNode::terminal(token));
        } else {
            return ParserResult::Err(ParserError::new("Expected a number (array size) after an .arr directive", self.line));
        }

        let type_directive_node = self.parse_type_directive();
        match type_directive_node {
            ParserResult::Some(node) => { children.push(node); },
            ParserResult::None => { return ParserResult::Err(ParserError::new("Expected type in .arr directive.", self.line)); },
            ParserResult::Err(_) => { return type_directive_node; },
        }

        ParserResult::Some(CstNode::nonterminal(CstNodeKind::ArrDirective, children)) 

    }



    fn parse_assignment(&mut self) -> ParserResult<CstNode, ParserError> {
        let mut children: Vec<CstNode> = Vec::new();

        if let Some(token) = self.pop_token_if_lexeme( "{") {
            children.push(CstNode::terminal(token));
        } else {
            return ParserResult::None;
        }

        let assignment_values_node = self.parse_assignment_values();
        match assignment_values_node {
            ParserResult::Some(node) => { children.push(node); },
            ParserResult::None => { panic!("NEVER") },
            ParserResult::Err(_) => { return assignment_values_node; },
        }

        if let Some(token) = self.pop_token_if_lexeme( "}") {
            children.push(CstNode::terminal(token));
        } else {
            return ParserResult::Err(ParserError::new("Assignment list must be terminated with }.", self.line));
        }


        let assignment_repetition_node = self.parse_assignment_repetition();
        match assignment_repetition_node {
            ParserResult::Some(node) => { children.push(node); },
            ParserResult::None => {  },
            ParserResult::Err(_) => { return assignment_repetition_node; },
        }


        ParserResult::Some(CstNode::nonterminal(CstNodeKind::Assignment, children)) 

    }



    fn parse_assignment_repetition(&mut self) -> ParserResult<CstNode, ParserError> {
        let mut children: Vec<CstNode> = Vec::new();

        if let Some(token) = self.pop_token_if_lexeme( "*") {
            children.push(CstNode::terminal(token));
        } else {
            return ParserResult::None;
        }

        if let Some(token) = self.pop_token_if_kind(TokenKind::Number) {
            children.push(CstNode::terminal(token));
        } 

        ParserResult::Some(CstNode::nonterminal(CstNodeKind::AssignmentRepetition, children)) 

    }


    fn parse_assignment_values(&mut self) -> ParserResult<CstNode, ParserError> {
        let mut children: Vec<CstNode> = Vec::new();


        loop {
            let assignment_value_node = self.parse_assignment_value();
            match assignment_value_node {
                ParserResult::Some(node) => { children.push(node); },
                ParserResult::None => { break; },
                ParserResult::Err(_) => { return assignment_value_node; },
            }
        }

        ParserResult::Some(CstNode::nonterminal(CstNodeKind::AssignmentValues, children)) 

    }


    fn parse_assignment_value(&mut self) -> ParserResult<CstNode, ParserError> {
        let mut children: Vec<CstNode> = Vec::new();


        let assignment_node = self.parse_assignment();
        match assignment_node {
            ParserResult::Some(node) => { children.push(node); },
            ParserResult::None => {  },
            ParserResult::Err(_) => { return assignment_node; },
        }
        
        if !children.is_empty() {
            return ParserResult::Some(CstNode::nonterminal(CstNodeKind::AssignmentValue, children))
        }

        if let Some(token) = self.pop_token_if_kind(TokenKind::Number) {
            children.push(CstNode::terminal(token));
        } else if let Some(token) = self.pop_token_if_kind(TokenKind::String) {
            children.push(CstNode::terminal(token));
        } else {
            return ParserResult::None
        }
        
        ParserResult::Some(CstNode::nonterminal(CstNodeKind::AssignmentValue, children)) 


    }


    fn parse_start_directive(&mut self) -> ParserResult<CstNode, ParserError> {
        let mut children: Vec<CstNode> = Vec::new();

        if let Some(token) = self.pop_token_if_lexeme( ".start") {
            children.push(CstNode::terminal(token));
        } else {
            return ParserResult::None;
        }

        if let Some(token) = self.pop_token_if_lexeme( ":") {
            children.push(CstNode::terminal(token));
        } else {
            return ParserResult::Err(ParserError::new("Expected : after a .start directive.", self.line))
        }

        ParserResult::Some(CstNode::nonterminal(CstNodeKind::StartDirective, children)) 

    }



    fn parse_import_directive(&mut self) -> ParserResult<CstNode, ParserError> {
        let mut children: Vec<CstNode> = Vec::new();

        if let Some(token) = self.pop_token_if_lexeme( ".import") {
            children.push(CstNode::terminal(token));
        } else {
            return ParserResult::None;
        }

        let label_definition_node = self.parse_label_definition();
        match label_definition_node {
            ParserResult::Some(node) => { children.push(node) },
            ParserResult::None => { return ParserResult::Err(ParserError::new("Expected label definition after .import directive", self.line)) },
            ParserResult::Err(_) => { return label_definition_node }
        }

        let label_external_node = self.parse_label_external();
        match label_external_node {
            ParserResult::Some(node) => { children.push(node) },
            ParserResult::None => { return ParserResult::Err(ParserError::new("Expected external label in .import directive", self.line)) },
            ParserResult::Err(_) => { return label_external_node }
        }

        ParserResult::Some(CstNode::nonterminal(CstNodeKind::ImportDirective, children)) 
    }



    fn parse_export_directive(&mut self) -> ParserResult<CstNode, ParserError> {
        let mut children: Vec<CstNode> = Vec::new();

        if let Some(token) = self.pop_token_if_lexeme( ".export") {
            children.push(CstNode::terminal(token));
        } else {
            return ParserResult::None;
        }


        let label_access_node = self.parse_label_access();
        match label_access_node {
            ParserResult::Some(node) => { children.push(node) },
            ParserResult::None => { return ParserResult::Err(ParserError::new("Expected label access after .export directive", self.line)) },
            ParserResult::Err(_) => { return label_access_node }
        }

        let label_external_node = self.parse_label_external();
        match label_external_node {
            ParserResult::Some(node) => { children.push(node) },
            ParserResult::None => { return ParserResult::Err(ParserError::new("Expected external label in .export directive", self.line)) },
            ParserResult::Err(_) => { return label_external_node }
        }


        ParserResult::Some(CstNode::nonterminal(CstNodeKind::ExportDirective, children)) 
    }



    fn parse_label_directive(&mut self) -> ParserResult<CstNode, ParserError> {
        let mut children: Vec<CstNode> = Vec::new();

        let label_definition_node = self.parse_label_definition();
        match label_definition_node {
            ParserResult::Some(node) => { children.push(node); },
            ParserResult::None => { return ParserResult::None },
            ParserResult::Err(_) => { return label_definition_node }
        }

        if let Some(token) = self.pop_token_if_lexeme( ":") {
            children.push(CstNode::terminal(token));
        } else {
            return ParserResult::Err(ParserError::new("Expected : after a label directive.", self.line))
        }

        ParserResult::Some(CstNode::nonterminal(CstNodeKind::LabelDirective, children)) 
    }









    fn parse_instruction(&mut self) -> ParserResult<CstNode, ParserError> {
        let mut children: Vec<CstNode> = Vec::new();

        if let Some(token) = self.pop_token_if_kind(TokenKind::Instruction) {
            children.push(CstNode::terminal(token));
        } else {
            return ParserResult::None;
        }


        let condition_code_node = self.parse_condition_code();
        match condition_code_node {
            ParserResult::Some(node) => { children.push(node); },
            ParserResult::None => {  },
            ParserResult::Err(_) => { return condition_code_node; },
        }


        let arguments_node = self.parse_instruction_arguments();
        match arguments_node {
            ParserResult::Some(node) => { children.push(node); },
            ParserResult::None => { panic!("NEVER") },
            ParserResult::Err(_) => { return arguments_node; },
        }


        ParserResult::Some(CstNode::nonterminal(CstNodeKind::Instruction, children)) 

        
    }


    fn parse_condition_code(&mut self) -> ParserResult<CstNode, ParserError> {
        let mut children: Vec<CstNode> = Vec::new();

        if let Some(token) = self.pop_token_if_lexeme( ":") {
            children.push(CstNode::terminal(token));
        } else {
            return ParserResult::None;
        }

        if let Some(token) = self.pop_token_if_kind(TokenKind::ConditionCode) {
            children.push(CstNode::terminal(token));
        } else {
            return ParserResult::Err(ParserError::new("Expected condition code after : in an instruction.", self.line));
        }

        ParserResult::Some(CstNode::nonterminal(CstNodeKind::ConditionCode, children)) 

    }


    fn parse_instruction_arguments(&mut self) -> ParserResult<CstNode, ParserError> {
        let mut children: Vec<CstNode> = Vec::new();


        loop {
            let instruction_argument_node = self.parse_instruction_argument();
            match instruction_argument_node {
                ParserResult::Some(node) => { children.push(node); },
                ParserResult::None => { break; },
                ParserResult::Err(_) => { return instruction_argument_node; },
            }
        }

        ParserResult::Some(CstNode::nonterminal(CstNodeKind::InstructionArguments, children)) 

    }


    fn parse_instruction_argument(&mut self) -> ParserResult<CstNode, ParserError> {
        let mut children: Vec<CstNode> = Vec::new();

        if !children.is_empty() {
            return ParserResult::Some(CstNode::nonterminal(CstNodeKind::InstructionArgument, children)); 
        }

        if let Some(token) = self.pop_token_if_kind(TokenKind::Register) {
            children.push(CstNode::terminal(token));
        } else if let Some(token) = self.pop_token_if_kind(TokenKind::SystemRegister) {
            children.push(CstNode::terminal(token));
        } else if let Some(token) = self.pop_token_if_kind(TokenKind::Port) {
            children.push(CstNode::terminal(token));
        } else if let Some(token) = self.pop_token_if_kind(TokenKind::Number) {
            children.push(CstNode::terminal(token));
        } else if let Some(token) = self.pop_token_if_kind(TokenKind::String) {
            children.push(CstNode::terminal(token));
        } else if let Some(token) = self.pop_token_if_kind(TokenKind::LongRegister) {
            children.push(CstNode::terminal(token));
        } else {
            return ParserResult::None;
        }

        ParserResult::Some(CstNode::nonterminal(CstNodeKind::InstructionArgument, children))

    }




    fn parse_macro(&mut self) -> ParserResult<CstNode, ParserError> {
        let mut children: Vec<CstNode> = Vec::new();


        if let Some(token) = self.pop_token_if_kind(TokenKind::Macro) {
            children.push(CstNode::terminal(token));
        } else {
            return ParserResult::None;
        }


        let condition_code_node = self.parse_condition_code();
        match condition_code_node {
            ParserResult::Some(node) => { children.push(node); },
            ParserResult::None => {  },
            ParserResult::Err(_) => { return condition_code_node; },
        }


        let arguments_node = self.parse_macro_arguments();
        match arguments_node {
            ParserResult::Some(node) => { children.push(node); },
            ParserResult::None => { panic!("NEVER") },
            ParserResult::Err(_) => { return arguments_node; },
        }


        ParserResult::Some(CstNode::nonterminal(CstNodeKind::Macro, children)) 

    }


    fn parse_macro_arguments(&mut self) -> ParserResult<CstNode, ParserError> {
        let mut children: Vec<CstNode> = Vec::new();


        loop {
            let macro_argument_node = self.parse_macro_argument();
            match macro_argument_node {
                ParserResult::Some(node) => { children.push(node); },
                ParserResult::None => { break; },
                ParserResult::Err(_) => { return macro_argument_node; },
            }
        }

        ParserResult::Some(CstNode::nonterminal(CstNodeKind::MacroArguments, children)) 

    }


    fn parse_macro_argument(&mut self) -> ParserResult<CstNode, ParserError> {
        let mut children: Vec<CstNode> = Vec::new();
        

        let label_access_node = self.parse_label_access();
        match label_access_node {
            ParserResult::Some(node) => { children.push(node) },
            ParserResult::None => { },
            ParserResult::Err(_) => { return label_access_node }
        }

        if children.len() != 0 {
            return ParserResult::Some(CstNode::nonterminal(CstNodeKind::MacroArgument, children))
        }

        if let Some(token) = self.pop_token_if_kind(TokenKind::Register) {
            children.push(CstNode::terminal(token));
        } else if let Some(token) = self.pop_token_if_kind(TokenKind::Number) {
            children.push(CstNode::terminal(token));
        } else if let Some(token) = self.pop_token_if_kind(TokenKind::LongRegister) {
            children.push(CstNode::terminal(token));
        } else {
            return ParserResult::None;
        }

        ParserResult::Some(CstNode::nonterminal(CstNodeKind::MacroArgument, children))
    }


}







