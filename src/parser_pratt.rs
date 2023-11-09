use crate::token::Token;
use crate::lexer::Lexer;
use crate::parse_tree::ParseTree;

pub fn main() {

    // create a sequence of tokens that is assumed to
    //   be output of the lexer
    let tokens = vec![
        Token::ID(String::from("a")),
        Token::ASSIGN, Token::ID(String::from("b")),
        Token::ADD, Token::ID(String::from("c")),
        Token::ADD, Token::ID(String::from("d")),
        Token::ASSIGN, Token::ID(String::from("e")),
        Token::ASSIGN, Token::ID(String::from("f")),
        Token::ASSIGN, Token::ID(String::from("g")),
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

    // create Pratt parser
    let lexer = Lexer::new(input);
    let mut parser = PrattParser::new(lexer);

    // start Pratt top-down operator precedence parsing
    let tree = parser.analyze();

    // print parse tree
    tree.print();

}

struct PrattParser {
    lexer: Lexer,
}

impl PrattParser {
    fn new(lexer : Lexer) -> PrattParser {
        PrattParser { lexer }
    }

    fn analyze(&mut self) -> ParseTree {
        self.pratt_driver(Token::EOI.right_bp() )
    }

    fn pratt_driver(&mut self, requested_bp: i32) -> ParseTree {
        let mut current_token = self.current();
        self.advance();
        let mut left_denotation = self.func_prefix(current_token);
        loop {
            current_token = self.current();
            // compare binding powers
            if requested_bp >= current_token.left_bp() {
                // finish subexpression (requested rbp >= curr lbp)
                return left_denotation;
            }
            // go on with subexpression (requested rbp < curr lbp)
            self.advance();
            left_denotation = self.func_infix(current_token, left_denotation);
        }
    }

    pub fn func_prefix(&mut self, token: Token) -> ParseTree {
        match token {
            Token::ID(_) => {
                ParseTree::new(token.clone())
            }
            Token::ADD => { todo!() }
            Token::ASSIGN => { todo!() }
            Token::EOI => { todo!() }
            _ => {
                panic!("Missing prefix function for token {:?}", token);
            }
        }
    }

    fn func_infix(&mut self, token: Token, left_denotation : ParseTree) -> ParseTree {
        match token {
            Token::LIT_INT32(_) => { todo!() }
            Token::ADD => {
                let mut node = ParseTree::new(token.clone());
                let right_denotation = self.pratt_driver(token.right_bp());
                node.push(left_denotation);
                node.push(right_denotation);
                node
            }
            Token::ASSIGN => {
                let mut node = ParseTree::new(token.clone());
                let right_denotation = self.pratt_driver(token.right_bp());
                node.push(left_denotation);
                node.push(right_denotation);
                node
            }
            Token::EOI => { todo!() }
            _ => {
                panic!("Missing infix function for token {:?}", token);
            }
        }
    }
}


impl PrattParser { // utility functions for lexer

    fn current(&mut self) -> Token {
        self.lexer.curr()
    }

    fn advance(&mut self) {
        self.lexer.advance();
    }

}


impl Token {

    fn binding_powers(token : &Token) -> (i32, i32) {
        match token {
            Token::ID(_) => (3,3),
            Token::ADD => (2,3),
            Token::ASSIGN=> (2,1),
            Token::EOI => (0,0),
            _ => {
                panic!("Missing binding powers for token {:?}", token);
            }
        }
    }

    fn left_bp(&self) -> i32 { Token::binding_powers(self).0 }
    fn right_bp(&self) -> i32 { Token::binding_powers(self).1 }

}


