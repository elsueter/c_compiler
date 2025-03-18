mod token_types;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

#[repr(u8)]
#[derive(Clone, Copy)] //TODO remove this trait if possible. Copy is cheap but clone is not
enum TokenType {
    Type,
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

//debug printing function
impl Token {
    fn render(&self) -> String {
        let mut output = self.val.clone();

        match self.t_type as u8 {
            0 => output += " : Type",
            1 => output += " : Identifier",
            2 => output += " : Keyword",
            3 => output += " : Seperator",
            4 => output += " : Operator",
            5 => output += " : Literal",
            6 => output += " : Comment",
            7 => output += " : Whitespace",
            _ => (),
        }

        output
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> String {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let out: String = buf.lines().map(|line| line.expect("error")).collect();
    out
}

//TODO make the list of tokens this can read complete. It is currently not
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
            //Operator [ ]
            42..=46 | 60..=62 | 91 | 93 => {
                if cur_token_type as u8 != TokenType::Operator as u8
                    && cur_token_type as u8 != TokenType::Whitespace as u8
                {
                    output.push(Token {
                        t_type: cur_token_type,
                        val: cur_token_string,
                    });
                    cur_token_string = "".to_string();
                }
                cur_token_type = TokenType::Operator;
                cur_token_string.push(*c as char);
            }
            //Literal
            //Comment
            //Any alphanumeric value or _ "
            47..=57 | 65..=90 | 97..=122 | 95 | 34 => {
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
            _ => (), //?
        }
    }
    output.push(Token {
        t_type: cur_token_type,
        val: cur_token_string,
    });
    output
}

//Converts any given "Identifier" type to more specific token types based on the string
fn parse_type(input: &str) -> TokenType {
    let types = vec!["int", "void"];
    let keywords = vec!["if"];
    let operators = vec!["=="];

    //potentially unsafe but if an empty string hits here then other things are wrong
    if input.chars().next().unwrap() == '"' {
        return TokenType::Literal;
    }
    //TODO unsafe and also not capturing corretctly
    if input.chars().next().unwrap() == '/' {
        return TokenType::Comment;
    }

    //these are safe and fine
    if input.chars().all(|x| x.is_numeric()) {
        return TokenType::Literal;
    }
    if types.contains(&input) {
        return TokenType::Type;
    }
    if keywords.contains(&input) {
        return TokenType::Keyword;
    }
    if operators.contains(&input) {
        return TokenType::Operator;
    }

    return TokenType::Identifier;
}

pub fn run_lexer(debug: bool) {
    //read file
    let lines = lines_from_file("test_code/input.c");
    //tokenise string
    let mut tokens = tokenise(lines);
    //update token metadata (identifier) with relevant type. (this is what makes this a lexer)
    for t in tokens.iter_mut() {
        if t.t_type as u8 == TokenType::Identifier as u8 {
            t.t_type = parse_type(&t.val);
        }
    }
    //temp debug switch for "rendering" code
    if debug {
        for t in tokens.iter_mut() {
            println!("{}", t.render());
        }
    }
    //put into AST
}

