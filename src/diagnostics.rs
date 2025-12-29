#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

pub struct Diagnostic {
    pub message: String,
    pub location: crate::Location,
    pub severity: Severity,
    // TODO: implement using these fields
    // pub help: Option<String>,
    // pub code_snippet: Option<String>,
}

pub struct DiagnosticsSink {
    diagnostics: Vec<Diagnostic>,
    has_errors: bool,
}

impl DiagnosticsSink {
    pub fn new() -> Self {
        DiagnosticsSink {
            diagnostics: Vec::new(),
            has_errors: false,
        }
    }

    pub fn report(&mut self, diag: Diagnostic) {
        if let Severity::Error = diag.severity {
            self.has_errors = true;
        }

        // Add diagnostic to the list
        self.diagnostics.push(diag);
    }

    pub fn emit(&self) {
        for diag in &self.diagnostics {
            // In the future use a crate like 'miette' or 'codespan-reporting' for better error reporting
            eprintln!("[{}:{}] {:?}: {}",
                diag.location.line,
                diag.location.column,
                diag.severity,
                diag.message
            );
        }
    }

    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    pub fn has_errors(&self) -> bool {
        self.has_errors
    }
}