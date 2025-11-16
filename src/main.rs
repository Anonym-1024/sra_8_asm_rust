use std::time::Instant;

use colorize::AnsiColor;

use crate::{parser::{cst::CstNode, result::ParserResult}, sema::{ast::{file::File, statement::Statement}, label_expander::expand_labels}};



mod lexer;
mod parser;
mod sema;


fn main() {
    
    println!("");

    let start = Instant::now();
    
    let src = std::fs::read_to_string("resources/test.txt").expect("could not read");
    println!("{}", "*** Starting lexical analysis.".cyan());
    let a = lexer::Lexer::tokenise(&src);
    
    
    
    match &a {
        
        Err(x) => println!("{}\n", x.desc().red().bold()),
        Ok(x) => {
            println!("{}\n", "*** Lexer success.".green().bold());
            for _i in x {
                //println!("{i}");
            }
            println!("{}", "*** Starting syntactic analysis.".b_cyan());
            let p = parser::Parser::parse(x);

            match &p {
                ParserResult::Err(err) => println!("{}", err.desc().red().bold()),
                ParserResult::Some(s) => {println!("{}\n", "*** Syntactic analysis success.".green().bold()); std::fs::write("resources/res.txt", format!("{:#?}", s).as_bytes()); 
            test(s);
            },
                ParserResult::None => {panic!("")}
            }
        }
    }


    
    let end = start.elapsed().as_millis();

    println!("Time: {}", end);
}


fn test(s: &CstNode) {
    let mut file = File::from(s);

    match expand_labels(&mut file) {
            Ok(_) => println!("OK"),
            Err(e) => println!("{}", e.desc())
        }

    for stmt in &file.statements {
        match stmt {
            Statement::ExportDirective(r) => {
                println!("\n{:#?} {:#?}\n", r.label_intern.str, r.label_extern.str);
            
            },
            Statement::ImportDirective(r) => {
                println!("\n{:#?} {:#?}\n", r.label_intern.str, r.label_extern.str);
            
            },
            Statement::ResDirective(r) => {
                println!("\n{:#?}\n", r.label.str);
            
            },

            Statement::LabelDirective(r) => {
                println!("\n{:#?}\n", r.label.str);
            
            },
            Statement::Macro(r) => {
                println!("\n{:#?}\n", r.args[0]);
            
            },
            _ => {}
        }
    }
}