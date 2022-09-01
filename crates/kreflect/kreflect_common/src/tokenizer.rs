use crate::Token;

#[derive(Debug)]
pub struct TokenPosition {
    pub line: usize,
    pub last_character: usize,
}

fn peek_or<'a>(
    iter: &mut std::iter::Peekable<std::str::CharIndices>,
    c: char,
    a: Token<'a>,
    b: Token<'a>,
) -> Token<'a> {
    iter.next();
    match iter.peek() {
        Some((_, character)) if *character == c => {
            iter.next();
            a
        }
        _ => b,
    }
}

fn parse_number<'a>(
    i: usize,
    s: &str,
    iter: &mut std::iter::Peekable<std::str::CharIndices>,
) -> Token<'a> {
    while iter.peek().map_or(false, |c| c.1.is_digit(10)) {
        iter.next();
    }
    if iter.peek().map_or(false, |c| c.1 == '.') {
        iter.next();
        while iter.peek().map_or(false, |c| c.1.is_digit(10)) {
            iter.next();
        }
        let end = iter.peek().map_or(s.len(), |c| c.0);
        Token::FloatLiteral(s[i..end].parse().unwrap())
    } else {
        let end = iter.peek().map_or(s.len(), |c| c.0);
        Token::IntegerLiteral(s[i..end].parse().unwrap())
    }
}

pub fn tokenize(s: &str) -> (Vec<Token>, Vec<TokenPosition>) {
    use Token::*;

    let mut iter = s.char_indices().peekable();
    let mut tokens = Vec::new();
    let mut token_positions = Vec::new();
    let mut current_line = 0;
    while let Some((i, c)) = iter.peek().cloned() {
        tokens.push(match c {
            c if c.is_whitespace() => {
                if c == '\n' {
                    current_line += 1;
                }
                iter.next();
                continue;
            }
            '{' => {
                iter.next();
                OpenBrace
            }
            '}' => {
                iter.next();
                CloseBrace
            }
            '[' => {
                iter.next();
                OpenBracket
            }
            ']' => {
                iter.next();
                CloseBracket
            }
            '(' => {
                iter.next();
                OpenParentheses
            }
            ')' => {
                iter.next();
                CloseParentheses
            }
            ',' => {
                iter.next();
                Comma
            }
            ';' => {
                iter.next();
                SemiColon
            }
            '|' => peek_or(&mut iter, '|', VerticalBar, Or),
            '+' => peek_or(&mut iter, '=', PlusEqual, Plus),
            '*' => peek_or(&mut iter, '=', TimesEqual, Star),
            '!' => peek_or(&mut iter, '=', NotEqual, Not),
            ':' => peek_or(&mut iter, ':', Colon2, Colon),
            '=' => {
                iter.next();
                match iter.peek() {
                    Some((_, '=')) => {
                        iter.next();
                        EqualEqual
                    }
                    Some((_, '>')) => {
                        iter.next();
                        FatArrow
                    }
                    _ => Equal,
                }
            }
            '-' => {
                iter.next();
                match iter.peek() {
                    Some((_, '=')) => {
                        iter.next();
                        MinusEqual
                    }
                    Some((_, '>')) => {
                        iter.next();
                        RArrow
                    }
                    Some((_, c)) if c.is_digit(10) => parse_number(i, s, &mut iter),
                    _ => Minus,
                }
            }

            '.' => {
                iter.next();
                Dot
            }
            '<' => {
                iter.next();
                match iter.peek() {
                    Some((_, '=')) => {
                        iter.next();
                        Token::LessThanOrEqual
                    }
                    _ => LessThan,
                }
            }
            '>' => {
                iter.next();
                match iter.peek() {
                    Some((_, '=')) => {
                        iter.next();
                        Token::GreaterThanOrEqual
                    }
                    _ => GreaterThan,
                }
            }
            '/' => {
                iter.next();
                // If a comment occurs skip the rest of the line.
                match iter.peek() {
                    Some((_, '/')) => {
                        while iter.peek().map_or(false, |c| c.1 != '\n') {
                            iter.next();
                        }
                        continue;
                    }
                    Some((_, '*')) => {
                        // Block comments support nesting.
                        let mut nesting = 1;
                        while let Some((_, c)) = iter.next() {
                            match c {
                                '*' => {
                                    if iter.next().map_or(false, |(_, c)| c == '/') {
                                        nesting -= 1;
                                        if nesting == 0 {
                                            break;
                                        }
                                    }
                                }
                                '/' => {
                                    if iter.next().map_or(false, |(_, c)| c == '*') {
                                        nesting += 1;
                                    }
                                }
                                _ => {}
                            }
                        }
                        continue;
                    }
                    Some((_, '=')) => DivEqual,
                    _ => Div,
                }
            }
            c if c.is_digit(10) => parse_number(i, s, &mut iter),
            '\'' => {
                iter.next();
                match iter.peek().cloned() {
                    Some((_, '\\')) => {
                        iter.next();
                        let token = match iter.peek() {
                            Some((_, '\\')) => CharLiteral('\\'),
                            Some((_, 'n')) => CharLiteral('\n'),
                            Some((_, 'r')) => CharLiteral('\r'),
                            Some((_, 't')) => CharLiteral('\t'),
                            Some((_, '0')) => CharLiteral('\0'),
                            Some((_, 'x')) => unimplemented!(),
                            Some((_, 'u')) => unimplemented!(),
                            Some((_, c)) => CharLiteral(*c),
                            None => panic!("Expected token"),
                        };
                        iter.next();
                        match iter.peek() {
                            Some((_, '\'')) => iter.next(),
                            _ => panic!("Expected closing \'"),
                        };
                        token
                    }
                    Some((_, c)) => {
                        iter.next();
                        let (_, c_next) = iter.peek().cloned().expect("Unexpected end of file");
                        match c_next {
                            '\'' => {
                                // This is a character
                                iter.next();
                                CharLiteral(c)
                            }
                            _ => {
                                panic!("Extra chararacter in character");
                            }
                        }
                    }
                    None => panic!("Unexpected end of file in character"),
                }
            }
            '"' => {
                iter.next();
                while let Some((_, c)) = iter.peek() {
                    match c {
                        '\\' => {
                            // Instead of this should a new string be allocated with the escape already handled?
                            iter.next();
                            iter.next();
                        }
                        '"' => break,
                        _ => {
                            iter.next();
                        }
                    }
                }
                iter.next();

                let end = iter.peek().map_or(s.len(), |c| c.0);
                if end - i == 0 {
                    panic!("Missing closing \"");
                }

                let identifier = &s[i + 1..end - 1];
                Token::StringLiteral(identifier.into())
            }
            _ => {
                // Instead of a block-list here this should instead use an allow-list.
                // But for now this is ok.
                fn is_identifier_character(c: char) -> bool {
                    match c {
                        c if c.is_whitespace() => false,
                        ':' | '+' | '-' | '*' | '(' | ')' | '{' | '}' | '^' | '/' | '.' | '|'
                        | '&' | '"' | '\'' | '!' | ',' | '>' | '<' => false,
                        _ => true,
                    }
                }
                while iter
                    .peek()
                    .map_or(false, |(_, c)| is_identifier_character(*c))
                {
                    iter.next();
                }

                let end = iter.peek().map_or(s.len(), |c| c.0);
                if end - i == 0 {
                    panic!("Unknown token: {:?}", iter.peek());
                }

                let identifier = &s[i..end];
                match identifier {
                    "fn" => Fn,
                    "let" => Let,
                    "if" => If,
                    "mod" => Mod,
                    "true" => BoolLiteral(true),
                    "false" => BoolLiteral(false),
                    "else" => Else,
                    "return" => Return,
                    "for" => For,
                    "in" => In,
                    _ => Token::Identifier(identifier.into()),
                }
            } // _ => panic!("Unexpected character: {:?}", c),
        });
        token_positions.push(TokenPosition {
            line: current_line,
            last_character: iter.peek().map_or(s.len(), |(i, _)| *i),
        });
    }
    (tokens, token_positions)
}
