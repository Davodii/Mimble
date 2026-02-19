
// Declare submodules
pub mod span;
pub mod symbol;
pub mod diagnostics;
pub mod types;
pub mod context;

// Re-export commonly used items
pub use span::Span;
pub use symbol::{Symbol, SymbolPool};
pub use diagnostics::{DiagnosticsSink, Severity};
pub use types::Type;