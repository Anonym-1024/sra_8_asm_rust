use token::{Token, TokenKind};
use lexer_error::LexerError;



pub mod token;
pub mod lexer_error;
pub mod resources;


fn is_word_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || (c == '_')
}

fn is_punctation_character(c: char) -> bool {
    (c == '{') || (c == '}') || (c == ':') || (c == '*') || (c == '>')
}


fn drop_comment(chars: &[char], index: &mut usize) {
    let chars_c = chars.len();

    while *index < chars_c && chars[*index] != '\n' {
        
        *index += 1;
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


fn make_word_token(chars: &[char], index: &mut usize, line: i32) -> Token {
    let mut lexeme = String::new();
    

    let chars_c = chars.len();

    
    while *index < chars_c && is_word_char(chars[*index]) {
        lexeme.push(chars[*index]);
        *index += 1;
    }


    let kind = get_word_token_kind(&lexeme);
    Token::new(kind, lexeme, line)
}


fn make_macro_token(chars: &[char], index: &mut usize, line: i32) -> Result<Token, LexerError> {
    let mut lexeme = String::new();
    

    let chars_c = chars.len();


    lexeme.push(chars[*index]);
    *index += 1;

    while *index < chars_c && is_word_char(chars[*index]) {
        lexeme.push(chars[*index]);
        *index += 1;
    }

    if is_macro_name(&lexeme) {
        Ok(Token::new(TokenKind::Macro, lexeme, line))
    } else {
        Err(LexerError::new(lexer_error::LexerErrorKind::InvalidMacro(lexeme), line))
    }
}


fn make_directive_token(chars: &[char], index: &mut usize, line: i32) -> Result<Token, LexerError> {
    let mut lexeme = String::new();
    

    let chars_c = chars.len();


    lexeme.push(chars[*index]);
    *index += 1;

    while *index < chars_c && is_word_char(chars[*index]) {
        lexeme.push(chars[*index]);
        *index += 1;
    }

    if is_directive_name(&lexeme) {
        Ok(Token::new(TokenKind::Directive, lexeme, line))
    } else {
        Err(LexerError::new(lexer_error::LexerErrorKind::InvalidDirective(lexeme), line))
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

fn make_number_lit_token(chars: &[char], index: &mut usize, line: i32) -> Result<Token, LexerError> {
    let mut lexeme = String::new();
    

    let chars_c = chars.len();


    lexeme.push(chars[*index]);
    *index += 1;

    if chars[*index] == '-' {
        lexeme.push(chars[*index]);
        *index += 1;
    }

    let radix_prefix: char;

    if is_radix_prefix(chars[*index]) {
        lexeme.push(chars[*index]);
        radix_prefix = chars[*index];
        *index += 1;
    } else {
        lexeme.push('d');
        radix_prefix = 'd';
    }
    
    while *index < chars_c && is_valid_digit(radix_prefix, chars[*index]) {
        lexeme.push(chars[*index]);
        *index += 1;
    }

    if lexeme.len() == 2 {
        return Err(LexerError::new(lexer_error::LexerErrorKind::InvalidNumberLit(lexeme), line));
    }

    Ok(Token::new(TokenKind::Number, lexeme, line))
}


fn is_valid_string_char(c: char) -> bool {
    !c.is_ascii_control()
}

fn make_string_lit_token(chars: &[char], index: &mut usize, line: i32) -> Result<Token, LexerError> {
    let mut lexeme = String::new();
    

    let chars_c = chars.len();


    lexeme.push(chars[*index]);
    *index += 1;

    while *index < chars_c && chars[*index] != '"' {
        if !is_valid_string_char(chars[*index]) {
            return Err(LexerError::new(lexer_error::LexerErrorKind::InvalidCharacterInString(chars[*index]), line));
        }
        lexeme.push(chars[*index]);
        *index += 1;
    }
    if *index >= chars_c {
        return Err(LexerError::new(lexer_error::LexerErrorKind::UnterminatedString, line));
    }

    lexeme.push(chars[*index]);
    *index += 1;


    Ok(Token::new(TokenKind::String, lexeme, line))
}








pub fn tokenise(src: String) -> Result<Vec<Token>, LexerError> {

    let chars: Vec<char> = src.chars().collect();
    let chars_c = chars.len();
    let mut index = 0;
    let mut line = 1;

    let mut tokens: Vec<Token> = Vec::new();
    
    while index < chars_c {

        let char = chars[index];

        if char == ';' {
            drop_comment(&chars, &mut index);

        } else if is_word_char(char) {
            let new_word_token = make_word_token(&chars, &mut index, line);
            tokens.push(new_word_token);

        } else if char == '!' {
            let new_macro_token = make_macro_token(&chars, &mut index, line)?;
            tokens.push(new_macro_token);

        } else if char == '.' {
            let new_directive_token = make_directive_token(&chars, &mut index, line)?;
            tokens.push(new_directive_token);

        } else if char == '#' {
            let new_number_token = make_number_lit_token(&chars, &mut index, line)?;
            tokens.push(new_number_token);

        } else if char == '"' {
            let new_string_token = make_string_lit_token(&chars, &mut index, line)?;
            tokens.push(new_string_token);

        } else if is_punctation_character(char) {
            tokens.push(Token::new(TokenKind::Punctuation, String::from(chars[index]), line));
            index += 1;

        } else if char == '\n' { 
            tokens.push(Token::new(TokenKind::Punctuation, String::from("\n"), line));
            index += 1;
            line += 1;

        } else if char.is_ascii_whitespace() {
            index += 1;

        } else {
            return Err(LexerError::new(lexer_error::LexerErrorKind::UnknownSymbol(chars[index]), line));
        }

    }
    
    tokens.push(Token::eof_token(line));
    Ok(tokens)
}


