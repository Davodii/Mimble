use crate::Location;
use crate::lexer::LexerErrorKind;
use crate::parser::ParserErrorKind;
use crate::interpreter::RuntimeError;

#[derive(Debug)]
pub struct MimbleError<T> {
    pub kind: T,
    pub location: Location,
}

impl<T: std::fmt::Debug + 'static> std::fmt::Display for MimbleError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            _ if std::any::TypeId::of::<T>() == std::any::TypeId::of::<LexerErrorKind>() => {
                write!(f, "[Lexer error] at {}: {:?}", self.location, self.kind)
            }
            _ if std::any::TypeId::of::<T>() == std::any::TypeId::of::<ParserErrorKind>() => {
                write!(f, "[Parser error] at {}: {:?}", self.location, self.kind)
            }
            _ if std::any::TypeId::of::<T>() == std::any::TypeId::of::<RuntimeError>() => {
                write!(f, "[Runtime error] at {}: {:?}", self.location, self.kind)
            }
            _ => write!(f, "Error at {}: {:?}", self.location, self.kind),
        }
    }
}

impl<T: std::fmt::Debug + 'static> std::error::Error for MimbleError<T> {}