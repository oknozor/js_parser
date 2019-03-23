use AssignmentOperator::*;

/// A complete program source tree.
#[derive(Debug, Clone)]
pub struct JSProgram<'a> {
    pub body: Vec<JSStatement<'a>>
}

/// Any statement.
#[derive(Debug, Clone)]
pub enum JSStatement<'a> {
    ExpressionStatement(JSExpressionStatement<'a>),
    Declaration(JSDeclaration<'a>),
}

/// An expression statement, i.e., a statement consisting of a single expression.
#[derive(Debug, Clone)]
pub struct JSExpressionStatement<'a> {
    pub expression: JSExpression<'a>,
}

#[derive(Debug, Clone)]
pub enum JSDeclaration<'a> {
    VariableDeclaration(JSVariableDeclaration<'a>)
}

#[derive(Debug, Clone)]
pub struct JSVariableDeclaration<'a> {
    pub declarations: Vec<VariableDeclarator<'a>>,
    pub kind: Kind,
}

#[derive(Debug, Clone)]
pub struct VariableDeclarator<'a> {
    pub id: &'a str,
    pub init: Option<JSExpression<'a>>,
}

#[derive(Debug, Clone)]
pub enum JSExpression<'a> {
    Literal(JSLiteral<'a>),
    AssignmentExpression(JSAssignmentExpression<'a>),
}

/// An assignment operator expression.
#[derive(Debug, Clone)]
pub struct JSAssignmentExpression<'a> {
    pub operator: AssignmentOperator,
    pub left: Box<JSExpression<'a>>,
    pub right: Box<JSExpression<'a>>,
}

/// An assignment operator token.
#[derive(Debug, Clone)]
pub enum AssignmentOperator {
    Eq,
    AddEq,
    SubEq,
    MulEq,
    DivEq,
    ModEq,
}

impl From<&str> for  AssignmentOperator {
    fn from(op: &str) -> Self {
        match op {
            "=" => Eq,
            "+=" => AddEq,
            "-=" => SubEq,
            "*=" => MulEq,
            "/=" => DivEq,
            "%=" => ModEq,
            _=> panic!("Unexpected error")
        }
    }
}

/// A literal token.
#[derive(Debug, Clone)]
pub enum JSLiteral<'a> {
    String(&'a str),
    Boolean(bool),
    Number(JSNumber),
}

/// This is slightly different from the ES5 grammar which does make the difference between floating
/// numbers and integers
#[derive(Debug, Clone)]
pub enum JSNumber {
    Int(i64),
    Float(f64),
}

#[derive(Debug, Clone)]
pub enum Kind {
    Var,
}