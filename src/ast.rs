
/// A complete program source tree.
#[derive(Debug)]
pub struct JSProgram<'a> {
    body: Vec<JSStatement<'a>>
}

/// Any statement.
#[derive(Debug)]
pub enum JSStatement<'a> {
    ExpressionStatement(JSExpressionStatement<'a>),
    Declaration(JSDeclaration<'a>),
}

/// An expression statement, i.e., a statement consisting of a single expression.
#[derive(Debug)]
pub struct JSExpressionStatement<'a> {
    pub expression: JSExpression<'a>,
}

#[derive(Debug)]
pub enum JSDeclaration<'a> {
    VariableDeclaration(JSVariableDeclaration<'a>)
}

#[derive(Debug)]
pub struct JSVariableDeclaration<'a> {
    declarations: Vec<VariableDeclarator<'a>>,
    kind: Kind::Var,
}

#[derive(Debug)]
pub struct VariableDeclarator<'a> {
    id: &'a str,
    init: Option<JSExpression<'a>>
}

#[derive(Debug)]
pub enum JSExpression<'a> {
    Literal(JSLiteral<'a>),
    AssignmentExpression(JSAssignementExpression),
}

/// An assignment operator expression.
#[derive(Debug)]
pub struct JSAssignmentExpression<'a> {
    pub operator: JSAssignmentOperator,
    pub left: JSExpression<'a>,
    pub right: JSExpression<'a>,
}

/// An assignment operator token.
#[derive(Debug)]
pub enum AssignmentOperator {
    Eq,
    AddEq,
    SubEq,
    MulEq,
    DivEq,
    ModEq,
}

/// A literal token.
#[derive(Debug)]
pub enum JSLiteral<'a> {
    String(&'a str),
    Boolean(bool),
    Number(JSNumber),
}

/// This is slightly different from the ES5 grammar which does make the difference between floating
/// numbers and integers
#[derive(Debug)]
pub enum JSNumber {
    Int(i64),
    Float(f64),
}

pub enum Kind {
    Var,
}