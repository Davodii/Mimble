use crate::lexer::{Token, TokenKind};

#[derive(Debug, Clone)]
pub enum Expr {
    Binary { left: Box<Expr>, op: TokenKind, right: Box<Expr> },
    Unary { op: TokenKind, expr: Box<Expr> },
    Literal(LiteralValue),
    Variable(Token),
    Assign { name: Box<Expr>, value: Box<Expr> },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    ExprStmt(Expr),
    PrintStmt(Expr),
    If { cond: Expr, then: Box<Stmt>, else_branch: Option<Box<Stmt>> },
    While { cond: Expr, body: Box<Stmt> },
    Block(Vec<Stmt>),
    VarDeclaration { name: Token, var_type: Option<TokenKind>, initializer: Option<Expr> },
    // TODO: FuncDeclaration { name: Token, params: Vec<(Token, Option<TokenKind>)>, return_type: Option<TokenKind>, body: Box<Stmt> },
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    // Nil,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

impl Expr {
    pub fn pretty_print(&self, indent: usize) {
        let pad = "  ".repeat(indent);

        match self {
            Expr::Binary { left, op, right } => {
                let operation = match op {
                    TokenKind::Plus => "+",
                    TokenKind::Minus => "-",
                    TokenKind::Star => "*",
                    TokenKind::Slash => "/",
                    TokenKind::Modulus => "%",
                    TokenKind::EQ => "==",
                    TokenKind::NEQ => "!=",
                    TokenKind::LT => "<",
                    TokenKind::LEQ => "<=",
                    TokenKind::GT => ">",
                    TokenKind::GEQ => ">=",
                    _ => "ERROR TOKEN",
                };
                println!("{pad}Binary Operation:");
                left.pretty_print(indent + 1);
                println!("{pad}  {operation}");
                right.pretty_print(indent + 1);
            },
            Expr::Unary { op, expr } => {
                println!("{pad}Unary Operation:");
                match op {
                    TokenKind::Plus => println!("{pad}  +"),
                    TokenKind::Minus => println!("{pad} -"),
                    _ => println!("ERROR TOKEN"),
                };
                expr.pretty_print(indent + 1);
            },
            Expr::Literal(literal_value) => {
                match literal_value {
                    LiteralValue::Number(val) => println!("{pad}Number: {val}"),
                    LiteralValue::String(val) => println!("{pad}String: \"{val}\""),
                    LiteralValue::Boolean(val) => println!("{pad}Boolean: {val}"),
                };
            }
            Expr::Variable(token) => println!("{pad}Identifier: {}", token.lexeme),
            Expr::Assign { name, value } =>{
                print!("{pad}Assign:");
                name.pretty_print(indent + 1);
                value.pretty_print(indent + 1);
            },
        }
    }

    pub fn to_test_string(&self) -> String {
        match self {
            Expr::Binary { left, op, right } => {
                format!("(Binary {} {:?} {})", left.to_test_string(), op, right.to_test_string())
            },
            Expr::Unary { op, expr } => {
                format!("(Unary {:?} {})", op, expr.to_test_string())
            },
            Expr::Literal(literal_value) => {
                match literal_value {
                    LiteralValue::Number(val) => format!("(Number {})", val),
                    LiteralValue::String(val) => format!("(String \"{}\")", val),
                    LiteralValue::Boolean(val) => format!("(Boolean {})", val),
                }
            }
            Expr::Variable(token) => format!("(Variable {})", token.lexeme),
            Expr::Assign { name, value } =>{
                format!("(Assign {} {})", name.to_test_string(), value.to_test_string())
            },
        }
    }
}

impl Stmt {
    pub fn pretty_print(&self, indent: usize) {
        let pad = "  ".repeat(indent);

        match self {
            Self::ExprStmt(expr) => {
                println!("{pad}Expression Stmt:");
                expr.pretty_print(indent + 1);
            },
            Self::PrintStmt(expr) => {
                println!("{pad}Print:");
                expr.pretty_print(indent + 1);
            },
            Self::If { cond, then, else_branch } => {
                println!("{pad}If Statement:");
                println!("{pad}  Condition:");
                cond.pretty_print(indent + 2);
                println!("{pad}  Then:");
                then.pretty_print(indent + 2);
                if let Some(other) = else_branch {
                    println!("{pad}  Else:");
                    other.pretty_print(indent + 2);

                }
            },
            Self::While { cond, body } => {
                println!("{pad}While:");
                println!("{pad}  Cond:");
                cond.pretty_print(indent + 2);
                body.pretty_print(indent + 1);
            },
            Self::Block(stmts) => {
                println!("{pad}Block:");
                for stmt in stmts {
                    stmt.pretty_print(indent + 1);
                }
            },
            Self::VarDeclaration { name, var_type, initializer } => {
                println!("{pad}Var Declaration:");
                println!("{pad}  Name: {}", name.lexeme);
                if let Some(vt) = var_type {
                    println!("{pad}  Type: {:?}", vt);
                } else {
                    println!("{pad}  Type: Inferred");
                }
                if let Some(init) = initializer {
                    println!("{pad}  Initializer:");
                    init.pretty_print(indent + 2);
                }
            },
        }
    }

    pub fn to_test_string(&self) -> String {
        match self {
            Self::ExprStmt(expr) => {
                format!("(ExprStmt {})", expr.to_test_string())
            },
            Self::PrintStmt(expr) => {
                format!("(PrintStmt {})", expr.to_test_string())
            },
            Self::If { cond, then, else_branch } => {
                if let Some(other) = else_branch {
                    format!("(If {} {} {})", cond.to_test_string(), then.to_test_string(), other.to_test_string())
                } else {
                    format!("(If {} {})", cond.to_test_string(), then.to_test_string())
                }
            },
            Self::While { cond, body } => {
                format!("(While {} {})", cond.to_test_string(), body.to_test_string())
            },
            Self::Block(stmts) => {
                let mut result = String::from("(Block");
                for stmt in stmts {
                    result.push_str(&format!(" {}", stmt.to_test_string()));
                }
                result.push(')');
                result
            },
            Self::VarDeclaration { name, var_type, initializer } => {
                let var_type_str = if let Some(vt) = var_type {
                    format!("{:?}", vt)
                } else {
                    "Inferred".to_string()
                };
                if let Some(init) = initializer {
                    format!("(VarDeclaration {} {} {})", name.lexeme, var_type_str, init.to_test_string())
                } else {
                    format!("(VarDeclaration {} {} )", name.lexeme, var_type_str)
                }
            },
        }
    }
}

impl Program {
    pub fn pretty_print(&self) {
        println!("Program:");
        for stmt in &self.statements {
            stmt.pretty_print(1);
        }
    }

    pub fn to_test_string(&self) -> String {
        let mut result = String::new();
        for stmt in &self.statements {
            result.push_str(&format!("{:?}\n", stmt));
        }
        result
    }
}