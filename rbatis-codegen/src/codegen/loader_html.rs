use crate::codegen::diagnostic::SourceSpan;
use crate::error::Error;
use scraper::{Html, Node};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Element {
    pub tag: String,
    pub data: String,
    pub attrs: HashMap<String, String>,
    pub childs: Vec<Element>,
    /// Source location of the start tag. [`SourceSpan::default()`] for text nodes and
    /// for elements created by code generation, where no source line exists.
    pub source: SourceSpan,
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.tag.as_str() {
            "" => {
                f.write_str(&self.data)?;
            }
            _ => {
                f.write_str("<")?;
                f.write_str(&self.tag)?;
                for (k, v) in &self.attrs {
                    f.write_str(" ")?;
                    f.write_str(k)?;
                    f.write_str("=\"")?;
                    // an attribute value that carries a `"` would end the attribute
                    // and be read back as a truncated one
                    f.write_str(&crate::codegen::string_util::escape_attr_value(v))?;
                    f.write_str("\"")?;
                }
                f.write_str(">")?;
                for x in &self.childs {
                    std::fmt::Display::fmt(x, f)?;
                }
                //</if>
                f.write_str("</")?;
                f.write_str(&self.tag)?;
                f.write_str(">")?;
            }
        }
        Ok(())
    }
}

/// Start-tag lines collected from the raw template, consumed in document order while
/// the scraper tree is walked.
type TagLines = std::vec::IntoIter<(String, usize, String)>;

fn collect_elements(
    el: scraper::ElementRef<'_>,
    lines: &mut std::iter::Peekable<TagLines>,
    parent_source: &SourceSpan,
) -> Vec<Element> {
    let mut els = vec![];
    for node in el.children() {
        match node.value() {
            Node::Text(text) => {
                let txt = text.to_string();
                if txt.trim().is_empty() {
                    continue;
                }
                // a text node has no start tag of its own: use the source of the
                // enclosing element as a best-effort source position
                els.push(Element {
                    tag: String::new(),
                    data: txt,
                    attrs: HashMap::new(),
                    childs: vec![],
                    source: parent_source.clone(),
                });
            }
            Node::Element(elem) => {
                if let Some(child_ref) = scraper::ElementRef::wrap(node) {
                    let tag = elem.name().to_string();
                    // consume the position before the children: start tags are in
                    // document order and come before their children
                    let (line, line_text) = take_line_for_tag(&tag, lines);
                    let source = SourceSpan { line, line_text };
                    let mut attrs = HashMap::new();
                    for (k, v) in elem.attrs() {
                        attrs.insert(k.to_string(), v.to_string());
                    }
                    let childs = collect_elements(child_ref, lines, &source);
                    els.push(Element {
                        tag,
                        data: String::new(),
                        attrs,
                        childs,
                        source,
                    });
                }
            }
            Node::Comment(_) => {}
            _ => {}
        }
    }
    els
}

/// Take the line and source text of the next scanned start tag with `tag`, skipping
/// entries the scraper normalized away. Returns `(0, "")` when the tag has no source
/// position.
fn take_line_for_tag(
    tag: &str,
    lines: &mut std::iter::Peekable<TagLines>,
) -> (usize, String) {
    while let Some((scanned_tag, line, line_text)) = lines.next() {
        if scanned_tag.eq_ignore_ascii_case(tag) {
            return (line, line_text);
        }
    }
    (0, String::new())
}

/// Find the 1-based line and the line text of every start tag in `html`, in document
/// order.
///
/// `scraper`/`html5ever` do not expose source positions, so this small scanner walks
/// the template itself. It is only used for diagnostics: the tag names are matched
/// against the parsed tree to attach a source position to each [`Element`].
fn scan_start_tag_lines(html: &str) -> Vec<(String, usize, String)> {
    let bytes = html.as_bytes();
    let mut result = Vec::new();
    let mut i = 0usize;
    let mut line = 1usize;

    while i < bytes.len() {
        if bytes[i] == b'\n' {
            line += 1;
            i += 1;
            continue;
        }
        if bytes[i] != b'<' {
            i += 1;
            continue;
        }

        // <!-- comment -->
        if html[i..].starts_with("<!--") {
            if let Some(end) = html[i + 4..].find("-->") {
                let end = i + 4 + end + 3;
                line += count_newlines(&bytes[i..end]);
                i = end;
                continue;
            }
        }

        let next = match bytes.get(i + 1).copied() {
            Some(next) => next,
            None => break,
        };
        if next == b'/' || next == b'!' || next == b'?' {
            // closing tag or declaration
            if let Some(end) = find_tag_end(bytes, i + 1) {
                line += count_newlines(&bytes[i..=end]);
                i = end + 1;
            } else {
                i += 1;
            }
            continue;
        }
        if !next.is_ascii_alphabetic() {
            i += 1;
            continue;
        }

        let start = i + 1;
        let mut j = start;
        while j < bytes.len()
            && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'-' || bytes[j] == b':')
        {
            j += 1;
        }
        let line_start = html[..i].rfind('\n').map(|p| p + 1).unwrap_or(0);
        let line_end = html[i..].find('\n').map(|p| i + p).unwrap_or(html.len());
        let line_text = html[line_start..line_end]
            .trim_end_matches('\r')
            .to_string();
        result.push((html[start..j].to_ascii_lowercase(), line, line_text));

        if let Some(end) = find_tag_end(bytes, j) {
            line += count_newlines(&bytes[i..=end]);
            i = end + 1;
        } else {
            i = j;
        }
    }
    result
}

fn count_newlines(bytes: &[u8]) -> usize {
    bytes.iter().filter(|b| **b == b'\n').count()
}

/// Byte index of the `>` that closes the tag, ignoring `>` inside quoted attributes.
fn find_tag_end(bytes: &[u8], start: usize) -> Option<usize> {
    let mut quote = 0u8;
    let mut i = start;
    while i < bytes.len() {
        let b = bytes[i];
        if quote == 0 {
            match b {
                b'"' | b'\'' => quote = b,
                b'>' => return Some(i),
                _ => {}
            }
        } else if b == quote {
            quote = 0;
        }
        i += 1;
    }
    None
}

/// Loads HTML and records the source line of every element.
pub fn load_html(html: &str) -> std::result::Result<Vec<Element>, String> {
    load_html_with_source(html, true)
}

/// Loads HTML. With `track_source == false` every element gets an empty
/// [`SourceSpan`], which is what generated templates (py_sql -> html) need: their
/// line numbers do not map back to the user's template.
pub fn load_html_with_source(
    html: &str,
    track_source: bool,
) -> std::result::Result<Vec<Element>, String> {
    let document = Html::parse_fragment(html);
    let mut lines = if track_source {
        scan_start_tag_lines(html).into_iter().peekable()
    } else {
        Vec::new().into_iter().peekable()
    };
    Ok(collect_elements(
        document.root_element(),
        &mut lines,
        &SourceSpan::default(),
    ))
}

impl Element {
    /// get all strings
    pub fn child_strings(&self) -> Vec<&str> {
        let mut elements = vec![];
        for x in &self.childs {
            if x.tag.is_empty() {
                elements.push(x.data.as_str());
            }
            let v = x.child_strings();
            for x in v {
                elements.push(x);
            }
        }
        elements
    }
    /// get all strings
    pub fn child_string_cup(&self) -> usize {
        let mut u = 0;
        for x in self.child_strings() {
            u += x.len();
        }
        u
    }
}

/// Loads HTML content into a vector of elements
pub fn load_mapper_vec(html: &str) -> std::result::Result<Vec<Element>, Error> {
    load_mapper_vec_with_source(html, true)
}

/// Loads HTML content into a vector of elements, optionally recording source lines.
pub fn load_mapper_vec_with_source(
    html: &str,
    track_source: bool,
) -> std::result::Result<Vec<Element>, Error> {
    let elements = load_html_with_source(html, track_source).map_err(Error::from)?;

    let mut mappers = Vec::new();
    for element in elements {
        if element.tag == "mapper" {
            mappers.extend(element.childs);
        } else {
            mappers.push(element);
        }
    }

    Ok(mappers)
}
