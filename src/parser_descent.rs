#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::lexer::Lexer;
use crate::token::Token;

const INDENT : usize = 2;


pub fn main() {

    // create a sequence of tokens that is assumed to
    //   be output of the lexer
    let tokens = vec![
        Token::FUNC,
        Token::ID(String::from("add")),
        Token::PARENS_L,
        Token::ID(String::from("a")),
        Token::COLON,
        Token::ID(String::from("i32")),
        Token::COMMA,
        Token::ID(String::from("b")),
        Token::COLON,
        Token::ID(String::from("i32")),
        Token::PARENS_R,
        Token::ARROW_R,
        Token::ID(String::from("i32")),
        Token::BRACKET_L,
        Token::BRACKET_L,
        Token::BRACKET_L,
        Token::BRACKET_R,
        Token::BRACKET_L,
        Token::BRACKET_R,
        Token::BRACKET_R,
        Token::BRACKET_L,
        Token::BRACKET_R,
        Token::BRACKET_R,
    ];

    let mut input = String::from("\
    func add(x : int32) -> int32\
    [\
        let value : int32 = 35;\
        value = value + x;\
        return value;\
    ]\
    \
    func main()\
    [\
        let sum : int32 = 0;\
        sum = add(2) + 5;\
        print \"sum=\", sum;\
    ]");

    // create recursive descent parser
    let lexer = Lexer::new(input);
    let mut parser = DescentParser::new(lexer);

    // start recursive descent parsing
    parser.analyze();

}


struct DescentParser {
    lexer: Lexer,
    indent: usize,
}


impl DescentParser {  // simple recursive descend parser

    fn new(lexer: Lexer) -> DescentParser {
        DescentParser {
            lexer,
            indent: 0,
        }
    }

    pub fn analyze(&mut self) {
        self.indent = 0;
        self.parse_func();
        self.expect(Token::EOI);
    }

    fn parse_func(&mut self) {
        self.indent_print("parse_func()");
        self.indent_increment();
        {
            self.expect(Token::FUNC);
            self.expect(Token::id());
            self.parse_parameter_list();
            self.expect(Token::ARROW_R);
            self.expect(Token::id());
            self.parse_block_nest();
        }
        self.indent_decrement();
    }

    fn parse_parameter_list(&mut self) {
        self.indent_print("parse_parameter_list()");
        self.indent_increment();
        {
            self.expect(Token::PARENS_R);
            if self.accept(Token::PARENS_R) {
                return;
            }
            self.parse_parameter();
            while self.accept(Token::COMMA) {
                self.parse_parameter();
            }
            self.expect(Token::PARENS_R);
        }
        self.indent_decrement();
    }

    fn parse_parameter(&mut self) {
        self.indent_print("parse_parameter()");
        self.indent_increment();
        {
            self.expect(Token::id());
            self.expect(Token::COLON);
            self.expect(Token::id());
        }
        self.indent_decrement();
    }

    fn parse_block_nest(&mut self) {
        self.indent_print("parse_block_nest()");
        self.indent_increment();
        {
            self.expect(Token::BRACKET_L);
            if self.peek(Token::BRACKET_L) {
                self.parse_block_list();
            }
            self.expect(Token::BRACKET_R);
        }
        self.indent_decrement();
    }

    fn parse_block_list(&mut self) {
        self.indent_print("parse_block_list()");
        self.indent_increment();
        {
            self.parse_block_nest();
            if self.peek(Token::BRACKET_L) {
                self.parse_block_list()
            }
        }
        self.indent_decrement();
    }
}


impl DescentParser { // utility functions for lexer

    fn curr(&mut self) -> Token {
        self.lexer.curr()
    }

    fn advance(&mut self) {
        self.lexer.advance();
    }

    fn expect(&mut self, symbol: Token) {
        if self.curr() == symbol {
            self.advance();
            println!("{:<indent$}expect({symbol:?})", "", indent = self.indent);
        } else {
            panic!("Did not expect '{symbol:?}'!");
        }
    }

    fn accept(&mut self, symbol: Token) -> bool {
        if self.curr() == symbol {
            self.advance();
            true
        } else {
            false
        }
    }

    fn peek(&mut self, symbol: Token) -> bool {
        self.lexer.curr() == symbol
    }

}


impl DescentParser { // utility functions for pretty print

    fn indent_print(&mut self, msg: &'static str) {
        println!("{:<indent$}{:}", "", msg, indent=self.indent);
    }

    fn indent_increment(&mut self) {
        self.indent += INDENT;
    }
    fn indent_decrement(&mut self) {
        self.indent -= INDENT;
    }

}
