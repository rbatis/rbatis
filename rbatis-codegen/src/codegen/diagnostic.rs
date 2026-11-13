use proc_macro2::Span;

/// Source location of an element in an html/sql template.
///
/// Parsed elements own one of these; AST nodes do not store source fields. The
/// parser attaches it when a node returns a [`Diagnostic`], so every node gets the
/// same code-frame output.
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct SourceSpan {
    /// 1-based line, `0` when unknown (generated elements, inline `py_sql` output).
    pub line: usize,
    /// The source text of that line.
    pub line_text: String,
}

impl SourceSpan {
    /// `true` when there is no source location to show.
    pub fn is_empty(&self) -> bool {
        self.line == 0 || self.line_text.is_empty()
    }

    /// Render a compiler-style snippet:
    ///
    /// ```text
    ///   --> template:74:13
    ///    |
    /// 74 |             <if >
    ///    |             ^^^ add test="..."
    /// ```
    ///
    /// Rust's proc-macro API cannot point a [`Span`] into a non-Rust file (the
    /// sql/html template), so the snippet is part of the error message.
    pub fn code_frame(&self, hint: &str) -> String {
        if self.is_empty() {
            return String::new();
        }
        let line_no = self.line.to_string();
        let gutter = " ".repeat(line_no.len());
        let trimmed = self.line_text.trim_start();
        let indent = self.line_text.len() - trimmed.len();
        let column = indent + 1;
        // underline the tag name (the first whitespace/`>` after it)
        let tag_len = trimmed
            .find(|c: char| c.is_whitespace() || c == '>')
            .unwrap_or(trimmed.len())
            .max(1);
        let hint = if hint.is_empty() {
            String::new()
        } else {
            format!(" {}", hint)
        };
        format!(
            "\n  --> template:{}:{}\n{} |\n{} | {}\n{} | {}{}{}",
            line_no,
            column,
            gutter,
            line_no,
            self.line_text,
            gutter,
            " ".repeat(indent),
            "^".repeat(tag_len),
            hint
        )
    }
}

/// A macro diagnostic before it is attached to a source line.
///
/// `HtmlAstNode::try_*` returns this type. `parse_elements` attaches the
/// [`SourceSpan`] of the element the node came from, which is what lets every node
/// share the same code-frame rendering without a `line`/`line_text` field on each
/// node struct.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    /// The main error text.
    pub message: String,
    /// Short hint shown after the caret.
    pub hint: String,
    /// Span of the Rust item (usually the attribute literal).
    pub span: Span,
    /// Source line of the template element; set once by the parser.
    pub source: Option<SourceSpan>,
}

impl Diagnostic {
    /// Create a diagnostic with a message and a caret hint.
    pub fn new(span: Span, message: impl Into<String>, hint: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            hint: hint.into(),
            span,
            source: None,
        }
    }

    /// Wrap an existing [`syn::Error`] and add a caret hint.
    pub fn from_error(error: syn::Error, hint: impl Into<String>) -> Self {
        Self {
            message: error.to_string(),
            hint: hint.into(),
            span: error.span(),
            source: None,
        }
    }

    /// Attach `source` unless a nested node already attached its own (the innermost
    /// element wins, which is the one the user needs to fix).
    pub fn with_source(mut self, source: SourceSpan) -> Self {
        if self.source.is_none() {
            self.source = Some(source);
        }
        self
    }

    /// Render the final [`syn::Error`] with the code frame.
    pub fn to_syn(self) -> syn::Error {
        let source = self.source.unwrap_or_default();
        let mut message = self.message;
        if source.is_empty() {
            if !self.hint.is_empty() {
                message.push_str("; ");
                message.push_str(&self.hint);
            }
        } else {
            message.push_str(&source.code_frame(&self.hint));
        }
        syn::Error::new(self.span, message)
    }
}

impl From<syn::Error> for Diagnostic {
    fn from(error: syn::Error) -> Self {
        Diagnostic::from_error(error, "")
    }
}
