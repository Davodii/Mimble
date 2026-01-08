#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

pub struct Diagnostic {
    pub message: String,
    pub span: crate::common::Span,
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

    pub fn report(&mut self, span: crate::common::Span, msg: impl Into<String>, severity: Severity) {
        if let Severity::Error = severity {
            self.has_errors = true;
        }

        // Add diagnostic to the list
        self.diagnostics.push(Diagnostic {
            message: msg.into(),
            span,
            severity,
        });
    }

    pub fn emit_all(&self, source: &str) {
        for diag in &self.diagnostics {
            // In the future use a crate like 'miette' or 'codespan-reporting' for better error reporting
            
            eprintln!("{}", self.format_diagnostic(diag, source));
        }
    }

    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    pub fn format_diagnostic(&self, diag: &Diagnostic, source: &str) -> String {
        let line_num = source[..diag.span.start].lines().count();
        let line_text = source.lines().nth(line_num - 1).unwrap_or("");

        let col = diag.span.column.saturating_sub(1);

        // Create the "pointer" line
        let padding = " ".repeat(col);

        format!(
            "Error on line {line_num}\n {line_text}\n {padding}^--- {msg}",
            line_num = line_num,
            line_text = line_text,
            padding = padding,
            msg = diag.message,
        )
    }

    pub fn has_errors(&self) -> bool {
        self.has_errors
    }
}