mod token_types;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

// type_specifier   : Int
//                  | Void
//                  | ...
//
// identifier       : [a-z][A-Z][0-9]
//
// parameter        : type_specifier identifier
//
// parameter_list   : ( parameter* )
//
// Operator         : Add
//                  | Minus
//                  | ...
//
// expression       : identifier
//                  | identifier operator
//                  | operator identifier
//                  | expression operator expression
//
// statement        : expression
//                  | conditional_statement
//
// body             : { statement* }
//
// func_declaration : type_specifier identifier parameter_list body
//
// declaration      : type_specifier identifier assignment_operator expression

// -------------------- Macro for defining enums and linked functions -----------

macro_rules! make_enum {
    (
        $name:ident $array:ident {
            $( $variant:ident, )*
        } $array2:ident{
            $( $variant_literal:literal, )*
        }
    ) => {
        #[derive(PartialEq)]
        enum $name {
            $( $variant, )*
        }
        impl $name{
            fn to_string(&self) -> &str{
                match self{
                    $( $name::$variant => $variant_literal, ) *
                }
            }
            fn new(input_string: &str) -> $name {
                match input_string{
                    $( $variant_literal => $name::$variant, )*
                    _ => $name::Any,
                }
            }
            fn get_literals() -> Vec<String>{
                let mut out = vec![];
                $( out.push($variant_literal.to_string());)*
                out
            }
        }
    }
}

//------------------- Basic Types --------------------------

make_enum! (Type VARIANTS{
    Any, //Temp value to allow a generic seperator TODO remove
    Int,
    Void,
}VARIANT_LITERAL{
    "",
    "int",
    "void",
});

type Identifier = String;

make_enum! (Keyword VARIANTS{
    Any, //Temp value to allow a generic seperator TODO remove
    If,
    While,
}VARIANT_LITERAL{
    "",
    "if",
    "while",
});

make_enum! (Seperator VARIANTS{
    Any, //Temp value to allow a generic seperator TODO remove
    OCB,
    CCB,
    OB,
    CB,
    SemiColon,
}VARIANT_LITERAL{
    "",
    "{",
    "}",
    "(",
    ")",
    ";",
});

make_enum! (Operator VARIANTS {
    Any, //Temp value to allow a generic seperator TODO remove
    Add,
    Minus,
    Divide,
    Multiply,
    Comma,
    Comparitor,
    Equal,
    OSB,
    CSB,
}VARIANT_LITERAL{
    "",
    "+",
    "-",
    "/",
    "*",
    ",",
    "==",
    "=",
    "[",
    "]",
});

//Some type verification needed
type Literal = String;

//Some type verification needed
type Comment = String;

//Some type verification needed
type Whitespace = String;

//------------------- Compound Types --------------------------

struct Parameter {
    ts: Type,
    id: Identifier,
}

type ParameterList = Vec<Parameter>;

enum Expression {
    Identifier(Identifier),
    PostOperator {
        exp: Box<Expression>,
        op: Operator,
    },
    PreOperator {
        op: Operator,
        exp: Box<Expression>,
    },
    SubExpression {
        lhs: Box<Expression>,
        op: Operator,
        rhs: Box<Expression>,
    },
}

type Body = Vec<Statement>;

enum Statement {
    Expression(Expression),
    Function {
        t: Type,
        id: Identifier,
        pl: ParameterList,
        b: Body,
    },
    //CondtitionalStatement(ConditionalStatement),
}

#[derive(PartialEq)]
enum Token {
    Type(Type),
    Identifier(Identifier),
    Keyword(Keyword),
    Seperator(Seperator),
    Operator(Operator),
    Literal(Literal),
    Comment(Comment),
    Whitespace(Whitespace),
}

impl Token {
    fn new(input_token: Token, input_string: &str) -> Token {
        let types = Type::get_literals();
        let keywords = Keyword::get_literals();
        let operators = Operator::get_literals();
        let seperators = Seperator::get_literals();

        //Checks the input string to generate a new token of the matching form
        //TODO restructure in some form, messy nested if's & repeated code
        if types.contains(&input_string.to_string()) {
            return Token::Type(Type::new(input_string));
        } else if keywords.contains(&input_string.to_string()) {
            return Token::Keyword(Keyword::new(input_string));
        } else if operators.contains(&input_string.to_string()) {
            return Token::Operator(Operator::new(input_string));
        } else if seperators.contains(&input_string.to_string()) {
            return Token::Seperator(Seperator::new(input_string));
        }

        //match the input token and insert
        match input_token {
            Token::Identifier(_) => Token::Identifier(input_string.to_string()),
            Token::Literal(_) => Token::Literal(input_string.to_string()),
            Token::Comment(_) => Token::Comment(input_string.to_string()),
            Token::Whitespace(_) => Token::Whitespace(input_string.to_string()),
            _ => Token::Whitespace("".to_string()),
        }
    }
}

//read in a file and return it as a string
fn string_from_file(filename: impl AsRef<Path>) -> String {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let out: String = buf.lines().map(|line| line.expect("error")).collect();
    out
}

//TODO make the list of tokens this can read complete. It is currently not
fn tokenise(input: String) -> Vec<Token> {
    let mut output: Vec<Token> = vec![];
    let mut cur_token_string = "".to_string();
    let mut cur_token_type: Token = Token::Whitespace("".to_string());
    for c in input.as_bytes() {
        match c {
            //Whitespace
            // ' ' \n
            32 | 10 => {
                if cur_token_type != Token::Whitespace("".to_string()) {
                    output.push(Token::new(cur_token_type, &cur_token_string));
                }
                cur_token_string = "".to_string();
                cur_token_type = Token::Whitespace("".to_string());
            }
            //Identifier
            //Keyword
            //Seperator
            // { } ( ) ;
            123 | 125 | 40 | 41 | 59 => {
                if cur_token_type != Token::Whitespace("".to_string()) {
                    output.push(Token::new(cur_token_type, &cur_token_string));
                }
                cur_token_type = Token::Seperator(Seperator::Any);
                cur_token_string = (*c as char).to_string();
            }
            //Operator [ ]
            42..=46 | 60..=62 | 91 | 93 => {
                if cur_token_type != Token::Operator(Operator::Any)
                    && cur_token_type != Token::Whitespace("".to_string())
                {
                    output.push(Token::new(cur_token_type, &cur_token_string));
                    cur_token_string = "".to_string();
                }
                cur_token_type = Token::Operator(Operator::Add);
                cur_token_string.push(*c as char);
            }
            //Literal
            //Comment
            //Any alphanumeric value or _ "
            47..=57 | 65..=90 | 97..=122 | 95 | 34 => {
                if cur_token_type != Token::Identifier("".to_string())
                    && cur_token_type != Token::Whitespace("".to_string())
                {
                    output.push(Token::new(cur_token_type, &cur_token_string));
                    cur_token_string = "".to_string();
                }
                cur_token_type = Token::Identifier("".to_string());
                cur_token_string.push(*c as char);
            }
            _ => (), //?
        }
    }
    output.push(Token::new(cur_token_type, &cur_token_string));
    output
}

fn check_pattern(token_string: Vec<Token>, pattern: Vec<Token>) -> bool {
    for i in 0..token_string.len() {
        if token_string[i] != pattern[i] {
            return false;
        }
    }
    true
}

pub fn run_lexer(debug: bool) {
    //read file
    let lines = string_from_file("test_code/input.c");
    //tokenise and lex string
    let mut tokens = tokenise(lines);
    //temp debug switch for printing out the current tokens
    if debug {
        for t in tokens.iter_mut() {
            println!("{}", t.to_string());
        }
    }
    //create AST

    //Bare State
    //Statement

    let mut state = "none";

    let func_pattern = vec![
        Token::Type,
        Token::Identifier,
        Token::Seperator(Seperator::OB),
    ];

    for t in tokens {
        if state == "none" {}
    }
}

//---------------------- Pretty printing functions ---------------------------------

impl Token {
    fn to_string(&self) -> String {
        let mut output = "".to_string();

        match self {
            Token::Type(x) => output += &(x.to_string().to_owned() + " : Type"),
            Token::Identifier(x) => output += &(x.to_owned() + " : Identifier"),
            Token::Keyword(x) => output += &(x.to_string().to_owned() + " : Keyword"),
            Token::Seperator(x) => output += &(x.to_string().to_owned() + " : Seperator"),
            Token::Operator(x) => output += &(x.to_string().to_owned() + " : Operator"),
            Token::Literal(x) => output += &(x.to_owned() + " : Literal"),
            Token::Comment(x) => output += &(x.to_owned() + " : Comment"),
            Token::Whitespace(x) => output += &(x.to_owned() + " : Whitespace"),
        }

        output
    }
}
