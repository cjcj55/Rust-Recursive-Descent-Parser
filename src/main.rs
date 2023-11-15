// CS 1163
// Chris Perrone

mod token;
mod lexer;
mod parse_tree;
mod parser_descent;

use lexer::Lexer;

fn main() {
    //let mut lexer = Lexer::new(String::from("func add(freevar : int32, y : flt32) -> int32"));
    //lexer.print_tokens();

    let mut input = Lexer::new(String::from("\
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
    ]"));
    //input.print_tokens();

    parser_descent::main();
}
