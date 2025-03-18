mod token_types;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

#[repr(u8)]
#[derive(Clone, Copy)] //TODO remove this trait if needed. Copy is cheap but clone is not
enum TokenType {
    Identifier,
    Keyword,
    Seperator,
    Operator,
    Literal,
    Comment,
    Whitespace,
}

struct Token {
    t_type: TokenType,
    val: String,
}

fn lines_from_file(filename: impl AsRef<Path>) -> String {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let out: String = buf.lines().map(|line| line.expect("error")).collect();
    out
}

fn tokenise(input: String) -> Vec<Token> {
    let mut output: Vec<Token> = vec![];
    let mut cur_token_string = "".to_string();
    let mut cur_token_type = TokenType::Whitespace;
    for c in input.as_bytes() {
        match c {
            //Whitespace
            // ' ' \n
            32 | 10 => {
                if cur_token_type as u8 != TokenType::Whitespace as u8 {
                    output.push(Token {
                        t_type: cur_token_type,
                        val: cur_token_string,
                    });
                }
                cur_token_string = "".to_string();
                cur_token_type = TokenType::Whitespace;
            }
            //Identifier
            //Keyword
            //Seperator
            // { } ( ) ;
            123 | 125 | 40 | 41 | 59 => {
                if cur_token_type as u8 != TokenType::Whitespace as u8 {
                    output.push(Token {
                        t_type: cur_token_type,
                        val: cur_token_string,
                    });
                }
                cur_token_type = TokenType::Seperator;
                cur_token_string = (*c as char).to_string();
            }
            //Operator
            42..=47 => {
                cur_token_type = TokenType::Operator;
                cur_token_string.push(*c as char);
            }
            //Literal
            //Comment
            //Catch all
            _ => {
                if cur_token_type as u8 != TokenType::Identifier as u8
                    && cur_token_type as u8 != TokenType::Whitespace as u8
                {
                    output.push(Token {
                        t_type: cur_token_type,
                        val: cur_token_string,
                    });
                    cur_token_string = "".to_string();
                }
                cur_token_type = TokenType::Identifier;
                cur_token_string.push(*c as char);
            }
        }
    }
    output.push(Token {
        t_type: cur_token_type,
        val: cur_token_string,
    });
    cur_token_string = "".to_string();
    output
}

pub fn run_lexer() {
    //read file
    let lines = lines_from_file("test_code/input.c");
    let tokens = tokenise(lines);
    for t in tokens {
        println!("{}, {}", t.t_type as u8, t.val);
    }
    //tokenise file
    //put into AST
}

