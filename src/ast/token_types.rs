// Grammar
//
// Example Code:
//  int main(argc, argv){
//      int foo = 0;
//      if(foo != 10){
//          return 0;
//      }
//      return 1;
//  }
//
// type_specifieer
//  : int
//  | void
//  | ... TODO
//
//  IDENTIFIER
//  : [A-Za-z0-9_.]*
//
// assignment_operator
//  : =
//
// arithmetic_operator
//  : +
//  | -
//  | ... TODO
//
// prefix_operator
//  : +
//  | -
//  | ++
//  | ... TODO
//
// postfix_operator
//  : +
//  | -
//  | ++
//  | --
//  | ... TODO
//
// operator
//  : arithmetic_operator
//  | ... TODO
//
// expression
//  : identifier
//  | expression operator expression
//  | prefix_operator expression
//  | expression postfix_operator
//
// declaration
//  : type_specifier IDENTIFIER assignment_operator expression
//
// body
//  : { expression* }
//
// conditional_statement
//  : if ( expression ) body
//
// return_statement
//  : return expression

pub enum TypeSpecifier {
    Void,
    Int,
}

enum AssignmentOperator {
    Equals,
}

enum ArithmeticOperator {
    Plus,
    Minus,
}

enum PrefixOperator {
    UnaryPrefixPlus,
    UnaryPrefixMinus,
}

enum PostfixOperator {
    UnaryPostfixPlus,
    UnaryPostfixMinus,
}

pub enum Operator {
    ArithmeticOperator,
    PrefixOperator,
    PostfixOperator,
}

pub enum Seperator {
    CurlyBraces,
    Braces,
    SemiColon,
}

pub enum Keyword {
    Type(TypeSpecifier),
    Auto,
    Break,
    // https://en.cppreference.com/w/c/keyword
}

enum ResolvableValue {
    Value(i32),
    Variable(String),
    Expression(Expression),
}

//TODO - remove box usage, box is heap allocated and is very slow
pub struct Expression {
    lhs: Box<ResolvableValue>,
    rhs: Option<RExpression>,
}

//TODO - remove box usage, box is heap allocated and is very slow
struct RExpression {
    op: Operator,
    rhs: Box<ResolvableValue>,
}

enum Statement {
    Exp(Expression),
}

pub struct Func {
    //storage_class_specifier   TODO implement
    //function specifier        TODO implement
    ret_type: TypeSpecifier,
    identifier: String,
    parameters: Vec<String>,
    body: Vec<Statement>,
}

pub enum Structure {
    Func(Func),
}
