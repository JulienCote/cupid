#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Integer(i64),
    Decimal(f64),
    StringLiteral(String),
    Operator(char), // e.g. +, -, *, %

    Colon,
    Semicolon,

    LBracket,
    RBracket,
    LCurly,
    RCurly,
    LParenthesis,
    RParenthesis,

    // Quote,
    // SingleQuote,
    Keyword(String),
    Identifier(String), // user-defined identifiers
}

fn parse_number<I>(input: &mut std::iter::Peekable<I>) -> Result<Token, String>
where
    I: Iterator<Item = char>,
{
    let mut num = String::new();
    let mut is_decimal = false;

    while let Some(&c) = input.peek() {
        if c.is_ascii_digit() {
            num.push(c);
        } else if c == '.' {
            if is_decimal {
                return Err(format!("Invalid number format: {}", num));
            }
            num.push(c);
            is_decimal = true;
        } else if c == '-' {
            if !num.is_empty() {
                break;
            }
            num.push(c);
        } else {
            break;
        }
        input.next();
    }

    if is_decimal {
        if let Ok(float_val) = num.parse::<f64>() {
            Ok(Token::Decimal(float_val))
        } else {
            Err(format!("Invalid number format: {}", num))
        }
    } else {
        if let Ok(int_val) = num.parse::<i64>() {
            Ok(Token::Integer(int_val))
        } else {
            Err(format!("Invalid number format: {}", num))
        }
    }
}

pub fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        // println!("Current char: {}", ch);
        match ch {
            '0'..='9' => {
                let token = parse_number(&mut chars)?;
                tokens.push(token);
            }

            '-' => {
                chars.next();
                if let Some(&next_ch) = chars.peek() {
                    if next_ch.is_ascii_digit() {
                        let token = parse_number(&mut chars)?;
                        let token = match token {
                            Token::Integer(i) => Token::Integer(-i),
                            Token::Decimal(f) => Token::Decimal(-f),
                            _ => unreachable!(),
                        };
                        tokens.push(token);
                        continue;
                    }
                }
                tokens.push(Token::Operator('-'));
            }

            '+' | '*' | '/' | ',' | '%' => {
                tokens.push(Token::Operator(ch));
                chars.next();
            }

            ';' => {
                tokens.push(Token::Semicolon);
                chars.next();
            }
            ':' => {
                tokens.push(Token::Colon);
                chars.next();
            }
            '[' => {
                tokens.push(Token::LBracket);
                chars.next();
            }
            ']' => {
                tokens.push(Token::RBracket);
                chars.next();
            }
            '"' => {
                let mut string_literal = String::new();
                chars.next();
                let mut is_string_terminated = false;
                while let Some(&c) = chars.peek() {
                    if c == '"' {
                        is_string_terminated = true;
                        chars.next();
                        break;
                    } else {
                        string_literal.push(c);
                        chars.next();
                    }
                }
                if !is_string_terminated {
                    return Err(format!("Unterminated string literal"));
                }
                tokens.push(Token::StringLiteral(string_literal));
            }

            // skip whitespace
            ' ' | '\t' | '\n' => {
                chars.next();
            }
            _ => {
                // identifiers (like foo, bar)
                if ch.is_alphabetic() {
                    let mut ident = String::new();
                    while let Some(&c) = chars.peek() {
                        if c.is_alphanumeric() {
                            ident.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Identifier(ident));
                } else {
                    return Err(format!("Unexpected char: {ch}"));
                }
            }
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number_integer() {
        let mut chars = "123abc".chars().peekable();
        let tok = parse_number(&mut chars).expect("should parse integer");
        assert_eq!(tok, Token::Integer(123));
        assert_eq!(chars.peek(), Some('a').as_ref());
    }

    #[test]
    fn parse_number_negative_integer() {
        let mut chars = "-42+".chars().peekable();
        let tok = parse_number(&mut chars).expect("should parse negative integer");
        assert_eq!(tok, Token::Integer(-42));

        println!("Peek after parsing: {:?}", chars.peek());
        assert_eq!(chars.peek(), Some('+').as_ref());
    }

    #[test]
    fn parse_number_decimal() {
        let mut chars = "3.14;".chars().peekable();
        let tok = parse_number(&mut chars).expect("should parse decimal");
        assert_eq!(tok, Token::Decimal(3.14));
        assert_eq!(chars.peek(), Some(';').as_ref());
    }

    #[test]
    fn parse_number_negative_decimal() {
        let mut chars = "-0.5 ".chars().peekable();
        let tok = parse_number(&mut chars).expect("should parse negative decimal");
        assert_eq!(tok, Token::Decimal(-0.5));
        assert_eq!(chars.peek(), Some(' ').as_ref());
    }

    #[test]
    fn parse_number_invalid_multiple_dots() {
        let mut chars = "1.2.3".chars().peekable();
        assert!(parse_number(&mut chars).is_err());
    }

    #[test]
    fn parse_number_multiple_minus() {
        let mut chars = "-5-5".chars().peekable();
        let tok = parse_number(&mut chars).expect("should parse -5");
        assert_eq!(tok, Token::Integer(-5));
    }
}
