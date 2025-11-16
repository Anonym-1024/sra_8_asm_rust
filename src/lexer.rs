use token::{Token, TokenKind};
use lexer_error::LexerError;



pub mod token;
pub mod lexer_error;
pub mod resources;


pub struct Lexer<'a> {
    chars: &'a [char],
    index: usize,
    line: u32
}

impl Lexer<'_> {
    pub fn tokenise(src: &str) -> Result<Vec<Token>, LexerError> {
        let chars: Vec<char> = src.chars().collect();
        let mut lexer = Lexer { chars: &chars, index: 0, line: 1 };

        lexer._tokenise()
    }
}


impl<'a> Lexer<'a> {
    fn is_word_char(c: char) -> bool {
        c.is_ascii_alphanumeric() || (c == '_')
    }

    fn is_punctation_character(c: char) -> bool {
        (c == '{') || (c == '}') || (c == ':') || (c == '*') || (c == '>') || (c == '(') || (c == ')') || (c == '$')
    }


    fn drop_comment(&mut self) {
        let chars_c = self.chars.len();

        while self.index < chars_c && self.chars[self.index] != '\n' {
            
            self.index += 1;
        }
    }

    fn get_word_token_kind(lexeme: &str) -> TokenKind {
        if resources::INSTRUCTION_NAMES.contains(&lexeme) {
            TokenKind::Instruction
        } else if resources::REGISTER_NAMES.contains(&lexeme) {
            TokenKind::Register
        } else if resources::LONG_REGISTER_NAMES.contains(&lexeme) {
            TokenKind::LongRegister
        } else if resources::SYSTEM_REGISTER_NAMES.contains(&lexeme) {
            TokenKind::SystemRegister
        } else if resources::PORT_NAMES.contains(&lexeme) {
            TokenKind::Port
        } else if resources::CONDITION_CODE_NAMES.contains(&lexeme) {
            TokenKind::ConditionCode
        } else {
            TokenKind::Identifier
        }
    }


    fn is_macro_name(lexeme: &str) -> bool {
        resources::MACRO_NAMES.contains(&lexeme)
    }


    fn is_directive_name(lexeme: &str) -> bool {
        resources::DIRECTIVE_NAMES.contains(&lexeme)
    }


    fn make_word_token(&mut self) -> Token {
        let mut lexeme = String::new();
        

        let chars_c = self.chars.len();

        
        while self.index < chars_c && Self::is_word_char(self.chars[self.index]) {
            lexeme.push(self.chars[self.index]);
            self.index += 1;
        }


        let kind = Self::get_word_token_kind(&lexeme);
        Token::new(kind, lexeme, self.line)
    }


    fn make_macro_token(&mut self) -> Result<Token, LexerError> {
        let mut lexeme = String::new();
        

        let chars_c = self.chars.len();


        lexeme.push(self.chars[self.index]);
        self.index += 1;

        while self.index < chars_c && Self::is_word_char(self.chars[self.index]) {
            lexeme.push(self.chars[self.index]);
            self.index += 1;
        }

        if Self::is_macro_name(&lexeme) {
            Ok(Token::new(TokenKind::Macro, lexeme, self.line))
        } else {
            Err(LexerError::new(lexer_error::LexerErrorKind::InvalidMacro(lexeme), self.line))
        }
    }


    fn make_directive_token(&mut self) -> Result<Token, LexerError> {
        let mut lexeme = String::new();
        

        let chars_c = self.chars.len();


        lexeme.push(self.chars[self.index]);
        self.index += 1;

        while self.index < chars_c && Self::is_word_char(self.chars[self.index]) {
            lexeme.push(self.chars[self.index]);
            self.index += 1;
        }

        if Self::is_directive_name(&lexeme) {
            Ok(Token::new(TokenKind::Directive, lexeme, self.line))
        } else {
            Err(LexerError::new(lexer_error::LexerErrorKind::InvalidDirective(lexeme), self.line))
        }
    }


    fn is_radix_prefix(c: char) -> bool {
        ['b', 'o', 'd', 'x'].contains(&c)
    }

    fn is_valid_digit(radix: char, digit: char) -> bool {
        match radix {
            'b' => ['0', '1'].contains(&digit),
            'o' => ['0', '1', '2', '3', '4', '5', '6', '7'].contains(&digit),
            'd' => digit.is_ascii_digit(),
            'x' => digit.is_ascii_hexdigit(),
            _ => panic!("Unknown radix")
        }
    }

    fn make_number_lit_token(&mut self) -> Result<Token, LexerError> {
        let mut lexeme = String::new();
        

        let chars_c = self.chars.len();


        lexeme.push(self.chars[self.index]);
        self.index += 1;

        

        let radix_prefix: char;

        if Self::is_radix_prefix(self.chars[self.index]) {
            lexeme.push(self.chars[self.index]);
            radix_prefix = self.chars[self.index];
            self.index += 1;
        } else {
            lexeme.push('d');
            radix_prefix = 'd';
        }

        if self.chars[self.index] == '-' {
            lexeme.push(self.chars[self.index]);
            self.index += 1;
        }
        
        while self.index < chars_c && Self::is_valid_digit(radix_prefix, self.chars[self.index]) {
            lexeme.push(self.chars[self.index]);
            self.index += 1;
        }

        if lexeme.len() == 2 {
            return Err(LexerError::new(lexer_error::LexerErrorKind::InvalidNumberLit(lexeme), self.line));
        }

        Ok(Token::new(TokenKind::Number, lexeme, self.line))
    }


    fn is_valid_string_char(c: char) -> bool {
        !c.is_ascii_control()
    }

    fn make_string_lit_token(&mut self) -> Result<Token, LexerError> {
        let mut lexeme = String::new();
        

        let chars_c = self.chars.len();


        lexeme.push(self.chars[self.index]);
        self.index += 1;

        while self.index < chars_c && self.chars[self.index] != '"' {
            if !Self::is_valid_string_char(self.chars[self.index]) {
                return Err(LexerError::new(lexer_error::LexerErrorKind::InvalidCharacterInString(self.chars[self.index]), self.line));
            }
            lexeme.push(self.chars[self.index]);
            self.index += 1;
        }
        if self.index >= chars_c {
            return Err(LexerError::new(lexer_error::LexerErrorKind::UnterminatedString, self.line));
        }

        lexeme.push(self.chars[self.index]);
        self.index += 1;


        Ok(Token::new(TokenKind::String, lexeme, self.line))
    }








    pub fn _tokenise(&mut self) -> Result<Vec<Token>, LexerError> {


        let mut tokens: Vec<Token> = Vec::new();
        

        let chars_c = self.chars.len();
        while self.index < chars_c {

            let char = self.chars[self.index];

            if char == ';' {
                self.drop_comment();

            } else if Self::is_word_char(char) {
                let new_word_token = self.make_word_token();
                tokens.push(new_word_token);

            } else if char == '!' {
                let new_macro_token = self.make_macro_token()?;
                tokens.push(new_macro_token);

            } else if char == '.' {
                let new_directive_token = self.make_directive_token()?;
                tokens.push(new_directive_token);

            } else if char == '#' {
                let new_number_token = self.make_number_lit_token()?;
                tokens.push(new_number_token);

            } else if char == '"' {
                let new_string_token = self.make_string_lit_token()?;
                tokens.push(new_string_token);

            } else if Self::is_punctation_character(char) {
                tokens.push(Token::new(TokenKind::Punctuation, String::from(self.chars[self.index]), self.line));
                self.index += 1;

            } else if char == '\n' { 
                tokens.push(Token::new(TokenKind::Punctuation, String::from("\n"), self.line));
                self.index += 1;
                self.line += 1;

            } else if char.is_ascii_whitespace() {
                self.index += 1;

            } else {
                return Err(LexerError::new(lexer_error::LexerErrorKind::UnknownSymbol(self.chars[self.index]), self.line));
            }

        }
        
        tokens.push(Token::eof_token(self.line));
        Ok(tokens)
    }
}





