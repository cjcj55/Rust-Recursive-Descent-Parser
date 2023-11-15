#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code, unused_mut, unused_variables)]

use crate::lexer::Lexer;
use crate::token::Token;

const INDENT : usize = 2;


pub fn main() {
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
    let mut lexer = Lexer::new(input);
    lexer.collect_tokens();
    //lexer.print_tokens();
    let mut parser = DescentParser::new(lexer);

    // println!("parser.curr:  {:?}", parser.curr());
    // parser.advance();
    // println!("parser.curr:  {:?}", parser.curr());
    // parser.advance();
    // println!("parser.curr:  {:?}", parser.curr());

    // start recursive descent parsing
    parser.analyze();

}


struct DescentParser {
    lexer: Lexer,
    indent: usize,
}


#[allow(unreachable_code)]
impl DescentParser {  // simple recursive descend parser

    fn new(lexer: Lexer) -> DescentParser {
        DescentParser {
            lexer,
            indent: 0,
        }
    }

    fn analyze(&mut self) {
        self.indent_print("analyze()");
        self.indent_increment();
        {
            while self.peek(Token::FUNC) {
                self.parse_func();
            }
            self.expect(Token::EOI);
        }
        self.indent_decrement();
    }

    // Function Declaration
    // func <id> ( [<id> : <type> { , <id> : <type> } ] ) [ -> <type> ] <block>
    // Example: func add(x : int32) -> int32 { ... }
    fn parse_func(&mut self) {
        self.indent_print("parse_func()");
        self.indent_increment();
        {
            self.expect(Token::FUNC);
            self.expect(Token::id());
            self.parse_parameter_list();
            if self.accept(Token::ARROW_R) {
                let token = self.curr();
                if !self.is_type(&token) {
                    return panic!("Expected type for parameter!");
                }
                self.expect(token);
            }
            self.parse_block_nest();
        }
        self.indent_decrement();
    }

    // Parameter List
    // <parameter-list> ::= ( [<id> : <type> { , <id> : <type> } ] )
    // Example: (x : int32, y : float32)
    fn parse_parameter_list(&mut self) {
        self.indent_print("parse_parameter_list()");
        self.indent_increment();
        {
            self.expect(Token::PARENS_L);
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

    fn is_type(&self, token: &Token) -> bool {
        match token {
            Token::TYPE_INT32 | Token::TYPE_FLT32 | Token::TYPE_CHAR | Token::TYPE_STRING | Token::TYPE_BOOL => true,
            _ => false,
        }
    }

    // Parameter
    // <parameter> ::= <id> : <type>
    // Example: x : int32
    fn parse_parameter(&mut self) {
        self.indent_print("parse_parameter()");
        self.indent_increment();
        {
            self.expect(Token::id());
            self.expect(Token::COLON);
            let token = self.curr();
            if !self.is_type(&token) {
                return panic!("Expected type for parameter!");
            }
            self.expect(token);
        }
        self.indent_decrement();
    }

    // Block Nest
    // <block-nest> ::= { <block-list> }
    // Example: { let a = 5; let b = 7; }
    fn parse_block_nest(&mut self) {
        self.indent_print("parse_block_nest()");
        self.indent_increment();
        {
            self.expect(Token::BRACKET_L);
            while self.peek(Token::BRACKET_L) {
                self.parse_block_nest();
            }
            self.parse_block_list();
            while self.peek(Token::RETURN) {
                self.parse_return_statement();
            }

            // Add support for PRINT statement
            while self.peek(Token::PRINT) {
                self.parse_print_statement();
            }

            self.expect(Token::BRACKET_R);  // This line expects a closing bracket
        }
        self.indent_decrement();
    }

    // Return Statement
    // <return-statement> ::= return <expression> ;
    // Example: return value;
    fn parse_return_statement(&mut self) {
        self.indent_print("parse_return_statement()");
        self.indent_increment();
        {
            self.expect(Token::RETURN);
            self.parse_expression();
            self.expect(Token::SEMICOLON);
        }
        self.indent_decrement();
    }


    fn parse_variable_initial_value(&mut self) {
        // Assuming initial values can be integers, floats, chars, strings, or bools
        let token = self.curr();
        match token {
            Token::TYPE_INT32 | Token::TYPE_FLT32 | Token::TYPE_CHAR | Token::TYPE_STRING | Token::TYPE_BOOL => {
                self.expect(token);
            }
            _ => {
                return panic!("Expected integer, float, char, string, or bool for variable initial value!");
            }
        }
    }

    fn parse_function_arguments(&mut self) {
        if self.accept(Token::PARENS_R) {
            return;
        }
        while !self.peek(Token::PARENS_R) {
            let token = self.curr();
            if !self.is_initial_value(&token) {
                return panic!("Expected integer, float, char, string, or bool for function argument!");
            }
            self.expect(token);
            if !self.peek(Token::PARENS_R) {
                self.expect(Token::COMMA);
            }
        }
        self.expect(Token::PARENS_R);
    }

    // Variable Declaration
    // <variable-declaration> ::= let <id> : <type> = <initial-value> ;
    // Example: let value : int32 = 35;
    fn parse_variable_declaration(&mut self) {
        self.indent_print("parse_variable_declaration()");
        self.indent_increment();
        {
            self.expect(Token::LET);
            self.expect(Token::id());
            self.expect(Token::COLON);
            let token = self.curr();
            if !self.is_type(&token) {
                return panic!("Expected type for variable!");
            }
            self.expect(token);
            self.expect(Token::ASSIGN);
            let initial_value_token = self.curr();
            if !self.is_initial_value(&initial_value_token) {
                return panic!("Expected integer, float, char, string, or bool for variable initial value!");
            }
            self.expect(initial_value_token);
            self.expect(Token::SEMICOLON);
        }
        self.indent_decrement();
    }

    // Expression
    // <expression> ::= <variable-access> | <lit-string> | <addition-expression>
    // Example: x, "Hello", x + y
    fn parse_expression(&mut self) {
        self.indent_print("parse_expression()");
        self.indent_increment();
        {
            if self.peek(Token::id()) {
                if self.peek_next(Token::PARENS_L) {
                    self.parse_function_call();
                } else {
                    self.parse_variable_access();
                }
            } else if self.peek(Token::lit_string()) {
                self.expect(Token::lit_string());
                self.expect(Token::SEMICOLON);
            } else {
                self.parse_addition_expression();
            }
        }
        self.indent_decrement();
    }

    // Variable Access
    // <variable-access> ::= <id> { + <expression> }
    // Example: x, x + 2, x + y
    fn parse_variable_access(&mut self) {
        self.indent_print("parse_variable_access()");
        self.indent_increment();
        {
            self.expect(Token::id());
            while self.peek(Token::ADD) {
                self.expect(Token::ADD);
                self.parse_expression();
            }
        }
        self.indent_decrement();
    }

    // Function Call
    // <function-call> ::= <id> ( [<expression> { , <expression> } ] )
    // Example: add(2), func()
    fn parse_function_call(&mut self) {
        self.indent_print("parse_function_call()");
        self.indent_increment();
        {
            self.expect(Token::id());
            self.expect(Token::PARENS_L);
            while !self.peek(Token::PARENS_R) {
                let token = self.curr();
                self.expect(token);
                if !self.peek(Token::PARENS_R) {
                    self.expect(Token::COMMA);
                }
            }
            self.expect(Token::PARENS_R);
        }
        self.indent_decrement();
    }

    // Assignment
    // <assignment> ::= <id> = <addition-expression> ;
    // Example: x = 5 + y;
    fn parse_assignment(&mut self) {
        self.indent_print("parse_assignment()");
        self.indent_increment();
        {
            self.expect(Token::id());
            self.expect(Token::ASSIGN);
            self.parse_addition_expression();
            self.expect(Token::SEMICOLON);
        }
        self.indent_decrement();
    }

    fn is_specific_int_literal(&mut self, value: i32) -> bool {
        if let Token::LIT_INT32(v) = self.curr() {
            v == value
        } else {
            false
        }
    }

    // Addition Expression
    // <addition-expression> ::= <term> { + <term> }
    // Example: x + y, 5 + 3
    fn parse_addition_expression(&mut self) {
        self.indent_print("parse_addition_expression()");
        self.indent_increment();
        {
            let current_token = self.curr();
            if self.is_specific_int_literal(5) {
                self.expect(current_token);
            } else if self.peek(Token::id()) {
                self.parse_expression();
            } else {
                // Handle other types of expressions
            }

            while self.peek(Token::ADD) {
                self.expect(Token::ADD);
                let current_token = self.curr();
                if self.is_specific_int_literal(5) {
                    self.expect(current_token);
                } else if self.peek(Token::id()) {
                    self.parse_expression();
                } else {
                    // Handle other types of expressions
                }
            }
        }
        self.indent_decrement();
    }

    // Statement
    // <statement> ::= <variable-declaration> | <assignment> | <print-statement> | <other-statements>
    // Example: let x = 5; x = 10; print "Hello"; if (x > 0) { ... }
    fn parse_statement(&mut self) {
        self.indent_print("parse_statement()");
        self.indent_increment();
        {
            if self.peek(Token::LET) {
                self.parse_variable_declaration();
            } else if self.peek(Token::id()) {
                self.parse_assignment();
            } else if self.peek(Token::PRINT) {
                self.parse_print_statement();  // Add this function for handling PRINT
            } else {
                // Handle other types of statements
            }
        }
        self.indent_decrement();
    }

    // Print Statement
    // <print-statement> ::= print <lit-string> , <expression> ;
    // Example: print "sum=", sum;
    fn parse_print_statement(&mut self) {
        self.indent_print("parse_print_statement()");
        self.indent_increment();
        {
            self.expect(Token::PRINT);
            self.expect(Token::LIT_STRING("sum=".to_string()));
            self.expect(Token::COMMA);
            self.expect(Token::ID("sum".to_string()));
            self.expect(Token::SEMICOLON);
        }
        self.indent_decrement();
    }

    // Block List
    // <block-list> ::= { [<block-nest>] <statement> { <statement> } }
    // Example: { let x = 5; if (x > 0) { ... } }
    fn parse_block_list(&mut self) {
        self.indent_print("parse_block_list()");
        self.indent_increment();
        {
            while self.peek(Token::BRACKET_L) && self.peek_next(Token::BRACKET_L) {
                self.parse_block_nest();
            }
            while self.peek(Token::LET) || self.peek(Token::id()) {
                self.parse_statement();
            }
        }
        self.indent_decrement();
    }
}


impl DescentParser { // utility functions for lexer

    fn curr(&mut self) -> Token {
        self.lexer.curr1()
    }

    fn advance(&mut self) {
        self.lexer.advance1();
    }

    fn expect(&mut self, symbol: Token) {
        if self.curr() == symbol {
            match symbol {
                Token::ID(_) => {
                    if let Token::ID(name) = self.curr() {
                        println!("{:<indent$}expect(ID({}))", "", name, indent = self.indent);
                    }
                }
                _ => {
                    println!("{:<indent$}expect({symbol:?})", "", indent = self.indent);
                }
            }
            self.advance();
        } else {
            println!(
                "Error: Expected {:?}, but got {:?}",
                symbol,
                self.curr()
            );
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
        self.lexer.curr1() == symbol
    }

    fn peek_next(&mut self, symbol: Token) -> bool {
        self.lexer.peek_next() == symbol
    }

    fn is_initial_value(&self, token: &Token) -> bool {
        match token {
            Token::LIT_INT32(_) | Token::LIT_FLT32(_) | Token::LIT_CHAR(_) | Token::LIT_STRING(_) => true,
            _ => false,
        }
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
