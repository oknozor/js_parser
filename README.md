#
A a rust crate for parsing and typing javascript without a name yet.

### Examples : 

To get a string representation of the output AST run  one of the examples javascript file,
or try with your own code : 

``` 
cargo run --bin parse src/bin/variable_declaration.js
```

## Status

Work in progress. 

## Todo 

- [ ] Add some tests to the parser
- [ ] Add some CI
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
