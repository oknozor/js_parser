#
A a rust crate for parsing and typing javascript without a name yet.

## Status

Work in progress. 

## Todo 

- [ ] add some tests to the parser
- [ ] add some CI
- [ ] implement the whole [ES5 spec](https://github.com/estree/estree/blob/master/es5.md)
   - [ ] Node objects
   - [ ] Identifier
   - [x] Literal
        - [ ] RegExpLiteral
   - [x] Programs
   - [ ] Functions
   - [ ] Statements
        - [x] ExpressionStatement
        - [ ] BlockStatement
        - [ ] EmptyStatement
        - [ ] DebuggerStatement
        - [ ] WithStatement
        - [ ] Control flow
             - [ ] ReturnStatement
             - [ ] LabeledStatement
             - [ ] BreakStatement
             - [ ] ContinueStatement
        - [ ] Choice
             - [ ] IfStatement
             - [ ] SwitchStatement
               - [ ] SwitchCase   
        - [ ] Exceptions
            - [ ] ThrowStatement
            - [ ] TryStatement
                - [ ] CatchClause
        - [ ] Loops
            - [ ] WhileStatement
            - [ ] DoWhileStatement
            - [ ] ForStatement
            - [ ] ForInStatement
   - [ ] Declarations
        - [ ]  FunctionDeclaration
        - [x]  VariableDeclaration
            - [x] VariableDeclarator
   - [ ] Expressions
        - [ ] ThisExpression
        - [ ] ArrayExpression
        - [ ] ObjectExpression    
            - [ ] Property
        - [ ] FunctionExpression
        - [ ] Unary operations
            UnaryExpression
                UnaryOperator
            UpdateExpression
                UpdateOperator
        - [ ] Binary operations
            - [ ] BinaryExpression
                - [ ] BinaryOperator
            - [x] AssignmentExpression
                - [ ] AssignmentOperator
            - [ ] LogicalExpression
                - [ ] LogicalOperator
            - [ ] MemberExpression
        - [ ] ConditionalExpression
        - [ ] CallExpression
        - [ ] NewExpression
        - [ ] SequenceExpression
    - [ ] Patterns
