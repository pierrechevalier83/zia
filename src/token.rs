/*  Copyright (C) 2018  Charles Johnson

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
along with this program. If not, see <http://www.gnu.org/licenses/>.*/
#[cfg(test)]
mod tokens {
    use super::parse_line;
    use super::parse_tokens;
    use Token::Atom;
    use Token::Expression;
    #[test]
    fn monad() {
        let parsed_line = parse_line("(not true)->");
        assert_eq!(parsed_line, ["not true", "->"].to_vec());
        assert_eq!(
            parse_tokens(&parsed_line),
            [Expression("not true".to_string()), Atom("->".to_string())].to_vec()
        );
    }
    #[test]
    fn diad() {
        assert_eq!(parse_line("(0 + 1)->"), ["0 + 1", "->"].to_vec());
    }
    #[test]
    fn lambda() {
        assert_eq!(
            parse_line("((lambda x_)(_f _x))_y ->"),
            ["(lambda x_)(_f _x)", "_y", "->"].to_vec()
        );
    }
}

pub fn parse_tokens(tokens: &Vec<String>) -> Vec<Token> {
    let mut new_tokens: Vec<Token> = [].to_vec();
    for token in tokens {
        if token.contains(" ") {
            new_tokens.push(Token::Expression(token[..].to_string()));
        } else {
            new_tokens.push(Token::Atom(token[..].to_string()));
        }
    }
    new_tokens
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Atom(String),
    Expression(String),
}

pub fn parse_line(buffer: &str) -> Vec<String> {
    let mut tokens: Vec<String> = [].to_vec();
    let mut token = String::new();
    let mut parenthesis_level = 0;
    for letter in buffer.chars() {
        parse_letter(letter, &mut parenthesis_level, &mut token, &mut tokens);
    }
    if token != "" {
        tokens.push(token.clone());
    }
    tokens
}

fn parse_letter(
    letter: char,
    parenthesis_level: &mut i8,
    token: &mut String,
    tokens: &mut Vec<String>,
) {
    match letter {
        '(' => {
            push_token(letter, parenthesis_level, token, tokens);
            *parenthesis_level += 1;
        }
        ')' => {
            *parenthesis_level -= 1;
            push_token(letter, parenthesis_level, token, tokens);
        }
        ' ' => push_token(letter, parenthesis_level, token, tokens),
        '\n' | '\r' => (),
        _ => token.push(letter),
    };
}

fn push_token(letter: char, parenthesis_level: &i8, token: &mut String, tokens: &mut Vec<String>) {
    if (token != "") & (*parenthesis_level == 0) {
        tokens.push(token.clone());
        *token = String::new();
    }
    if *parenthesis_level != 0 {
        token.push(letter);
    }
}
