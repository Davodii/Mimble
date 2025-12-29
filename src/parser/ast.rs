use crate::lexer::{Token, TokenKind};

#[derive(Debug, Clone)]
pub enum Expr {
    Binary { left: Box<Expr>, op: TokenKind, right: Box<Expr> },
    Unary { op: TokenKind, expr: Box<Expr> },
    Literal(LiteralValue),
    Identifier(Token),
    Assign { name: Box<Expr>, value: Box<Expr> },
    Error,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    ExprStmt(Box<Expr>),
    PrintStmt(Box<Expr>),
    If { cond: Box<Expr>, then: Box<Stmt>, else_branch: Option<Box<Stmt>> },
    While { cond: Box<Expr>, body: Box<Stmt> },
    Block(Vec<Stmt>),
    VarDeclaration { name: Token, var_type: Option<TokenKind>, initializer: Option<Box<Expr>> },
    // TODO: FuncDeclaration { name: Token, params: Vec<(Token, Option<TokenKind>)>, return_type: Option<TokenKind>, body: Box<Stmt> },
    Error,
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    Error,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Box<Stmt>>,
}

impl Expr {
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
                    LiteralValue::Nil => format!("(Nil)"),
                    LiteralValue::Error => format!("(Error)"),
                }
            }
            Expr::Identifier(token) => format!("(Variable {})", token.lexeme),
            Expr::Assign { name, value } =>{
                format!("(Assign {} {})", name.to_test_string(), value.to_test_string())
            },
            Expr::Error => format!("(Error)"),
        }
    }
}

impl Stmt {
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
            Stmt::ExprStmt(expr) => todo!(),
            Stmt::PrintStmt(expr) => todo!(),
            Stmt::If { cond, then, else_branch } => todo!(),
            Stmt::While { cond, body } => todo!(),
            Stmt::Block(stmts) => todo!(),
            Stmt::VarDeclaration { name, var_type, initializer } => todo!(),
            Stmt::Error => format!("(Error)"),
        }
    }
}

impl Program {
    pub fn to_test_string(&self) -> String {
        let mut result = String::new();
        for stmt in &self.statements {
            result.push_str(&format!("{:?}\n", stmt));
        }
        result
    }
}