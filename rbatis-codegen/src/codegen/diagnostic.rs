use proc_macro2::Span;

/// Source location of an element or text run in an html/sql template.
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
    /// 1-based column to highlight. `0` selects the first non-whitespace column.
    pub column: usize,
    /// Number of characters to underline. `0` selects a best-effort token width.
    pub length: usize,
}

impl SourceSpan {
    /// `true` when there is no source location to show.
    pub fn is_empty(&self) -> bool {
        self.line == 0 || self.line_text.is_empty()
    }

    /// Highlight the first occurrence of `needle` on this source line.
    ///
    /// Template expressions are normally identified by the parser after the
    /// surrounding text node has been loaded, so this lets the diagnostic point at
    /// the bad expression rather than only at the beginning of the text node.
    pub fn highlighting(mut self, needle: &str) -> Self {
        if needle.is_empty() {
            return self;
        }
        if let Some(byte_index) = self.line_text.find(needle) {
            self.column = self.line_text[..byte_index].chars().count() + 1;
            self.length = needle.chars().count().max(1);
        }
        self
    }

    /// Highlight `needle` inside a possibly multi-line text node.
    ///
    /// A source span initially points at the first non-whitespace line of the text
    /// node. If the expression is on a later line, use that line's text and offset.
    pub fn highlighting_in_text(mut self, text: &str, needle: &str) -> Self {
        if needle.is_empty() {
            return self;
        }
        if self.line_text.contains(needle) {
            return self.highlighting(needle);
        }
        let Some(needle_start) = text.find(needle) else {
            return self;
        };
        let Some(first_non_whitespace) = text
            .char_indices()
            .find_map(|(index, ch)| (!ch.is_whitespace()).then_some(index))
        else {
            return self;
        };

        let first_line = text[..first_non_whitespace].matches('\n').count();
        let needle_line = text[..needle_start].matches('\n').count();
        let line_delta = needle_line.saturating_sub(first_line);
        if line_delta == 0 {
            return self;
        }

        let line_start = text[..needle_start]
            .rfind('\n')
            .map(|index| index + 1)
            .unwrap_or(0);
        let line_end = text[needle_start..]
            .find('\n')
            .map(|index| needle_start + index)
            .unwrap_or(text.len());
        self.line += line_delta;
        self.line_text = text[line_start..line_end]
            .trim_end_matches('\r')
            .to_string();
        self.column = text[line_start..needle_start].chars().count() + 1;
        self.length = needle.chars().count().max(1);
        self
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
        let auto_column = self.line_text[..self.line_text.len() - trimmed.len()]
            .chars()
            .count()
            + 1;
        let column = if self.column == 0 {
            auto_column
        } else {
            self.column
        };
        // Underline the tag name (the first whitespace/`>` after it) unless the
        // diagnostic identified a more precise expression.
        let auto_length = trimmed
            .find(|c: char| c.is_whitespace() || c == '>')
            .map(|byte_index| trimmed[..byte_index].chars().count())
            .unwrap_or_else(|| trimmed.chars().count())
            .max(1);
        let length = if self.length == 0 {
            auto_length
        } else {
            self.length
        };
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
            " ".repeat(column.saturating_sub(1)),
            "^".repeat(length),
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
        // Generated helper nodes may carry an empty source. Keep looking outwards in
        // that case so their user-authored parent can still provide a useful frame.
        if self.source.as_ref().is_none_or(SourceSpan::is_empty) && !source.is_empty() {
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
