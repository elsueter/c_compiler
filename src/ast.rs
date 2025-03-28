use std::{
    fmt,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

// Context Free Grammar

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
            fn new(input_string: &str) -> $name {
                match input_string{
                    $( $variant_literal => $name::$variant, )*
                    _ => $name::Any,
                }
            }
            fn get_literals() -> Vec<String>{
                let out = vec![
                $( ($variant_literal.to_string()),)*];
                out
            }
        }
        impl fmt::Display for $name{
            fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
                match self{
                    $( $name::$variant => write!(f, "{}", $variant_literal),) *
                }
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
    Ocb,
    Ccb,
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
    Osb,
    Csb,
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
    Full {
        lhs: Box<Expression>,
        op: Operator,
        rhs: Box<Expression>,
    },
}

type Body = Vec<Statement>;

enum Statement {
    Expression(Expression),
    FunctionProto {
        t: Type,
        id: Identifier,
        pl: ParameterList,
    },
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
    Any,
}

impl Token {
    fn new(input_type: Token, input_string: &str) -> Token {
        let types = Type::get_literals();
        let keywords = Keyword::get_literals();
        let operators = Operator::get_literals();
        let seperators = Seperator::get_literals();

        match input_type {
            Token::Whitespace(_) => Token::Whitespace(input_string.to_string()),
            _ => {
                //Checks the input string to generate a new token of the matching form
                //TODO restructure in some form, messy nested if's & repeated code while also being slow
                if types.contains(&input_string.to_string()) {
                    return Token::Type(Type::new(input_string));
                } else if keywords.contains(&input_string.to_string()) {
                    return Token::Keyword(Keyword::new(input_string));
                } else if operators.contains(&input_string.to_string()) {
                    return Token::Operator(Operator::new(input_string));
                } else if seperators.contains(&input_string.to_string()) {
                    return Token::Seperator(Seperator::new(input_string));
                } else if input_string.starts_with("//") {
                    return Token::Comment(input_string.to_string());
                //This clause is not perfect and will need improving TODO
                //.unwrap() is potentially unsafe and parsing u64 is slow
                } else if input_string.starts_with('"') || input_string.parse::<f64>().is_ok() {
                    return Token::Literal(input_string.to_string());
                }

                Token::Identifier(input_string.to_string())
            }
        }
    }

    //TODO
    //I really really dislike how this function currently works.
    //In order to fetch a value from an enum you need to make use of pattern matching
    //and due to the setup here each enum holds a different type so you cannot make a
    //a generic getter or do any kind of structural check
    //
    //you can make use of std::mem::discriminant(x) to get the type of enum back out
    //but this does not return the value but rather the raw reprisentation of the
    //variant of the enum.
    //
    //After a lot of faffing I could not figure out a good way to resolve this so resolved
    //to making this. It is a lot of code, very slow and not nice.
    //
    //Due to this it may be worth not using enums at all for this structure, the benefits
    //of using enums here is it allows for some more neuonced type usage within the
    //code structure but it leads to headaches like this and some of the other matching
    //(see function above...)
    fn matches(&self, other: &Token) -> bool {
        match (self, other) {
            (Token::Type(x), Token::Type(y)) => {
                if x == y || *x == Type::Any || *y == Type::Any {
                    return true;
                }
            }
            (Token::Identifier(x), Token::Identifier(y)) => {
                if x == y || x.is_empty() || y.is_empty() {
                    return true;
                }
            }
            (Token::Keyword(x), Token::Keyword(y)) => {
                if x == y || *x == Keyword::Any || *y == Keyword::Any {
                    return true;
                }
            }
            (Token::Seperator(x), Token::Seperator(y)) => {
                if x == y {
                    return true;
                }
            }
            (Token::Operator(x), Token::Operator(y)) => {
                if x == y || *x == Operator::Any || *y == Operator::Any {
                    return true;
                }
            }
            (Token::Literal(x), Token::Literal(y)) => {
                if x == y || x.is_empty() || y.is_empty() {
                    return true;
                }
            }
            (Token::Comment(x), Token::Comment(y)) => {
                if x == y || x.is_empty() || y.is_empty() {
                    return true;
                }
            }
            (Token::Whitespace(x), Token::Whitespace(y)) => {
                if x == y || x.is_empty() || y.is_empty() {
                    return true;
                }
            }
            _ => (),
        }

        false
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
//TODO lots of .to_string() methods being used here, might need looking at
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
            //Identifier
            //Keyword
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

fn check_pattern(token_string: &[Token], idx: usize, pattern: &[Token]) -> (bool, usize) {
    let mut repeat = false;
    let mut b_depth = 0;
    let mut j = 0;
    let mut i = 0;
    while i < pattern.len() && j + idx < token_string.len() {
        if pattern[i] == Token::Any {
            repeat = true;
            i += 1;
            continue;
        }

        if let Token::Seperator(x) = &token_string[idx + j] {
            match x {
                Seperator::OB | Seperator::Ocb => b_depth += 1,
                Seperator::CB | Seperator::Ccb => b_depth -= 1,
                _ => (),
            }
        }

        if repeat {
            if token_string[idx + j].matches(&pattern[i]) && b_depth == 0 {
                repeat = false;
                i += 1;
            }
            j += 1;
            continue;
        } else if !token_string[idx + j].matches(&pattern[i]) {
            return (false, i - 2);
        }
        i += 1;
        j += 1;
    }

    //check for if end of file is found before matching a repeating token
    if repeat {
        (false, j - 1)
    } else {
        (true, j - 1)
    }
}

fn run_parser(input: Vec<Token>) -> Vec<Statement> {
    let func_pattern: Vec<Token> = vec![
        Token::Type(Type::Any),
        Token::Identifier("".to_string()),
        Token::Seperator(Seperator::OB),
        Token::Any,
        Token::Seperator(Seperator::CB),
        Token::Seperator(Seperator::Ocb),
        Token::Any,
        Token::Seperator(Seperator::Ccb),
    ];

    let mut output: Vec<Statement> = vec![];

    let mut i = 0;
    while i < input.len() {
        let (in_func, idx) = check_pattern(&input, i, &func_pattern);
        i += idx;
        if in_func {
            println!("in_function");
        } else {
            println!("not in function");
        }
        i += 1;
    }

    output
}

pub fn run_lexer(debug: bool) {
    //read file
    let lines = string_from_file("test_code/input.c");
    //tokenise and lex string
    let mut tokens = tokenise(lines);
    //temp debug switch for printing out the current tokens
    if debug {
        for t in tokens.iter_mut() {
            println!("{}", t);
        }
    }
    //create AST
    let ast = run_parser(tokens);
}

//---------------------- Print formatting ---------------------------------

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Type(x) => write!(f, "{} : Type", x),
            Token::Identifier(x) => write!(f, "{} : Identifier", x),
            Token::Keyword(x) => write!(f, "{} : Keyword", x),
            Token::Seperator(x) => write!(f, "{} : Seperator", x),
            Token::Operator(x) => write!(f, "{} : Operator", x),
            Token::Literal(x) => write!(f, "{} : Literal", x),
            Token::Comment(x) => write!(f, "{} : Comment", x),
            Token::Whitespace(x) => write!(f, "{} : Whitespace", x),
            Token::Any => write!(f, ""),
        }
    }
}
