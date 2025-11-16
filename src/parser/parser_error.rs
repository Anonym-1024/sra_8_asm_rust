



pub struct ParserError {
    desc: String,
    line: u32,
}


impl ParserError {
    pub fn desc(&self) -> String {
        format!("*** PARSER ERROR [LINE {}]: {}", self.line, self.desc)   
       
    }

    pub fn new(desc: &str, line: u32) -> Self {
        ParserError { desc: desc.to_string(), line }
    }
}


