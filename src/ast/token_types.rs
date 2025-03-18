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

impl Expression {
    fn render(&self) -> String {
        let mut return_string = String::from("");
        match &(*self.lhs) {
            ResolvableValue::Value(x) => return_string += &x.to_string(),
            ResolvableValue::Variable(x) => return_string += x,
            ResolvableValue::Expression(x) => return_string += &x.render(),
        }

        if let Some(rv) = &self.rhs {
            return_string += match rv.op {
                Operator::ArithmeticOperator => "+",
                _ => "another_one",
            };

            match &(*rv.rhs) {
                ResolvableValue::Value(x) => return_string += &x.to_string(),
                ResolvableValue::Variable(x) => return_string += x,
                ResolvableValue::Expression(x) => return_string += &x.render(),
            }
        }

        return_string
    }
}

struct Func {
    identifier: TypeSpecifier,
    name: String,
    parameters: Vec<String>,
    body: Vec<Expression>,
}

impl Func {
    fn render(&self) -> String {
        let mut return_string = String::from("");

        match &self.identifier {
            TypeSpecifier::Void => return_string.push_str("void"),
            TypeSpecifier::Int => return_string.push_str("int"),
        }

        return_string += " ";

        return_string.push_str(&self.name);

        return_string += "(";

        for p in self.parameters.iter() {}

        return_string += ")\n{\n    ";

        for e in self.body.iter() {
            return_string += &e.render();
            return_string += ";\n";
        }

        return_string += "}";

        return_string
    }
}
