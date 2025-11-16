

pub struct SemaError {
    desc: String,
    line: u32,
}


impl SemaError {
    pub fn desc(&self) -> String {
        format!("*** SEMA ERROR [LINE {}]: {}", self.line, self.desc)   
       
    }

    pub fn new(desc: &str, line: u32) -> Self {
        SemaError { desc: desc.to_string(), line }
    }
}