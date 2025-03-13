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
//
enum TypeSpecifier {
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

enum Operator {
    ArithmeticOperator,
    PrefixOperator,
    PostfixOperator,
}

enum Expression {
    ID(String),
}

struct Declaration {
    c_type: TypeSpecifier,
    name: String,
    assignment: AssignmentOperator,
    rhs: Vec<Expression>,
}

fn test() {
    let mine = Declaration {
        c_type: TypeSpecifier::Void,
        name: "test".to_string(),
        assignment: AssignmentOperator::Equals,
        rhs: vec![],
    };
}
