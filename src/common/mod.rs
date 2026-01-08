
// Declare submodules
pub mod span;
pub mod symbol;
pub mod diagnostics;
pub mod types;

// Re-export commonly used items
pub use span::Span;
pub use symbol::{Symbol, StringPool};
pub use diagnostics::{DiagnosticsSink, Severity};
pub use types::Type;