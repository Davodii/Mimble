
// Declare submodules
pub mod span;
pub mod symbol;
pub mod diagnostics;

// Re-export commonly used items
pub use span::Span;
pub use symbol::{Symbol, StringPool};
pub use diagnostics::{Diagnostic, DiagnosticsSink, Severity};