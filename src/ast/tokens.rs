pub enum Arithmetic {
    Addition,
    Subtraction,
    UPlus,  //unary plus
    UMinus, //unary minus
    Multiplication,
    Division,
    Modulo,
    Increment,
    Decriment,
}

enum UnaryOperator {
    LNOT, //logical not
    BNOT, //binary not
    ADDRESS,
    INDIRECTION,
    Sizeof,
}

enum Operator {
    Arithmetic,
    UnaryOperator,
}

pub enum Token {
    Operator,
    Numeric,
}
