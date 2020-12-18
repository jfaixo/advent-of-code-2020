use std::fs;
use crate::model::{Expression, Number, Addition, Multiplication};

pub fn parse_text_file(file_name: String) -> Vec<Expression> {
    let file_content = fs::read_to_string(file_name)
        .expect("Error while reading the data file");

    parse_string(file_content)
}

pub fn parse_string(content: String) -> Vec<Expression> {
    content.split("\n")
        .map(|line| {
            let mut lexer = Lexer { line: line.to_string(), current_index: 0 };

            let expression : Expression = parse(&mut lexer);
            expression
        })
        .collect()
}

fn parse (lexer: &mut Lexer) -> Expression {
    let mut previous_expression: Expression = Box::new(Number { value: 0 });

    while !lexer.is_empty() {
        let token = lexer.see_next_token();

        match token {
            LexerToken::LeftParenthesis => {
                lexer.pop_next_token();
                previous_expression = parse(lexer);
            }
            LexerToken::RightParenthesis => {
                lexer.pop_next_token();
                return previous_expression;
            }
            LexerToken::Number(_) => {
                previous_expression = get_next_expression(lexer);
            }
            LexerToken::Multiply => {
                lexer.pop_next_token();
                previous_expression = Box::new(Multiplication {
                    left: previous_expression,
                    right: get_next_expression(lexer),
                });
            }
            LexerToken::Add => {
                lexer.pop_next_token();
                previous_expression = Box::new(Addition {
                    left: previous_expression,
                    right: get_next_expression(lexer),
                });
            }
        }
    }

    return previous_expression;
}

fn get_next_expression(lexer: &mut Lexer) -> Expression {
    let token = lexer.pop_next_token();

    return match token {
        LexerToken::LeftParenthesis => {
            parse(lexer)
        }
        LexerToken::Number(value) => {
            Box::new(Number { value })
        }
        _ => { panic!("not normal") }
    }
}

//region Lexer
enum LexerToken {
    LeftParenthesis,
    RightParenthesis,
    Number(i64),
    Multiply,
    Add,
}

#[derive(Debug, Clone)]
struct Lexer {
    line: String,
    current_index: usize,
}

impl Lexer {
    fn is_empty(&self) -> bool {
        self.current_index == self.line.len()
    }

    fn see_next_token(&mut self) -> LexerToken {
        let saved_index = self.current_index;
        let token = self.pop_next_token();
        self.current_index = saved_index;
        return token;
    }

    fn pop_next_token(&mut self) -> LexerToken {
        loop {
            match &self.line.as_str()[self.current_index..self.current_index + 1] {
                " " => {
                    self.current_index += 1;
                },
                "(" => {
                    self.current_index += 1;
                    return LexerToken::LeftParenthesis;
                }
                ")" => {
                    self.current_index += 1;
                    return LexerToken::RightParenthesis;
                }
                "*" => {
                    self.current_index += 1;
                    return LexerToken::Multiply;
                }
                "+" => {
                    self.current_index += 1;
                    return LexerToken::Add;
                }
                _ => {
                    // Number
                    let mut value : i64 = 0;
                    let mut current_char = self.line.chars().nth(self.current_index).unwrap();
                    while self.line.chars().nth(self.current_index).unwrap().is_ascii_digit() {
                        value = 10 * value + current_char.to_digit(10).unwrap() as i64;
                        self.current_index += 1;

                        if self.current_index != self.line.len() {
                            current_char = self.line.chars().nth(self.current_index).unwrap();
                        }
                        else {
                            break;
                        }
                    }
                    return LexerToken::Number(value);
                }
            }
        }
    }
}
//endregion

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let content = "1 + 2 * 3 + 4 * 5 + 6".to_string();
        let expressions = parse_string(content);
        assert_eq!(expressions[0].evaluate(), 71);
    }

    #[test]
    fn example_2() {
        let content = "2 * 3 + (4 * 5)".to_string();
        let expressions = parse_string(content);
        assert_eq!(expressions[0].evaluate(), 26);
    }

    #[test]
    fn example_3() {
        let content = "5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string();
        let expressions = parse_string(content);
        assert_eq!(expressions[0].evaluate(), 437);
    }

    #[test]
    fn example_4() {
        let content = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string();
        let expressions = parse_string(content);
        assert_eq!(expressions[0].evaluate(), 12240);
    }

    #[test]
    fn example_5() {
        let content = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string();
        let expressions = parse_string(content);
        assert_eq!(expressions[0].evaluate(), 13632);
    }
}