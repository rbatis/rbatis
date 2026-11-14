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
    /// Source location of the start tag or text run. [`SourceSpan::default()`] for
    /// elements created by code generation, where no source line exists.
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

/// Source positions collected from the raw template. Start tags and text runs are
/// kept separately because the scraper tree exposes them as different node kinds,
/// but both are consumed in document order while the tree is walked.
#[derive(Default)]
struct ScannedSources {
    tags: Vec<(String, SourceSpan)>,
    text: Vec<SourceSpan>,
}

type TagLines = std::vec::IntoIter<(String, SourceSpan)>;
type TextLines = std::vec::IntoIter<SourceSpan>;

fn collect_elements(
    el: scraper::ElementRef<'_>,
    tag_lines: &mut std::iter::Peekable<TagLines>,
    text_lines: &mut std::iter::Peekable<TextLines>,
) -> Vec<Element> {
    let mut els = vec![];
    for node in el.children() {
        match node.value() {
            Node::Text(text) => {
                let txt = text.to_string();
                if txt.trim().is_empty() {
                    continue;
                }
                // Text nodes have no start tag. Use the source position of the text
                // run itself instead of inheriting the parent element's line.
                els.push(Element {
                    tag: String::new(),
                    data: txt,
                    attrs: HashMap::new(),
                    childs: vec![],
                    source: text_lines.next().unwrap_or_default(),
                });
            }
            Node::Element(elem) => {
                if let Some(child_ref) = scraper::ElementRef::wrap(node) {
                    let tag = elem.name().to_string();
                    // consume the position before the children: start tags are in
                    // document order and come before their children
                    let source = take_source_for_tag(&tag, tag_lines);
                    let mut attrs = HashMap::new();
                    for (k, v) in elem.attrs() {
                        attrs.insert(k.to_string(), v.to_string());
                    }
                    let childs = collect_elements(child_ref, tag_lines, text_lines);
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

/// Take the source position of the next scanned start tag with `tag`, skipping
/// entries the scraper normalized away. Returns an empty source when the tag has no
/// source position.
fn take_source_for_tag(tag: &str, lines: &mut std::iter::Peekable<TagLines>) -> SourceSpan {
    for (scanned_tag, source) in lines.by_ref() {
        if scanned_tag.eq_ignore_ascii_case(tag) {
            return source;
        }
    }
    SourceSpan::default()
}

/// Find the source positions of start tags and non-whitespace text runs in `html`, in
/// document order.
///
/// `scraper`/`html5ever` do not expose source positions, so this small scanner walks
/// the template itself. It is only used for diagnostics: the positions are matched
/// against the parsed tree to attach a source location to each [`Element`].
fn scan_source_positions(html: &str) -> ScannedSources {
    let bytes = html.as_bytes();
    let mut result = ScannedSources::default();
    let mut i = 0usize;
    let mut line = 1usize;

    while i < bytes.len() {
        if bytes[i] != b'<' || !is_markup_start(html, i) {
            let end = next_markup_start(html, i + 1).unwrap_or(bytes.len());
            if let Some(source) = first_text_source(html, i, end, line) {
                result.text.push(source);
            }
            line += count_newlines(&bytes[i..end]);
            i = end;
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
        result.tags.push((
            html[start..j].to_ascii_lowercase(),
            SourceSpan {
                line,
                line_text,
                ..SourceSpan::default()
            },
        ));

        if let Some(end) = find_tag_end(bytes, j) {
            line += count_newlines(&bytes[i..=end]);
            i = end + 1;
        } else {
            i = j;
        }
    }
    result
}

/// `true` when the `<` at `index` begins markup that the scanner should skip while
/// looking for text. SQL comparisons such as `a < #{b}` must remain text.
fn is_markup_start(html: &str, index: usize) -> bool {
    let rest = &html[index..];
    if rest.starts_with("<!--") {
        return true;
    }
    match rest.as_bytes().get(1).copied() {
        Some(b'/') | Some(b'!') | Some(b'?') => true,
        Some(next) => next.is_ascii_alphabetic(),
        None => false,
    }
}

/// Index of the next real markup start at or after `from`.
fn next_markup_start(html: &str, from: usize) -> Option<usize> {
    let mut offset = from;
    while let Some(relative) = html[offset..].find('<') {
        let index = offset + relative;
        if is_markup_start(html, index) {
            return Some(index);
        }
        offset = index + 1;
    }
    None
}

/// Source position of the first non-whitespace character in a text run.
fn first_text_source(
    html: &str,
    start: usize,
    end: usize,
    start_line: usize,
) -> Option<SourceSpan> {
    let relative = html[start..end]
        .char_indices()
        .find_map(|(index, ch)| (!ch.is_whitespace()).then_some(index))?;
    let position = start + relative;
    let line = start_line + count_newlines(&html.as_bytes()[start..position]);
    let line_start = html[..position].rfind('\n').map(|p| p + 1).unwrap_or(0);
    let line_end = html[position..]
        .find('\n')
        .map(|p| position + p)
        .unwrap_or(html.len());
    Some(SourceSpan {
        line,
        line_text: html[line_start..line_end]
            .trim_end_matches('\r')
            .to_string(),
        ..SourceSpan::default()
    })
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

/// Loads HTML and records the source line of every element and non-whitespace text
/// run.
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
    let sources = if track_source {
        scan_source_positions(html)
    } else {
        ScannedSources::default()
    };
    let mut tag_lines = sources.tags.into_iter().peekable();
    let mut text_lines = sources.text.into_iter().peekable();
    Ok(collect_elements(
        document.root_element(),
        &mut tag_lines,
        &mut text_lines,
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
