use crate::lexer::token::{Token, TokenKind};



pub fn num_lit_to_int(token: &Token) -> i32 {
    assert_eq!(token.kind, TokenKind::Number);

    let radix = token.lexeme.chars().nth(1).unwrap();
    print!("{radix}");
    let string = &token.lexeme[2..];
    match radix {
        'b' => i32::from_str_radix(string, 2),
        'o' => i32::from_str_radix(string, 8),
        'd' => i32::from_str_radix(string, 10),
        'x' => i32::from_str_radix(string, 16),
        _ => unreachable!(),
    }.unwrap()
}

pub fn str_lit_to_str(token: &Token) -> Vec<char> {
    assert_eq!(token.kind, TokenKind::String);

    token.lexeme[1..(token.lexeme.len()-1)].chars().collect()
}