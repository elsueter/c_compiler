#[derive(PartialEq)]
enum Type {
    Operator,
    Numeric,
}

//utility struct to hold a 'token' along with an associated id
pub struct Token {
    pub id: Type,
    pub val: Vec<u8>,
}

//taken an infix string and returns a vector of 'tokens' in postfix notation
pub fn to_postfix(input: Vec<u8>) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    let mut cur_type: Type = Type::Operator;
    let mut cur_token: Vec<u8> = vec![];
    for c in input {
        match c {
            42 | 43 | 45 | 47 => {
                if cur_type != Type::Operator {
                    // TODO restructure to remove this if on every iteration
                    if cur_token.len() > 0 {
                        tokens.push(Token {
                            id: cur_type,
                            val: cur_token,
                        });
                    }
                    cur_type = Type::Operator;
                    cur_token = vec![];
                }
                cur_token.push(c);
            }
            _ => {
                if cur_type != Type::Numeric {
                    // TODO restructure to remove this if on every iteration
                    if cur_token.len() > 0 {
                        tokens.push(Token {
                            id: cur_type,
                            val: cur_token,
                        });
                    }
                    cur_type = Type::Numeric;
                    cur_token = vec![];
                }
                cur_token.push(c);
            }
        }
    }
    tokens.push(Token {
        id: cur_type,
        val: cur_token,
    });
    tokens
}
