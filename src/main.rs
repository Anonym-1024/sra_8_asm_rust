use std::time::Instant;

use colorize::AnsiColor;

use crate::parser::result::ParserResult;



mod lexer;
mod parser;
mod sema;


fn main() {
    
    println!("");

    let start = Instant::now();
    
    let src = std::fs::read_to_string("resources/test.txt").expect("could not read");
    println!("{}", "*** Starting lexical analysis.".cyan());
    let a = lexer::tokenise(src);
    
    
    
    match &a {
        
        Err(x) => println!("{}\n", x.desc().red().bold()),
        Ok(x) => {
            println!("{}\n", "*** Lexer success.".green().bold());
            for _i in x {
                //println!("{i}");
            }
            println!("{}", "*** Starting syntactic analysis.".b_cyan());
            let p = parser::parse(x);

            match &p {
                ParserResult::Err(err) => println!("{}", err.desc().red().bold()),
                ParserResult::Some(s) => {println!("{}\n", "*** Syntactic analysis success.".green().bold()); std::fs::write("resources/res.txt", format!("{:#?}", s).as_bytes());},
                ParserResult::None => {panic!("")}
            }
        }
    }


    
    let end = start.elapsed().as_millis();

    println!("Time: {}", end);
}


