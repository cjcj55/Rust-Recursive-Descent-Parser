// CS 1163
// Chris Perrone

use crate::token::Token;

#[allow(non_camel_case_types, dead_code)]

#[allow(dead_code)]
enum LexerState {
    Start,
    InIdentifier,
    InNumber,
    InReservedWord,
    InType,
    InStringLiteral,
    End,
    ArrowR,
}

#[allow(dead_code)]
pub(crate) struct Lexer {
    input: String,
    pos: usize,
    state: LexerState,
    current_token: Token,
    buffer: String,
}

#[allow(dead_code)]
impl Lexer {
    pub(crate) fn new(input: String) -> Self {
        Self {
            input,
            pos: 0,
            state: LexerState::Start,
            current_token: Token::EOI,
            buffer: String::new(),
        }
    }

    pub(crate) fn set_input(&mut self, input: String) {
        self.input = input;
        self.pos = 0;
        self.state = LexerState::Start;
        self.current_token = Token::EOI;
        self.buffer.clear();
    }

    pub(crate) fn advance(&mut self) -> Token {
        loop {
            if self.pos >= self.input.len() {
                self.state = LexerState::End;
                return Token::EOI;
            }

            let c = self.input.chars().nth(self.pos).unwrap();

            match self.state {
                LexerState::Start => {
                    match c {
                        '(' => {
                            self.state = LexerState::Start;
                            self.pos += 1;
                            return Token::PARENS_L;
                        }
                        ')' => {
                            self.state = LexerState::Start;
                            self.pos += 1;
                            return Token::PARENS_R;
                        }
                        '[' => {
                            self.state = LexerState::Start;
                            self.pos += 1;
                            return Token::BRACKET_L;
                        }
                        ']' => {
                            self.state = LexerState::Start;
                            self.pos += 1;
                            return Token::BRACKET_R;
                        }
                        '{' => {
                            self.state = LexerState::Start;
                            self.pos += 1;
                            return Token::BRACE_L;
                        }
                        '}' => {
                            self.state = LexerState::Start;
                            self.pos += 1;
                            return Token::BRACE_R;
                        }
                        '"' => {
                            self.state = LexerState::InStringLiteral;
                            self.pos += 1;
                        }
                        '.' => {
                            self.state = LexerState::Start;
                            self.pos += 1;
                            return Token::POINT;
                        }
                        ',' => {
                            self.state = LexerState::Start;
                            self.pos += 1;
                            return Token::COMMA;
                        }
                        ':' => {
                            self.state = LexerState::Start;
                            self.pos += 1;
                            return Token::COLON;
                        }
                        ';' => {
                            self.state = LexerState::Start;
                            self.pos += 1;
                            return Token::SEMICOLON;
                        }
                        '>' => {
                            self.state = LexerState::Start;
                            self.pos += 1;
                            return Token::GT;
                        }
                        '<' => {
                            self.state = LexerState::Start;
                            self.pos += 1;
                            return Token::LT;
                        }
                        '+' => {
                            self.state = LexerState::Start;
                            self.pos += 1;
                            return Token::ADD;
                        }
                        '-' => {
                            self.state = LexerState::ArrowR;
                            self.buffer.push(c);
                            self.pos += 1;
                        }
                        '*' => {
                            self.state = LexerState::Start;
                            self.pos += 1;
                            return Token::MUL;
                        }
                        '/' => {
                            self.state = LexerState::Start;
                            self.pos += 1;
                            return Token::DIV;
                        }
                        '=' => {
                            self.state = LexerState::Start;
                            self.pos += 1;
                            return Token::ASSIGN;
                        }
                        'a'..='z' | 'A'..='Z' => {
                            self.state = LexerState::InIdentifier;
                            self.buffer.push(c);
                            self.pos += 1;
                        }
                        '0'..='9' => {
                            self.state = LexerState::InNumber;
                            self.buffer.push(c);
                            self.pos += 1;
                        }
                        _ => {
                            self.pos += 1;
                        }
                    }
                }
                LexerState::InIdentifier => {
                    match c {
                        'a'..='z' | 'A'..='Z' | '0'..='9' => {
                            self.buffer.push(c);
                            self.pos += 1;
                        }
                        _ => {
                            let word = self.buffer.clone();
                            self.buffer.clear();
                            self.state = LexerState::Start;
                            if word == "func" {
                                return Token::FUNC;
                            } else if word == "let" {
                                return Token::LET;
                            } else if word == "if" {
                                return Token::IF;
                            } else if word == "then" {
                                return Token::THEN;
                            } else if word == "else" {
                                return Token::ELSE;
                            } else if word == "while" {
                                return Token::WHILE;
                            } else if word == "print" {
                                return Token::PRINT;
                            } else if word == "return" {
                                return Token::RETURN;
                            } else if word == "int32" {
                                return Token::TYPE_INT32;
                            } else if word == "flt32" {
                                return Token::TYPE_FLT32;
                            } else if word == "char" {
                                return Token::TYPE_CHAR;
                            } else {
                                return Token::ID(word);
                            }
                        }
                    }
                }
                LexerState::InNumber => {
                    match c {
                        '0'..='9' => {
                            self.buffer.push(c);
                            self.pos += 1;
                        }
                        _ => {
                            let num = self.buffer.parse::<i32>().unwrap();
                            self.buffer.clear();
                            self.state = LexerState::Start;
                            return Token::LIT_INT32(num);
                        }
                    }
                }
                LexerState::InStringLiteral => {
                    match c {
                        '"' => {
                            let word = self.buffer.clone();
                            self.buffer.clear();
                            self.state = LexerState::Start;
                            self.pos += 1;
                            return Token::LIT_STRING(word);
                        }
                        _ => {
                            self.buffer.push(c);
                            self.pos += 1;
                        }
                    }
                }
                LexerState::ArrowR => {
                    match c {
                        '>' => {
                            self.state = LexerState::Start;
                            self.buffer.push(c);
                            self.pos += 1;
                            if self.buffer.clone() == "->" {
                                self.pos += 1; // Advance to the next character after identifying "->"
                                self.buffer.clear();
                                return Token::ARROW_R;
                            }
                        }
                        _ => {
                            let word = self.buffer.clone();
                            self.buffer.clear();
                            self.state = LexerState::Start;
                            if word == "->" {
                                self.pos += 1; // Advance to the next character after identifying "->"
                                return Token::ARROW_R;
                            } else {
                                self.pos += 1; // Advance to the next character after identifying "-"
                                return Token::SUB;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    pub(crate) fn curr(&self) -> Token {
        if self.pos >= self.input.len() {
            return Token::EOI;
        }
        self.current_token.clone()
    }

    pub(crate) fn print_tokens(&mut self) {
        loop {
            let token = self.advance();
            println!("{:?}", token);
            if token == Token::EOI {
                break;
            }
        }
    }
}