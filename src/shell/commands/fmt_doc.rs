//! Document-family conversions for the `fmt` command: md <-> html <-> xml <-> docx.
//!
//! All conversions go through a shared intermediate representation (`Document`),
//! so any pair among `md`, `html`, `xml`, `docx` converts in at most two hops.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::Path;

// ════════════════════════════════════════════════════════════════════════
// Intermediate document model
// ════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Inline {
    Text(String),
    Bold(Vec<Inline>),
    Italic(Vec<Inline>),
    Underline(Vec<Inline>),
    Strikethrough(Vec<Inline>),
    Highlight(Vec<Inline>),
    Code(String),
    Link {
        url: String,
        content: Vec<Inline>,
    },
    Image {
        src: String,
        alt: String,
    },
    /// Hard line break.
    LineBreak,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Block {
    Heading {
        level: u8,
        inlines: Vec<Inline>,
    },
    Paragraph(Vec<Inline>),
    Code {
        lang: Option<String>,
        text: String,
    },
    Quote(Vec<Block>),
    List {
        ordered: bool,
        items: Vec<Vec<Block>>,
    },
    Table {
        header: Option<Vec<Vec<Inline>>>,
        rows: Vec<Vec<Vec<Inline>>>,
    },
    HorizontalRule,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Document {
    pub blocks: Vec<Block>,
}

// ════════════════════════════════════════════════════════════════════════
// Generic XML tree (shared by XML, HTML and DOCX internals)
// ════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq)]
pub enum XmlChild {
    Text(String),
    Elem(XmlElem),
}

#[derive(Debug, Clone, PartialEq)]
pub struct XmlElem {
    pub name: String,
    pub attrs: Vec<(String, String)>,
    pub children: Vec<XmlChild>,
}

impl XmlElem {
    fn attr(&self, key: &str) -> Option<&str> {
        self.attrs
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(key))
            .map(|(_, v)| v.as_str())
    }

    /// Extract the interleaved text of this element's sub-tree.
    fn text(&self) -> String {
        let mut out = String::new();
        self.collect_text_into(&mut out);
        out
    }

    fn collect_text_into(&self, out: &mut String) {
        for child in &self.children {
            match child {
                XmlChild::Text(t) => out.push_str(t),
                XmlChild::Elem(e) => e.collect_text_into(out),
            }
        }
    }
}

/// Decode XML/HTML entities.
fn unescape_entities(s: &str) -> String {
    let mut out = s.to_string();
    for (ent, ch) in [
        ("&lt;", "<"),
        ("&gt;", ">"),
        ("&quot;", "\""),
        ("&apos;", "'"),
        ("&nbsp;", "\u{00a0}"),
        ("&amp;", "&"),
    ] {
        out = out.replace(ent, ch);
    }
    out
}

/// Escape a string for inclusion in XML/HTML content.
fn escape_entities(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

// ─── forgiving XML parse via quick-xml ─────────────────────────────────

/// Parse XML text into top-level children of the root element.
fn parse_xml_tree(input: &str) -> Result<Vec<XmlChild>> {
    use quick_xml::events::Event;
    use quick_xml::Reader;

    let mut reader = Reader::from_str(input);

    // stack[0] is a synthetic root that collects top-level children.
    let root = XmlElem {
        name: String::new(),
        attrs: Vec::new(),
        children: Vec::new(),
    };
    let mut stack: Vec<XmlElem> = vec![root];

    loop {
        let ev = reader
            .read_event()
            .map_err(|e| anyhow!("XML parse error: {}", e))?;
        match ev {
            Event::Start(ref e) | Event::Empty(ref e) => {
                let name = e.name().as_ref().to_lowercase();
                let mut attrs = Vec::new();
                for attr in e.attributes().with_checks(false).flatten() {
                    let key = attr.key.as_ref().to_lowercase();
                    let raw = attr.value.as_ref().to_string();
                    attrs.push((key, unescape_entities(&raw)));
                }
                stack.push(XmlElem {
                    name,
                    attrs,
                    children: Vec::new(),
                });
                if matches!(ev, Event::Empty(_)) {
                    // Self-closing element: immediately pop it back.
                    let done = stack.pop().expect("xml stack");
                    stack
                        .last_mut()
                        .expect("xml root")
                        .children
                        .push(XmlChild::Elem(done));
                }
            }
            Event::End(_) => {
                if stack.len() > 1 {
                    let done = stack.pop().expect("xml stack");
                    stack
                        .last_mut()
                        .expect("xml root")
                        .children
                        .push(XmlChild::Elem(done));
                }
            }
            Event::Text(t) => {
                let text = unescape_entities(t.as_ref());
                stack
                    .last_mut()
                    .expect("xml root")
                    .children
                    .push(XmlChild::Text(text));
            }
            Event::CData(t) => {
                stack
                    .last_mut()
                    .expect("xml root")
                    .children
                    .push(XmlChild::Text(t.as_ref().to_string()));
            }
            Event::Eof => break,
            _ => {}
        }
    }

    Ok(stack
        .into_iter()
        .next()
        .map(|r| r.children)
        .unwrap_or_default())
}

// ─── HTML parse via scraper, converted to the generic tree ─────────────

const SKIP_HTML_TAGS: &[&str] = &[
    "head", "script", "style", "meta", "title", "link", "noscript",
];

fn html_to_xml_children(input: &str) -> Result<Vec<XmlChild>> {
    use scraper::Html;

    let doc = Html::parse_document(input);
    let root = doc.tree.root();

    let mut out = Vec::new();
    for child in root.children() {
        convert_dom(child, &mut out);
    }
    Ok(out)
}

fn convert_dom(node: ego_tree::NodeRef<'_, scraper::node::Node>, out: &mut Vec<XmlChild>) {
    use scraper::node::Node as DomNode;
    match node.value() {
        DomNode::Text(t) => {
            let text = t.to_string();
            if !text.trim().is_empty() {
                out.push(XmlChild::Text(text));
            }
        }
        DomNode::Element(el) => {
            let name = el.name().to_lowercase();
            if SKIP_HTML_TAGS.contains(&name.as_str()) {
                return;
            }
            let mut attrs = Vec::new();
            for key in [
                "href", "src", "alt", "title", "class", "type", "target", "lang",
            ] {
                if let Some(v) = el.attr(key) {
                    attrs.push((key.to_string(), v.to_string()));
                }
            }
            let mut children = Vec::new();
            for child in node.children() {
                convert_dom(child, &mut children);
            }
            out.push(XmlChild::Elem(XmlElem {
                name,
                attrs,
                children,
            }));
        }
        _ => {
            for child in node.children() {
                convert_dom(child, out);
            }
        }
    }
}

// ════════════════════════════════════════════════════════════════════════
// Generic XML tree -> Document (shared by XML and HTML)
// ════════════════════════════════════════════════════════════════════════

/// Inline-level tags we understand; unknown tags are treated as transparent.
const HTML_INLINE_TAGS: &[&str] = &[
    "b", "strong", "i", "em", "u", "ins", "s", "strike", "del", "mark", "code", "a", "img", "br",
    "span", "small", "sub", "sup", "font", "wbr",
];

fn heading_level(name: &str) -> Option<u8> {
    let bytes = name.as_bytes();
    if name.len() == 2 && bytes[0] == b'h' && bytes[1].is_ascii_digit() {
        let level = bytes[1] - b'0';
        if (1..=6).contains(&level) {
            return Some(level);
        }
    }
    None
}

fn doc_from_xml_children(children: &[XmlChild]) -> Document {
    Document {
        blocks: blocks_from_xml_children(children),
    }
}

/// Convert tree children into blocks. Consecutive inline content becomes a
/// paragraph; blocks flush pending inline content first.
fn blocks_from_xml_children(children: &[XmlChild]) -> Vec<Block> {
    let mut blocks: Vec<Block> = Vec::new();
    let mut cur_inlines: Vec<Inline> = Vec::new();

    for child in children {
        match child {
            XmlChild::Text(t) => {
                let text = unescape_entities(t);
                if !text.trim().is_empty() {
                    cur_inlines.push(Inline::Text(text));
                }
            }
            XmlChild::Elem(e) => {
                let is_block = matches!(
                    e.name.as_str(),
                    "p" | "blockquote" | "ul" | "ol" | "pre" | "table" | "hr" | "div"
                ) || heading_level(&e.name).is_some()
                    || e.name == "code";
                if is_block {
                    flush_paragraph(&mut cur_inlines, &mut blocks);
                }
                match e.name.as_str() {
                    "p" => {
                        if let Some(inlines) = merge_inlines(inlines_from_xml_children(&e.children))
                        {
                            blocks.push(Block::Paragraph(inlines));
                        }
                    }
                    name if heading_level(name).is_some() => {
                        blocks.push(Block::Heading {
                            level: heading_level(name).expect("checked"),
                            inlines: inlines_from_xml_children(&e.children),
                        });
                    }
                    "blockquote" => {
                        blocks.push(Block::Quote(doc_from_xml_children(&e.children).blocks));
                    }
                    "ul" | "ol" => blocks.push(list_from_xml(e)),
                    "pre" | "code" => blocks.push(code_block_from_xml(e)),
                    "table" => {
                        if let Some(t) = table_from_xml(e) {
                            blocks.push(t);
                        }
                    }
                    "hr" => blocks.push(Block::HorizontalRule),
                    "br" => cur_inlines.push(Inline::LineBreak),
                    name if HTML_INLINE_TAGS.contains(&name) => {
                        if let Some(inline) = inline_from_xml(e) {
                            cur_inlines.push(inline);
                        }
                    }
                    // Unknown containers (<div>, <section>, …): descend.
                    _ => {
                        blocks.extend(blocks_from_xml_children(&e.children));
                    }
                }
            }
        }
    }
    flush_paragraph(&mut cur_inlines, &mut blocks);
    blocks
}

/// Flush pending inline content as a paragraph (if any).
fn flush_paragraph(cur: &mut Vec<Inline>, blocks: &mut Vec<Block>) {
    if let Some(inlines) = merge_inlines(std::mem::take(cur)) {
        blocks.push(Block::Paragraph(inlines));
    }
}

/// Drop whitespace-only text and merge neighbouring text runs.
fn merge_inlines(inlines: Vec<Inline>) -> Option<Vec<Inline>> {
    let mut out: Vec<Inline> = Vec::new();
    for inline in inlines {
        match inline {
            Inline::Text(t) => {
                if t.trim().is_empty() {
                    continue;
                }
                if let Some(Inline::Text(prev)) = out.last_mut() {
                    prev.push_str(&t);
                } else {
                    out.push(Inline::Text(t));
                }
            }
            other => out.push(other),
        }
    }
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

fn list_from_xml(list_elem: &XmlElem) -> Block {
    let ordered = list_elem.name == "ol";
    let mut items = Vec::new();
    for child in &list_elem.children {
        if let XmlChild::Elem(e) = child {
            if e.name == "li" {
                items.push(blocks_from_xml_children(&e.children));
            }
        }
    }
    Block::List { ordered, items }
}

fn code_block_from_xml(elem: &XmlElem) -> Block {
    // A bare <code> (inline-position) or a <pre><code class="…">…</code></pre>.
    let (lang, text) = match elem.children.first() {
        Some(XmlChild::Elem(inner)) if inner.name == "code" => {
            let class_or_lang = inner
                .attr("lang")
                .or_else(|| inner.attr("class"))
                .map(|s| s.trim().to_string())
                .unwrap_or_default();
            let lang = class_or_lang
                .strip_prefix("language-")
                .unwrap_or(&class_or_lang)
                .to_string();
            let lang = if lang.is_empty() { None } else { Some(lang) };
            (lang, inner.text())
        }
        _ => (None, elem.text()),
    };
    Block::Code {
        lang,
        text: text.trim_end_matches(['\n', '\r']).to_string(),
    }
}

fn table_from_xml(table_elem: &XmlElem) -> Option<Block> {
    let mut header: Option<Vec<Vec<Inline>>> = None;
    let mut rows: Vec<Vec<Vec<Inline>>> = Vec::new();

    for child in &table_elem.children {
        match child {
            XmlChild::Elem(wrap) if matches!(wrap.name.as_str(), "thead" | "tbody" | "tfoot") => {
                for row_child in &wrap.children {
                    if let XmlChild::Elem(row) = row_child {
                        if row.name != "tr" {
                            continue;
                        }
                        if wrap.name == "thead" && header.is_none() {
                            header = Some(cells_from_row(row));
                        } else {
                            rows.push(cells_from_row(row));
                        }
                    }
                }
            }
            XmlChild::Elem(row) if row.name == "tr" => {
                let has_th = row
                    .children
                    .iter()
                    .any(|c| matches!(c, XmlChild::Elem(e) if e.name == "th"));
                if has_th && header.is_none() {
                    header = Some(cells_from_row(row));
                } else {
                    rows.push(cells_from_row(row));
                }
            }
            _ => {}
        }
    }
    Some(Block::Table { header, rows })
}

fn cells_from_row(row: &XmlElem) -> Vec<Vec<Inline>> {
    let mut cells = Vec::new();
    for cell in &row.children {
        if let XmlChild::Elem(e) = cell {
            if matches!(e.name.as_str(), "td" | "th") {
                cells.push(inlines_from_xml_children(&e.children));
            }
        }
    }
    cells
}

// ─── inline conversion for XML / HTML ──────────────────────────────────

fn inlines_from_xml_children(children: &[XmlChild]) -> Vec<Inline> {
    let mut out = Vec::new();
    for child in children {
        match child {
            XmlChild::Text(t) => {
                let text = unescape_entities(t);
                if !text.trim().is_empty() {
                    out.push(Inline::Text(text));
                }
            }
            XmlChild::Elem(e) => {
                if let Some(inline) = inline_from_xml(e) {
                    out.push(inline);
                }
            }
        }
    }
    merge_inlines(out).unwrap_or_default()
}

fn inline_from_xml(elem: &XmlElem) -> Option<Inline> {
    match elem.name.as_str() {
        "b" | "strong" => Some(Inline::Bold(inlines_from_xml_children(&elem.children))),
        "i" | "em" => Some(Inline::Italic(inlines_from_xml_children(&elem.children))),
        "u" | "ins" => Some(Inline::Underline(inlines_from_xml_children(&elem.children))),
        "s" | "strike" | "del" => Some(Inline::Strikethrough(inlines_from_xml_children(
            &elem.children,
        ))),
        "mark" => Some(Inline::Highlight(inlines_from_xml_children(&elem.children))),
        "code" => Some(Inline::Code(elem.text())),
        "a" => {
            let url = elem
                .attr("href")
                .or_else(|| elem.attr("url"))
                .unwrap_or("")
                .to_string();
            let content = inlines_from_xml_children(&elem.children);
            Some(Inline::Link {
                url: url.clone(),
                content: if content.is_empty() {
                    vec![Inline::Text(url)]
                } else {
                    content
                },
            })
        }
        "img" => Some(Inline::Image {
            src: elem.attr("src").unwrap_or("").to_string(),
            alt: elem.attr("alt").unwrap_or("").to_string(),
        }),
        "br" => Some(Inline::LineBreak),
        // Transparent wrappers.
        _ => None,
    }
}

// ════════════════════════════════════════════════════════════════════════
// Markdown -> Document (via comrak)
// ════════════════════════════════════════════════════════════════════════

fn doc_from_markdown(md: &str) -> Result<Document> {
    use comrak::{parse_document, Options};

    let arena = comrak::Arena::new();
    let mut options = Options::default();
    options.extension.table = true;
    options.extension.strikethrough = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;

    let root = parse_document(&arena, md, &options);
    let mut blocks = Vec::new();
    for child in root.children() {
        push_comrak_block(child, &mut blocks);
    }
    Ok(Document { blocks })
}

fn push_comrak_block<'a>(node: comrak::nodes::Node<'a>, blocks: &mut Vec<Block>) {
    use comrak::nodes::{NodeCodeBlock, NodeValue};
    match &node.data().value {
        NodeValue::Heading(heading) => {
            let inlines = comrak_inlines(node);
            blocks.push(Block::Heading {
                level: heading.level,
                inlines,
            });
        }
        NodeValue::Paragraph => {
            if let Some(inlines) = merge_inlines(comrak_inlines(node)) {
                blocks.push(Block::Paragraph(inlines));
            }
        }
        NodeValue::CodeBlock(block_data) => {
            let NodeCodeBlock { info, literal, .. } = &**block_data;
            let lang = info
                .split_whitespace()
                .next()
                .map(|s| s.to_string())
                .filter(|s| !s.is_empty());
            blocks.push(Block::Code {
                lang,
                text: literal.trim_end_matches('\n').to_string(),
            });
        }
        NodeValue::BlockQuote | NodeValue::MultilineBlockQuote(_) => {
            let mut inner = Vec::new();
            for child in node.children() {
                push_comrak_block(child, &mut inner);
            }
            blocks.push(Block::Quote(inner));
        }
        NodeValue::List(_) => {
            let (ordered, items) = comrak_list(node);
            blocks.push(Block::List { ordered, items });
        }
        NodeValue::Table(_) => {
            let mut header: Option<Vec<Vec<Inline>>> = None;
            let mut rows = Vec::new();
            for row in node.children() {
                let is_header = matches!(row.data().value, NodeValue::TableRow(true));
                let mut cells = Vec::new();
                for cell in row.children() {
                    cells.push(comrak_inlines(cell));
                }
                if is_header && header.is_none() {
                    header = Some(cells);
                } else {
                    rows.push(cells);
                }
            }
            blocks.push(Block::Table { header, rows });
        }
        NodeValue::ThematicBreak => blocks.push(Block::HorizontalRule),
        NodeValue::HtmlBlock(_) => {} // raw HTML blocks are dropped
        _ => {
            // Unknown block containers: keep their children (flat).
            for child in node.children() {
                push_comrak_block(child, blocks);
            }
        }
    }
}

/// Convert a comrak list: each `Item` child becomes one list item; nested
/// `List` blocks inside an item become nested lists.
fn comrak_list<'a>(node: comrak::nodes::Node<'a>) -> (bool, Vec<Vec<Block>>) {
    use comrak::nodes::{ListType, NodeValue};

    // `node` is the List node; its own value carries the list type.
    let mut ordered = false;
    if let NodeValue::List(nl) = &node.data().value {
        ordered = matches!(nl.list_type, ListType::Ordered);
    }

    let mut items: Vec<Vec<Block>> = Vec::new();
    for item in node.children() {
        let mut item_blocks = Vec::new();
        for child in item.children() {
            match child.data().value {
                NodeValue::Paragraph => {
                    if let Some(inlines) = merge_inlines(comrak_inlines(child)) {
                        item_blocks.push(Block::Paragraph(inlines));
                    }
                }
                _ => push_comrak_block(child, &mut item_blocks),
            }
        }
        items.push(item_blocks);
    }
    (ordered, items)
}

fn comrak_inlines<'a>(node: comrak::nodes::Node<'a>) -> Vec<Inline> {
    use comrak::nodes::{NodeCode, NodeMath, NodeValue};
    let mut out = Vec::new();
    for child in node.children() {
        match &child.data().value {
            NodeValue::Text(t) => out.push(Inline::Text(t.as_ref().to_string())),
            NodeValue::SoftBreak => match out.last_mut() {
                Some(Inline::Text(prev)) => {
                    if !prev.ends_with(' ') {
                        prev.push(' ');
                    }
                }
                _ => out.push(Inline::Text(" ".to_string())),
            },
            NodeValue::LineBreak => out.push(Inline::LineBreak),
            NodeValue::Code(NodeCode { literal, .. }) => out.push(Inline::Code(literal.clone())),
            NodeValue::Strong => out.push(Inline::Bold(comrak_inlines(child))),
            NodeValue::Emph => out.push(Inline::Italic(comrak_inlines(child))),
            NodeValue::Underline | NodeValue::Insert => {
                out.push(Inline::Underline(comrak_inlines(child)))
            }
            NodeValue::Strikethrough => out.push(Inline::Strikethrough(comrak_inlines(child))),
            NodeValue::Highlight => out.push(Inline::Highlight(comrak_inlines(child))),
            NodeValue::Link(link_data) => out.push(Inline::Link {
                url: link_data.url.clone(),
                content: comrak_inlines(child),
            }),
            NodeValue::Image(link_data) => out.push(Inline::Image {
                src: link_data.url.clone(),
                alt: child.collect_text(),
            }),
            NodeValue::HtmlInline(_) | NodeValue::Raw(_) => {}
            NodeValue::Math(NodeMath { literal, .. }) => out.push(Inline::Code(literal.clone())),
            _ => out.extend(comrak_inlines(child)),
        }
    }
    out
}

// ════════════════════════════════════════════════════════════════════════
// Plain text and document JSON
// ════════════════════════════════════════════════════════════════════════

/// Turn plain text into paragraphs: blank lines separate blocks, single
/// newlines are preserved as hard line breaks.
fn doc_from_text(content: &str) -> Document {
    let normalized = content.replace("\r\n", "\n").replace('\r', "\n");
    let trimmed = normalized.trim_matches('\n');
    if trimmed.trim().is_empty() {
        return Document::default();
    }
    let blocks = trimmed
        .split("\n\n")
        .filter(|p| !p.trim().is_empty())
        .map(|p| {
            let mut inlines = Vec::new();
            for (i, line) in p.lines().enumerate() {
                if i > 0 {
                    inlines.push(Inline::LineBreak);
                }
                if !line.is_empty() {
                    inlines.push(Inline::Text(line.to_string()));
                }
            }
            Block::Paragraph(inlines)
        })
        .collect();
    Document { blocks }
}

fn inlines_to_text(inlines: &[Inline]) -> String {
    let mut out = String::new();
    for inline in inlines {
        match inline {
            Inline::Text(s) | Inline::Code(s) => out.push_str(s),
            Inline::Bold(inner)
            | Inline::Italic(inner)
            | Inline::Underline(inner)
            | Inline::Strikethrough(inner)
            | Inline::Highlight(inner) => out.push_str(&inlines_to_text(inner)),
            Inline::Link { url, content } => {
                let text = inlines_to_text(content);
                if text.is_empty() {
                    out.push_str(url);
                } else {
                    out.push_str(&text);
                    if !url.is_empty() && url != &text {
                        out.push_str(" (");
                        out.push_str(url);
                        out.push(')');
                    }
                }
            }
            Inline::Image { src, alt } => {
                if alt.is_empty() {
                    out.push_str(src);
                } else {
                    out.push_str(alt);
                }
            }
            Inline::LineBreak => out.push('\n'),
        }
    }
    out
}

fn list_to_text(items: &[Vec<Block>], ordered: bool) -> String {
    let mut out = String::new();
    for (i, item) in items.iter().enumerate() {
        let marker = if ordered {
            format!("{}. ", i + 1)
        } else {
            "- ".to_string()
        };
        let body = blocks_to_text(item);
        for (j, line) in body.lines().enumerate() {
            if j == 0 {
                out.push_str(&marker);
                out.push_str(line);
            } else {
                out.push_str("   ");
                out.push_str(line);
            }
            out.push('\n');
        }
    }
    out.trim_end_matches('\n').to_string()
}

fn table_to_text(header: Option<&Vec<Vec<Inline>>>, rows: &[Vec<Vec<Inline>>]) -> String {
    let render_row = |cells: &[Vec<Inline>]| {
        cells
            .iter()
            .map(|cell| inlines_to_text(cell))
            .collect::<Vec<_>>()
            .join(" | ")
    };
    let mut lines = Vec::new();
    if let Some(h) = header {
        lines.push(render_row(h));
    }
    for row in rows {
        lines.push(render_row(row));
    }
    lines.join("\n")
}

fn block_to_text(block: &Block) -> String {
    match block {
        Block::Heading { inlines, .. } | Block::Paragraph(inlines) => inlines_to_text(inlines),
        Block::Code { text, .. } => text.clone(),
        Block::Quote(inner) => prefix_lines(&blocks_to_text(inner), "> "),
        Block::List { ordered, items } => list_to_text(items, *ordered),
        Block::Table { header, rows } => table_to_text(header.as_ref(), rows),
        Block::HorizontalRule => "---".to_string(),
    }
}

fn blocks_to_text(blocks: &[Block]) -> String {
    blocks
        .iter()
        .map(block_to_text)
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn document_to_text(doc: &Document) -> String {
    let body = blocks_to_text(&doc.blocks);
    if body.trim().is_empty() {
        String::new()
    } else {
        format!("{}\n", body.trim_end())
    }
}

fn document_to_json(doc: &Document) -> Result<String> {
    serde_json::to_string_pretty(doc)
        .map_err(|e| anyhow!("document JSON serialization error: {}", e))
}

fn doc_from_json(content: &str) -> Result<Document> {
    serde_json::from_str(content).map_err(|e| anyhow!("invalid document JSON: {}", e))
}

// ════════════════════════════════════════════════════════════════════════
// Public parse/serialize entry points
// ════════════════════════════════════════════════════════════════════════

/// Parse `md`, `html`, `xml`, `txt` or document `json` content into the
/// intermediate `Document`.
pub fn parse_document_text(content: &str, ext: &str) -> Result<Document> {
    match ext {
        "md" => doc_from_markdown(content),
        "html" => {
            let tree = html_to_xml_children(content)?;
            Ok(doc_from_xml_children(&tree))
        }
        "xml" => {
            let tree = parse_xml_tree(content)?;
            // If the XML root is a wrapper (e.g. <body>, <article>), look inside.
            Ok(doc_from_xml_children(&tree))
        }
        "txt" => Ok(doc_from_text(content)),
        "json" => doc_from_json(content),
        _ => bail!("unsupported document extension: {}", ext),
    }
}

/// Serialize a `Document` back to `md`, `html`, `xml`, `txt` or document `json`.
pub fn serialize_document_text(doc: &Document, ext: &str) -> Result<String> {
    match ext {
        "md" => Ok(format!("{}\n", blocks_to_markdown(&doc.blocks))),
        "html" => Ok(document_to_html(doc)),
        "xml" => Ok(document_to_xml(doc)),
        "txt" => Ok(document_to_text(doc)),
        "json" => document_to_json(doc),
        _ => bail!("unsupported document extension: {}", ext),
    }
}

// ════════════════════════════════════════════════════════════════════════
// Markdown emission
// ════════════════════════════════════════════════════════════════════════

fn blocks_to_markdown(blocks: &[Block]) -> String {
    let mut out = String::new();
    for block in blocks {
        out.push_str(&block_to_markdown(block));
    }
    out
}

fn block_to_markdown(block: &Block) -> String {
    match block {
        Block::Heading { level, inlines } => {
            format!(
                "{} {}\n\n",
                "#".repeat(*level as usize),
                inlines_to_markdown(inlines)
            )
        }
        Block::Paragraph(inlines) => {
            format!("{}\n\n", inlines_to_markdown(inlines))
        }
        Block::Code { lang, text } => {
            format!("```{}\n{}\n```\n\n", lang.clone().unwrap_or_default(), text)
        }
        Block::Quote(inner) => {
            let inner_md = blocks_to_markdown(inner);
            format!(
                "{}\n\n",
                prefix_lines(inner_md.trim_end_matches('\n'), "> ")
            )
        }
        Block::List { ordered, items } => {
            format!("{}\n\n", list_to_markdown(items, *ordered, 0))
        }
        Block::Table { header, rows } => {
            format!("{}\n\n", table_to_markdown(header.as_ref(), rows))
        }
        Block::HorizontalRule => "---\n\n".to_string(),
    }
}

fn prefix_lines(s: &str, prefix: &str) -> String {
    s.split('\n')
        .map(|line| prefix.to_string() + line)
        .collect::<Vec<_>>()
        .join("\n")
}

fn list_to_markdown(items: &[Vec<Block>], ordered: bool, depth: usize) -> String {
    let mut out = String::new();
    let indent = "  ".repeat(depth);
    for (i, item) in items.iter().enumerate() {
        let marker = if ordered {
            format!("{}. ", i + 1)
        } else {
            "- ".to_string()
        };
        for (j, blk) in item.iter().enumerate() {
            let continuing = j > 0;
            match blk {
                Block::Paragraph(inlines) => {
                    if continuing {
                        out.push_str(&indent);
                        out.push_str(&"  ".repeat(marker.chars().count()));
                    } else {
                        out.push_str(&indent);
                        out.push_str(&marker);
                    }
                    out.push_str(&inlines_to_markdown(inlines));
                    out.push('\n');
                }
                Block::Code { lang, text } => {
                    out.push_str(&format!(
                        "{}  ```{}\n{}\n{}  ```\n",
                        indent,
                        lang.clone().unwrap_or_default(),
                        text,
                        indent
                    ));
                }
                Block::List {
                    ordered,
                    items: nested,
                } => {
                    out.push_str(&list_to_markdown(nested, *ordered, depth + 1));
                    out.push('\n');
                }
                _ => {
                    let rendered = block_to_markdown(blk);
                    out.push_str(&prefix_lines(
                        rendered.trim_end_matches('\n'),
                        &format!("{}  > ", indent),
                    ));
                    out.push('\n');
                }
            }
        }
    }
    out.trim_end_matches('\n').to_string()
}

fn table_to_markdown(header: Option<&Vec<Vec<Inline>>>, rows: &[Vec<Vec<Inline>>]) -> String {
    let mut out = String::new();
    if let Some(h) = header {
        out.push_str(&row_to_md(h));
        out.push('\n');
        out.push_str(&format!("|{}\n", " --- |".repeat(h.len().max(1))));
    }
    for row in rows {
        out.push_str(&row_to_md(row));
        out.push('\n');
    }
    out
}

fn row_to_md(cells: &[Vec<Inline>]) -> String {
    let mut s = String::from("|");
    for cell in cells {
        s.push_str(&format!(" {} |", inlines_to_markdown(cell)));
    }
    s
}

fn inlines_to_markdown(inlines: &[Inline]) -> String {
    let mut out = String::new();
    for inline in inlines {
        match inline {
            Inline::Text(t) => out.push_str(&escape_md_text(t)),
            Inline::Bold(inner) => out.push_str(&format!("**{}**", inlines_to_markdown(inner))),
            Inline::Italic(inner) => out.push_str(&format!("*{}*", inlines_to_markdown(inner))),
            Inline::Underline(inner) => {
                out.push_str(&format!("<u>{}</u>", inlines_to_markdown(inner)))
            }
            Inline::Strikethrough(inner) => {
                out.push_str(&format!("~~{}~~", inlines_to_markdown(inner)))
            }
            Inline::Highlight(inner) => {
                out.push_str(&format!("=={}==", inlines_to_markdown(inner)))
            }
            Inline::Code(code) => out.push_str(&format!("`{}`", code)),
            Inline::Link { url, content } => {
                out.push_str(&format!("[{}]({})", inlines_to_markdown(content), url))
            }
            Inline::Image { src, alt } => {
                out.push_str(&format!("![{}]({})", escape_md_text(alt), src))
            }
            Inline::LineBreak => out.push_str("\n"),
        }
    }
    out
}

/// Escape characters that would be interpreted as markdown structure,
/// except inside emitted markers and code spans.
fn escape_md_text(t: &str) -> String {
    let mut out = String::with_capacity(t.len());
    for c in t.chars() {
        if matches!(c, '\\' | '*' | ']' | '~') || (c == '[') {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

// ════════════════════════════════════════════════════════════════════════
// HTML / XML emission
// ════════════════════════════════════════════════════════════════════════

fn document_to_html(doc: &Document) -> String {
    let mut out = String::from(
        "<!doctype html>\n<html>\n<head>\n<meta charset=\"utf-8\">\n</head>\n<body>\n",
    );
    for block in &doc.blocks {
        out.push_str(&block.to_html());
    }
    out.push_str("</body>\n</html>\n");
    out
}

fn document_to_xml(doc: &Document) -> String {
    let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<body>\n");
    for block in &doc.blocks {
        out.push_str(&block.to_xml());
    }
    out.push_str("</body>\n");
    out
}

impl Block {
    fn to_html(&self) -> String {
        let mut out = String::new();
        match self {
            Block::Heading { level, inlines } => {
                out.push_str(&format!(
                    "<h{}>{}</h{}>\n",
                    level,
                    inlines_to_html(inlines),
                    level
                ));
            }
            Block::Paragraph(inlines) => {
                out.push_str(&format!("<p>{}</p>\n", inlines_to_html(inlines)));
            }
            Block::Code { lang, text } => {
                out.push_str(&format!(
                    "<pre><code{}>{}</code></pre>\n",
                    lang.as_ref()
                        .map(|l| format!(" lang=\"{}\"", escape_entities(l)))
                        .unwrap_or_default(),
                    escape_entities(text)
                ));
            }
            Block::Quote(blocks) => {
                out.push_str("<blockquote>\n");
                for b in blocks {
                    out.push_str(&b.to_html());
                }
                out.push_str("</blockquote>\n");
            }
            Block::List { ordered, items } => {
                out.push_str(if *ordered { "<ol>\n" } else { "<ul>\n" });
                for item in items {
                    let mut li = String::new();
                    for (i, blk) in item.iter().enumerate() {
                        if i > 0 {
                            if let Block::Paragraph(_) = blk {
                                li.push_str("</li>\n<li>");
                            }
                        }
                        match blk {
                            Block::Paragraph(inlines) => {
                                li.push_str(&format!("<p>{}</p>", inlines_to_html(inlines)))
                            }
                            _ => li.push_str(&blk.to_html()),
                        }
                    }
                    out.push_str("  <li>");
                    out.push_str(&li);
                    out.push_str("</li>\n");
                }
                out.push_str(if *ordered { "</ol>\n" } else { "</ul>\n" });
            }
            Block::Table { header, rows } => {
                out.push_str("<table>\n");
                if let Some(h) = header {
                    out.push_str("<thead>\n  <tr>");
                    for cell in h {
                        out.push_str(&format!("<th>{}</th>", inlines_to_html(cell)));
                    }
                    out.push_str("</tr>\n</thead>\n");
                }
                for row in rows {
                    out.push_str("  <tr>");
                    for cell in row {
                        out.push_str(&format!("<td>{}</td>", inlines_to_html(cell)));
                    }
                    out.push_str("</tr>\n");
                }
                out.push_str("</table>\n");
            }
            Block::HorizontalRule => out.push_str("<hr/>\n"),
        }
        out
    }
}

fn inlines_to_html(inlines: &[Inline]) -> String {
    let mut out = String::new();
    for inline in inlines {
        match inline {
            Inline::Text(t) => out.push_str(&escape_entities(t)),
            Inline::Bold(i) => out.push_str(&format!("<strong>{}</strong>", inlines_to_html(i))),
            Inline::Italic(i) => out.push_str(&format!("<em>{}</em>", inlines_to_html(i))),
            Inline::Underline(i) => out.push_str(&format!("<u>{}</u>", inlines_to_html(i))),
            Inline::Strikethrough(i) => out.push_str(&format!("<s>{}</s>", inlines_to_html(i))),
            Inline::Highlight(i) => out.push_str(&format!("<mark>{}</mark>", inlines_to_html(i))),
            Inline::Code(code) => out.push_str(&format!("<code>{}</code>", escape_entities(code))),
            Inline::Link { url, content } => out.push_str(&format!(
                "<a href=\"{}\">{}</a>",
                escape_entities(url),
                inlines_to_html(content)
            )),
            Inline::Image { src, alt } => out.push_str(&format!(
                "<img src=\"{}\" alt=\"{}\"/>",
                escape_entities(src),
                escape_entities(alt)
            )),
            Inline::LineBreak => out.push_str("<br/>"),
        }
    }
    out
}

// XML emission reuses the same HTML-like vocabulary; it just starts from
// <body> without the html/head preamble.
impl Block {
    fn to_xml(&self) -> String {
        let mut out = String::new();
        match self {
            Block::Heading { level, inlines } => {
                out.push_str(&format!(
                    "<h{}>{}</h{}>\n",
                    level,
                    inlines_to_xml(inlines),
                    level
                ));
            }
            Block::Paragraph(inlines) => {
                out.push_str(&format!("<p>{}</p>\n", inlines_to_xml(inlines)))
            }
            Block::Code { lang, text } => {
                out.push_str(&format!(
                    "<pre><code{}>{}</code></pre>\n",
                    lang.as_ref()
                        .map(|l| format!(" lang=\"{}\"", escape_entities(l)))
                        .unwrap_or_default(),
                    escape_entities(text)
                ));
            }
            Block::Quote(blocks) => {
                out.push_str("<blockquote>\n");
                for b in blocks {
                    out.push_str(&b.to_xml());
                }
                out.push_str("</blockquote>\n");
            }
            Block::List { ordered, items } => {
                out.push_str(if *ordered { "<ol>\n" } else { "<ul>\n" });
                for item in items {
                    out.push_str("  <li>");
                    for blk in item {
                        match blk {
                            Block::Paragraph(inlines) => {
                                out.push_str(&format!("<p>{}</p>", inlines_to_xml(inlines)))
                            }
                            _ => out.push_str(&blk.to_xml()),
                        }
                    }
                    out.push_str("</li>\n");
                }
                out.push_str(if *ordered { "</ol>\n" } else { "</ul>\n" });
            }
            Block::Table { header, rows } => {
                out.push_str("<table>\n");
                if let Some(h) = header {
                    out.push_str("<thead>\n  <tr>");
                    for cell in h {
                        out.push_str(&format!("<th>{}</th>", inlines_to_xml(cell)));
                    }
                    out.push_str("</tr>\n</thead>\n");
                }
                for row in rows {
                    out.push_str("  <tr>");
                    for cell in row {
                        out.push_str(&format!("<td>{}</td>", inlines_to_xml(cell)));
                    }
                    out.push_str("</tr>\n");
                }
                out.push_str("</table>\n");
            }
            Block::HorizontalRule => out.push_str("<hr/>\n"),
        }
        out
    }
}

fn inlines_to_xml(inlines: &[Inline]) -> String {
    let mut out = String::new();
    for inline in inlines {
        match inline {
            Inline::Text(t) => out.push_str(&escape_entities(t)),
            Inline::Bold(i) => out.push_str(&format!("<b>{}</b>", inlines_to_xml(i))),
            Inline::Italic(i) => out.push_str(&format!("<i>{}</i>", inlines_to_xml(i))),
            Inline::Underline(i) => out.push_str(&format!("<u>{}</u>", inlines_to_xml(i))),
            Inline::Strikethrough(i) => out.push_str(&format!("<s>{}</s>", inlines_to_xml(i))),
            Inline::Highlight(i) => out.push_str(&format!("<mark>{}</mark>", inlines_to_xml(i))),
            Inline::Code(code) => out.push_str(&format!("<code>{}</code>", escape_entities(code))),
            Inline::Link { url, content } => out.push_str(&format!(
                "<a href=\"{}\">{}</a>",
                escape_entities(url),
                inlines_to_xml(content)
            )),
            Inline::Image { src, alt } => out.push_str(&format!(
                "<img src=\"{}\" alt=\"{}\"/>",
                escape_entities(src),
                escape_entities(alt)
            )),
            Inline::LineBreak => out.push_str("<br/>"),
        }
    }
    out
}

/// Read a document-family source from disk: `docx` unzips the package; the
/// text formats are read as strings (BOM stripped) and parsed.
pub fn read_document(path: &Path, ext: &str) -> Result<Document> {
    match ext {
        "docx" => read_docx(path),
        _ => {
            let raw =
                std::fs::read_to_string(path).map_err(|e| anyhow!("failed to read file: {}", e))?;
            if raw.contains('\0') {
                bail!("the file looks binary; fmt only supports text-based formats here");
            }
            let content = raw.trim_start_matches('\u{feff}');
            parse_document_text(content, ext)
        }
    }
}

// ════════════════════════════════════════════════════════════════════════
// DOCX read (zip + quick-xml)
// ════════════════════════════════════════════════════════════════════════

/// DOCX round-trip supports: headings (Heading1..6/Title styles), paragraph
/// text with bold/italic/underline/strike, hyperlinks (via rels), lists
/// (w:numPr resolved against numbering.xml), tables, and hard breaks.
pub fn read_docx(path: &Path) -> Result<Document> {
    let file = std::fs::File::open(path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    // hyperlink relationships: rId -> url
    let mut rels: HashMap<String, String> = HashMap::new();
    if let Ok(mut rel_file) = archive.by_name("word/_rels/document.xml.rels") {
        let mut rels_xml = String::new();
        rel_file.read_to_string(&mut rels_xml)?;
        let tree = parse_xml_tree(&rels_xml)?;
        let rel_root = find_recursitive(&tree);
        for e in rel_root {
            if e.name.ends_with("relationship") {
                let id = e.attr("id").unwrap_or("").to_string();
                let target = e.attr("target").unwrap_or("").to_string();
                let mode = e.attr("targetmode").unwrap_or("internal");
                if !id.is_empty() && !target.is_empty() {
                    let resolved = if mode.eq_ignore_ascii_case("external") {
                        target
                    } else {
                        format!("#{}", target)
                    };
                    rels.insert(id, resolved);
                }
            }
        }
    }

    // numbering: numId -> abstractNumId -> lvl0 numFmt
    let ordered_lists = read_numbering_map(&mut archive);

    let mut doc_file = archive
        .by_name("word/document.xml")
        .map_err(|e| anyhow!("not a valid DOCX file ({})", e))?;
    let mut doc_xml = String::new();
    doc_file.read_to_string(&mut doc_xml)?;
    let tree = parse_xml_tree(&doc_xml)?;

    let body = find_recursive(&tree, "w:body");

    let mut doc = Document::default();
    if let Some(b) = body {
        for child in &b.children {
            if let XmlChild::Elem(e) = child {
                match e.name.as_str() {
                    "w:p" => push_docx_paragraph(e, &rels, &ordered_lists, &mut doc.blocks),
                    "w:tbl" => {
                        if let Some(table) = docx_table(e, &rels) {
                            doc.blocks.push(table);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(doc)
}

type Rels = HashMap<String, String>;

/// Flatten all element descendants in document order.
fn find_recursitive(children: &[XmlChild]) -> Vec<&XmlElem> {
    fn walk<'a>(children: &'a [XmlChild], out: &mut Vec<&'a XmlElem>) {
        for child in children {
            if let XmlChild::Elem(e) = child {
                out.push(e);
                walk(&e.children, out);
            }
        }
    }
    let mut out = Vec::new();
    walk(children, &mut out);
    out
}

/// Depth-first search of the first element with the given name.
fn find_recursive<'a>(children: &'a [XmlChild], name: &str) -> Option<&'a XmlElem> {
    for child in children {
        if let XmlChild::Elem(e) = child {
            if e.name == name {
                return Some(e);
            }
            if let Some(found) = find_recursive(&e.children, name) {
                return Some(found);
            }
        }
    }
    None
}

/// Map numId -> "ordered?" for lists, by reading w:numbering (numFmt decimal).
fn read_numbering_map(archive: &mut zip::ZipArchive<std::fs::File>) -> HashMap<u64, bool> {
    let mut map: HashMap<u64, bool> = HashMap::new();
    let mut abstract_fmt: HashMap<u64, String> = HashMap::new();
    if let Ok(mut num_file) = archive.by_name("word/numbering.xml") {
        let mut xml = String::new();
        if num_file.read_to_string(&mut xml).is_ok() {
            if let Ok(tree) = parse_xml_tree(&xml) {
                for e in find_recursitive(&tree) {
                    match e.name.as_str() {
                        "w:abstractnum" => {
                            let id = e
                                .attr("w:abstractnumid")
                                .and_then(|s| s.parse().ok())
                                .unwrap_or(u64::MAX);
                            let fmt = find_numfmt(e).unwrap_or_else(|| "bullet".to_string());
                            abstract_fmt.insert(id, fmt);
                        }
                        "w:num" => {
                            let num_id = e.attr("w:numid").and_then(|s| s.parse().ok());
                            let abstract_id = e.children.iter().find_map(|c| match c {
                                XmlChild::Elem(inner) if inner.name == "w:abstractnumid" => inner
                                    .attr("w:val")
                                    .and_then(|s| s.parse().ok())
                                    .or_else(|| inner.text().trim().parse().ok()),
                                _ => None,
                            });
                            if let (Some(num_id), Some(abstract_id)) = (num_id, abstract_id) {
                                let fmt =
                                    abstract_fmt.get(&abstract_id).cloned().unwrap_or_default();
                                let ordered = matches!(
                                    fmt.to_lowercase().as_str(),
                                    "decimal"
                                        | "lowerletter"
                                        | "upperletter"
                                        | "lowerroman"
                                        | "upperroman"
                                );
                                map.insert(num_id, ordered);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    map
}

fn find_numfmt(abstract_num: &XmlElem) -> Option<String> {
    for child in &abstract_num.children {
        if let XmlChild::Elem(lvl) = child {
            if lvl.name == "w:lvl" {
                let ilvl_zero = lvl.attr("w:ilvl").map(|s| s == "0").unwrap_or(true);
                if ilvl_zero {
                    for lvl_child in &lvl.children {
                        if let XmlChild::Elem(f) = lvl_child {
                            if f.name == "w:numfmt" {
                                return Some(
                                    f.attr("w:val")
                                        .map(|s| s.to_string())
                                        .unwrap_or_else(|| f.text()),
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

enum ParaKind {
    Heading {
        level: u8,
        inlines: Vec<Inline>,
    },
    Quote(Vec<Inline>),
    ListItem {
        ordered: bool,
        ilvl: u64,
        inlines: Vec<Inline>,
    },
    Paragraph(Vec<Inline>),
}

fn push_docx_paragraph(
    para: &XmlElem,
    rels: &Rels,
    ordered_lists: &HashMap<u64, bool>,
    blocks: &mut Vec<Block>,
) {
    let mut style: Option<String> = None;
    let mut numbering: Option<u64> = None;
    let mut ilvl: u64 = 0;

    for child in &para.children {
        if let XmlChild::Elem(e) = child {
            if e.name == "w:ppr" {
                for pchild in &e.children {
                    if let XmlChild::Elem(pe) = pchild {
                        match pe.name.as_str() {
                            "w:pstyle" => {
                                style = pe.attr("w:val").map(|s| s.to_lowercase());
                            }
                            "w:numpr" => {
                                for nchild in &pe.children {
                                    if let XmlChild::Elem(ne) = nchild {
                                        if ne.name == "w:numid" {
                                            let value = ne
                                                .attr("w:val")
                                                .map(|s| s.to_string())
                                                .or_else(|| Some(ne.text()))
                                                .unwrap_or_default();
                                            numbering = value.trim().parse().ok();
                                        }
                                        if ne.name == "w:ilvl" {
                                            let value = ne
                                                .attr("w:val")
                                                .map(|s| s.to_string())
                                                .or_else(|| Some(ne.text()))
                                                .unwrap_or_default();
                                            ilvl = value.trim().parse().unwrap_or(0);
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    let inlines = docx_runs(&para.children_no_ppr(), rels);
    if merge_inlines(inlines.clone()).is_none() {
        // Heading paragraphs may legitimately be empty on re-exports; treat as
        // empty otherwise.
        if style.is_some()
            && style
                .as_deref()
                .map(|s| s.starts_with("heading"))
                .unwrap_or(false)
        {
            blocks.push(Block::Paragraph(Vec::new()));
        }
        return;
    }

    let kind = match style.as_deref() {
        Some(s) if s.starts_with("heading") => extract_heading_level(s)
            .map(|level| ParaKind::Heading {
                level,
                inlines: inlines.clone(),
            })
            .unwrap_or(ParaKind::Paragraph(inlines)),
        Some("title") => ParaKind::Heading { level: 1, inlines },
        Some("quote") => ParaKind::Quote(inlines),
        _ => match numbering {
            Some(num_id) => ParaKind::ListItem {
                ordered: *ordered_lists.get(&num_id).unwrap_or(&false),
                ilvl,
                inlines,
            },
            None => ParaKind::Paragraph(inlines),
        },
    };

    match kind {
        ParaKind::Heading { level, inlines } => blocks.push(Block::Heading { level, inlines }),
        ParaKind::Quote(inlines) => {
            // Merge consecutive quote paragraphs into one quote block.
            match blocks.last_mut() {
                Some(Block::Quote(items)) => items.push(Block::Paragraph(inlines)),
                _ => blocks.push(Block::Quote(vec![Block::Paragraph(inlines)])),
            }
        }
        ParaKind::ListItem {
            ordered,
            ilvl,
            inlines,
        } => push_list_item(blocks, ordered, ilvl, inlines),
        ParaKind::Paragraph(inlines) => blocks.push(Block::Paragraph(inlines)),
    }
}

/// Merge a list item into a preceding `Block::List`, nesting via `ilvl`.
fn push_list_item(blocks: &mut Vec<Block>, ordered: bool, ilvl: u64, inlines: Vec<Inline>) {
    if ilvl == 0 {
        match blocks.last_mut() {
            Some(Block::List {
                ordered: last_ordered,
                items,
            }) if *last_ordered == ordered => {
                items.push(vec![Block::Paragraph(inlines)]);
            }
            _ => blocks.push(Block::List {
                ordered,
                items: vec![vec![Block::Paragraph(inlines)]],
            }),
        }
        return;
    }
    // Deeper level: attach into the last item of the enclosing list.
    if let Some(Block::List { items, .. }) = blocks.last_mut() {
        if let Some(item) = items.last_mut() {
            if let Some(Block::List {
                ordered: last_ordered,
                items: nested,
            }) = item.last_mut()
            {
                if *last_ordered == ordered {
                    nested.push(vec![Block::Paragraph(inlines)]);
                    return;
                }
            }
            item.push(Block::List {
                ordered,
                items: vec![vec![Block::Paragraph(inlines)]],
            });
        }
    } else {
        blocks.push(Block::List {
            ordered,
            items: vec![vec![Block::Paragraph(inlines)]],
        });
    }
}

impl XmlElem {
    /// All children except paragraph-properties (`w:pPr`).
    fn children_no_ppr(&self) -> Vec<XmlChild> {
        self.children
            .iter()
            .filter(|c| !matches!(c, XmlChild::Elem(e) if e.name == "w:ppr"))
            .cloned()
            .collect()
    }
}

fn extract_heading_level(style_lower: &str) -> Option<u8> {
    for level in 1..=9 {
        let prefix = format!("heading{}", level.min(6));
        if style_lower.starts_with(&prefix) {
            return Some(level.min(6) as u8);
        }
    }
    None
}

/// Walk the children of a paragraph (after `w:pPr` was removed) and collect
/// inline runs, including hyperlinks.
fn docx_runs(children: &[XmlChild], rels: &Rels) -> Vec<Inline> {
    let mut out = Vec::new();
    for child in children {
        if let XmlChild::Elem(e) = child {
            match e.name.as_str() {
                "w:r" => docx_run(e, rels, &mut out),
                "w:hyperlink" => {
                    let id = e
                        .attr("r:id")
                        .or_else(|| e.attr("id"))
                        .unwrap_or("")
                        .to_string();
                    let url = rels.get(&id).cloned().unwrap_or_default();
                    let mut inner = merge_inlines(docx_runs(&e.children, rels)).unwrap_or_default();
                    if inner.is_empty() {
                        inner = vec![Inline::Text(url.clone())];
                    }
                    out.push(Inline::Link {
                        url,
                        content: inner,
                    });
                }
                _ => {}
            }
        }
    }
    out
}

fn docx_run(run: &XmlElem, _rels: &Rels, out: &mut Vec<Inline>) {
    let mut props = (false, false, false, false); // bold, italic, underline, strike
    for child in &run.children {
        if let XmlChild::Elem(rpr) = child {
            if rpr.name == "w:rpr" {
                for s in &rpr.children {
                    if let XmlChild::Elem(se) = s {
                        match se.name.as_str() {
                            "w:b" => props.0 = docx_style_flag(se),
                            "w:i" => props.1 = docx_style_flag(se),
                            "w:u" => props.2 = docx_style_flag(se),
                            "w:strike" => props.3 = docx_style_flag(se),
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    for child in &run.children {
        if let XmlChild::Elem(e) = child {
            let inline = match e.name.as_str() {
                "w:t" => Some(Inline::Text(e.text())),
                "w:br" => Some(Inline::LineBreak),
                "w:tab" => Some(Inline::Text("\t".to_string())),
                _ => None,
            };
            if let Some(mut inline) = inline {
                if props.2 {
                    inline = Inline::Underline(vec![inline]);
                }
                if props.3 {
                    inline = Inline::Strikethrough(vec![inline]);
                }
                if props.1 {
                    inline = Inline::Italic(vec![inline]);
                }
                if props.0 {
                    inline = Inline::Bold(vec![inline]);
                }
                out.push(inline);
            }
        }
    }
}

fn docx_style_flag(elem: &XmlElem) -> bool {
    match elem.attr("w:val").or_else(|| elem.attr("val")) {
        Some(v) => {
            !v.eq_ignore_ascii_case("false")
                && !v.eq_ignore_ascii_case("0")
                && !v.eq_ignore_ascii_case("none")
        }
        None => true,
    }
}

fn docx_table(table: &XmlElem, rels: &Rels) -> Option<Block> {
    let mut first = true;
    let mut rows = Vec::new();
    let mut header: Option<Vec<Vec<Inline>>> = None;

    for row_child in &table.children {
        if let XmlChild::Elem(row) = row_child {
            if row.name != "w:tr" {
                continue;
            }
            let mut cells = Vec::new();
            for cell_child in &row.children {
                if let XmlChild::Elem(cell) = cell_child {
                    if cell.name != "w:tc" {
                        continue;
                    }
                    let mut cell_inlines = Vec::new();
                    for para_child in &cell.children {
                        if let XmlChild::Elem(para) = para_child {
                            if para.name == "w:p" {
                                let runs = docx_runs(&para.children_no_ppr(), rels);
                                if !runs.is_empty() {
                                    if !cell_inlines.is_empty() {
                                        cell_inlines.push(Inline::LineBreak);
                                    }
                                    cell_inlines.extend(merge_inlines(runs).unwrap_or_default());
                                }
                            }
                        }
                    }
                    cells.push(cell_inlines);
                }
            }
            if first {
                header = Some(cells);
                first = false;
            } else {
                rows.push(cells);
            }
        }
    }
    Some(Block::Table { header, rows })
}

// ════════════════════════════════════════════════════════════════════════
// DOCX write
// ════════════════════════════════════════════════════════════════════════

pub fn write_docx(doc: &Document, path: &Path) -> Result<()> {
    use zip::write::SimpleFileOptions;
    use zip::ZipWriter;

    let file = std::fs::File::create(path)?;
    let mut zip = ZipWriter::new(file);
    let opts = SimpleFileOptions::default();

    let mut link_counter: usize = 0;
    let body = docx_body(doc, &mut link_counter);
    let doc_rels = document_rels_xml(doc);

    zip.start_file("[Content_Types].xml", opts)?;
    zip.write_all(CONTENT_TYPES_XML.as_bytes())?;
    zip.start_file("_rels/.rels", opts)?;
    zip.write_all(ROOT_RELS_XML.as_bytes())?;
    zip.start_file("word/document.xml", opts)?;
    zip.write_all(body.as_bytes())?;
    zip.start_file("word/_rels/document.xml.rels", opts)?;
    zip.write_all(doc_rels.as_bytes())?;
    zip.start_file("word/numbering.xml", opts)?;
    zip.write_all(NUMBERING_XML.as_bytes())?;

    zip.finish()?;
    Ok(())
}

const CONTENT_TYPES_XML: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
<Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
<Default Extension="xml" ContentType="application/xml"/>
<Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>
<Override PartName="/word/numbering.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml"/>
</Types>"#;

const ROOT_RELS_XML: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/>
</Relationships>"#;

const NUMBERING_XML: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:numbering xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
<w:abstractNum w:abstractNumId="0">
  <w:lvl w:ilvl="0"><w:start w:val="1"/><w:numFmt w:val="bullet"/><w:lvlText w:val="•"/><w:lvlJc w:val="left"/></w:lvl>
</w:abstractNum>
<w:abstractNum w:abstractNumId="1">
  <w:lvl w:ilvl="0"><w:start w:val="1"/><w:numFmt w:val="decimal"/><w:lvlText w:val="%1."/><w:lvlJc w:val="left"/></w:lvl>
</w:abstractNum>
<w:num w:numId="1"><w:abstractNumId w:val="0"/></w:num>
<w:num w:numId="2"><w:abstractNumId w:val="1"/></w:num>
</w:numbering>"#;

fn document_rels_xml(doc: &Document) -> String {
    let mut urls: Vec<String> = Vec::new();
    collect_links_blocks(&doc.blocks, &mut urls);
    let mut out = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\">\n",
    );
    for (i, url) in urls.iter().enumerate() {
        out.push_str(&format!(
            "<Relationship Id=\"rIdH{}\" Type=\"{}\" Target=\"{}\" TargetMode=\"External\"/>\n",
            i + 1,
            NS_REL,
            url
        ));
    }
    out.push_str("</Relationships>");
    out
}

const NS_REL: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";

/// Collect hyperlink URLs in document order (their positional index becomes
/// the rId used in document.xml).
fn collect_links_blocks(blocks: &[Block], urls: &mut Vec<String>) {
    for block in blocks {
        match block {
            Block::Paragraph(inlines) => collect_links_inlines(inlines, urls),
            Block::Heading { inlines, .. } => collect_links_inlines(inlines, urls),
            Block::Quote(inner) => {
                for b in inner {
                    if let Block::Paragraph(inlines) = b {
                        collect_links_inlines(inlines, urls);
                    }
                }
            }
            Block::List { items, .. } => {
                for item in items {
                    collect_list_item_links(item, urls, 0);
                }
            }
            Block::Table { header, rows } => {
                if let Some(h) = header {
                    for cell in h {
                        collect_links_inlines(cell, urls);
                    }
                }
                for row in rows {
                    for cell in row {
                        collect_links_inlines(cell, urls);
                    }
                }
            }
            Block::Code { .. } | Block::HorizontalRule => {}
        }
    }
}

/// Mirror of `block_to_docx` list handling, so link indices stay aligned.
fn collect_list_item_links(item: &[Block], urls: &mut Vec<String>, depth: usize) {
    for blk in item {
        match blk {
            Block::Paragraph(inlines) => collect_links_inlines(inlines, urls),
            Block::List { items: nested, .. } => {
                for nested_item in nested {
                    collect_list_item_links(nested_item, urls, depth + 1);
                }
            }
            _ => {}
        }
    }
}

fn collect_links_inlines(inlines: &[Inline], urls: &mut Vec<String>) {
    for inline in inlines {
        match inline {
            Inline::Link { url, content } => {
                urls.push(url.clone());
                collect_links_inlines(content, urls);
            }
            Inline::Bold(i)
            | Inline::Italic(i)
            | Inline::Underline(i)
            | Inline::Strikethrough(i)
            | Inline::Highlight(i) => collect_links_inlines(i, urls),
            _ => {}
        }
    }
}

fn docx_body(doc: &Document, link_counter: &mut usize) -> String {
    let mut out = String::new();
    out.push_str(
        "<w:document xmlns:w=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\" xmlns:r=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships\">\n<w:body>\n",
    );
    for block in &doc.blocks {
        out.push_str(&block_to_docx(block, link_counter));
    }
    out.push_str("<w:p><w:pPr><w:sectPr/></w:pPr></w:p>\n</w:body>\n</w:document>");
    out
}

fn block_to_docx(block: &Block, link_counter: &mut usize) -> String {
    match block {
        Block::Heading { level, inlines } => {
            let runs = inlines_to_docx_runs_plain(inlines, link_counter);
            format!(
                "<w:p><w:pPr><w:pStyle w:val=\"Heading{}\"/></w:pPr>{}</w:p>\n",
                level, runs
            )
        }
        Block::Paragraph(inlines) => {
            let runs = inlines_to_docx_runs(inlines, false, false, false, false, link_counter);
            format!("<w:p>{}</w:p>\n", runs)
        }
        Block::Code { text, .. } => {
            let mut out = String::new();
            for line in text.lines() {
                out.push_str(&format!(
                    "<w:p><w:r><w:rPr><w:rFonts w:ascii=\"Consolas\" w:hAnsi=\"Consolas\"/></w:rPr><w:t xml:space=\"preserve\">{}</w:t></w:r></w:p>\n",
                    escape_entities(line)
                ));
            }
            out
        }
        Block::Quote(blocks) => {
            let mut out = String::new();
            for b in blocks {
                if let Block::Paragraph(inlines) = b {
                    let runs = inlines_to_docx_runs(inlines, false, true, false, false, link_counter);
                    out.push_str(&format!(
                        "<w:p><w:pPr><w:ind w:left=\"720\" w:right=\"720\"/></w:pPr>{}</w:p>\n",
                        runs
                    ));
                }
            }
            out
        }
        Block::List { ordered, items } => {
            let num_id = if *ordered { 2 } else { 1 };
            let mut out = String::new();
            for item in items {
                for blk in item {
                    match blk {
                        Block::Paragraph(inlines) => {
                            let runs = inlines_to_docx_runs(inlines, false, false, false, false, link_counter);
                            out.push_str(&format!(
                                "<w:p><w:pPr><w:numPr><w:ilvl w:val=\"0\"/><w:numId w:val=\"{}\"/></w:numPr><w:ind w:left=\"720\"/></w:pPr>{}</w:p>\n",
                                num_id, runs
                            ));
                        }
                        Block::List { ordered, items: nested } => {
                            let n_id = if *ordered { 2 } else { 1 };
                            for nested_item in nested {
                                for nb in nested_item {
                                    if let Block::Paragraph(ni) = nb {
                                        let runs = inlines_to_docx_runs(ni, false, false, false, false, link_counter);
                                        out.push_str(&format!(
                                            "<w:p><w:pPr><w:numPr><w:ilvl w:val=\"1\"/><w:numId w:val=\"{}\"/></w:numPr><w:ind w:left=\"1440\"/></w:pPr>{}</w:p>\n",
                                            n_id, runs
                                        ));
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            out
        }
        Block::Table { header, rows } => {
            let mut out = String::new();
            out.push_str("<w:tbl><w:tblPr><w:tblW w:w=\"0\" w:type=\"auto\"/><w:tblBorders><w:top w:val=\"single\" w:sz=\"4\" w:color=\"999999\"/><w:left w:val=\"single\" w:sz=\"4\" w:color=\"999999\"/><w:bottom w:val=\"single\" w:sz=\"4\" w:color=\"999999\"/><w:right w:val=\"single\" w:sz=\"4\" w:color=\"999999\"/><w:insideH w:val=\"single\" w:sz=\"4\" w:color=\"999999\"/><w:insideV w:val=\"single\" w:sz=\"4\" w:color=\"999999\"/></w:tblBorders></w:tblPr>\n");
            if let Some(h) = header {
                out.push_str("<w:tr>");
                for cell in h {
                    let runs = inlines_to_docx_runs(cell, false, false, true, false, link_counter);
                    out.push_str(&format!("<w:tc><w:p>{}</w:p></w:tc>", runs));
                }
                out.push_str("</w:tr>\n");
            }
            for row in rows {
                out.push_str("<w:tr>");
                for cell in row {
                    let runs = inlines_to_docx_runs(cell, false, false, false, false, link_counter);
                    out.push_str(&format!("<w:tc><w:p>{}</w:p></w:tc>", runs));
                }
                out.push_str("</w:tr>\n");
            }
            out.push_str("</w:tbl>\n");
            out
        }
        Block::HorizontalRule => {
            "<w:p><w:pPr><w:pBdr><w:bottom w:val=\"single\" w:sz=\"6\" w:space=\"1\"/></w:pBdr></w:pPr></w:p>\n".to_string()
        }
    }
}

/// Plain inline run emitter (top-level: no inherited styles).
fn inlines_to_docx_runs_plain(inlines: &[Inline], link_counter: &mut usize) -> String {
    inlines_to_docx_runs(inlines, false, false, false, false, link_counter)
}

/// Emit inline content as OOXML runs, resolving style combinations and
/// hyperlinks by counting link usage the same way `collect_links_*` does.
fn inlines_to_docx_runs(
    inlines: &[Inline],
    underline_inherited: bool,
    italic_inherited: bool,
    bold_inherited: bool,
    strike_inherited: bool,
    link_counter: &mut usize,
) -> String {
    let mut out = String::new();
    for inline in inlines {
        match inline {
            Inline::Text(t) => {
                out.push_str(&docx_text_run(
                    t,
                    bold_inherited,
                    italic_inherited,
                    underline_inherited,
                    strike_inherited,
                ));
            }
            Inline::Bold(inner) => {
                out.push_str(&inlines_to_docx_runs(
                    inner,
                    underline_inherited,
                    italic_inherited,
                    true,
                    strike_inherited,
                    link_counter,
                ));
            }
            Inline::Italic(inner) => {
                out.push_str(&inlines_to_docx_runs(
                    inner,
                    underline_inherited,
                    true,
                    bold_inherited,
                    strike_inherited,
                    link_counter,
                ));
            }
            Inline::Underline(inner) => {
                out.push_str(&inlines_to_docx_runs(
                    inner,
                    true,
                    italic_inherited,
                    bold_inherited,
                    strike_inherited,
                    link_counter,
                ));
            }
            Inline::Strikethrough(inner) => {
                out.push_str(&inlines_to_docx_runs(
                    inner,
                    underline_inherited,
                    italic_inherited,
                    bold_inherited,
                    true,
                    link_counter,
                ));
            }
            Inline::Highlight(inner) => {
                out.push_str(&inlines_to_docx_runs(
                    inner,
                    underline_inherited,
                    italic_inherited,
                    bold_inherited,
                    strike_inherited,
                    link_counter,
                ));
            }
            Inline::Code(code) => {
                out.push_str(&format!(
                    "<w:r><w:rPr><w:rFonts w:ascii=\"Consolas\" w:hAnsi=\"Consolas\"/>{}</w:rPr><w:t xml:space=\"preserve\">{}</w:t></w:r>",
                    if underline_inherited { "<w:u w:val=\"single\"/>" } else { "" },
                    escape_entities(code)
                ));
            }
            Inline::Link { url, content } => {
                let id = format!("rIdH{}", *link_counter + 1);
                *link_counter += 1;
                let inner = inlines_to_docx_runs(
                    content,
                    underline_inherited,
                    italic_inherited,
                    bold_inherited,
                    strike_inherited,
                    link_counter,
                );
                if url.is_empty() {
                    out.push_str(&inner);
                } else {
                    out.push_str(&format!(
                        "<w:hyperlink r:id=\"{}\">{}</w:hyperlink>",
                        id, inner
                    ));
                }
            }
            Inline::Image { alt, src } => {
                out.push_str(&format!(
                    "<w:r><w:rPr><w:i/></w:rPr><w:t xml:space=\"preserve\">[!: {} ({})]</w:t></w:r>",
                    escape_entities(alt),
                    escape_entities(src)
                ));
            }
            Inline::LineBreak => out.push_str("<w:r><w:br/></w:r>"),
        }
    }
    out
}

fn docx_text_run(text: &str, bold: bool, italic: bool, underline: bool, strike: bool) -> String {
    let mut rpr = String::new();
    if bold {
        rpr.push_str("<w:b/>");
    }
    if italic {
        rpr.push_str("<w:i/>");
    }
    if underline {
        rpr.push_str("<w:u w:val=\"single\"/>");
    }
    if strike {
        rpr.push_str("<w:strike/>");
    }
    let rpr = if rpr.is_empty() {
        String::new()
    } else {
        format!("<w:rPr>{}</w:rPr>", rpr)
    };
    format!(
        "<w:r>{}<w:t xml:space=\"preserve\">{}</w:t></w:r>",
        rpr,
        escape_entities(text)
    )
}

#[cfg(not(target_os = "android"))]
pub fn write_pdf(doc: &Document, path: &Path) -> Result<()> {
    use printpdf::*;
    use std::fs::File;
    use std::io::BufWriter;

    const PAGE_W: Mm = Mm(210.0);
    const PAGE_H: Mm = Mm(297.0);
    const MARGIN: Mm = Mm(18.0);
    const BOTTOM: f64 = 18.0;

    struct PdfLine {
        text: String,
        size: f64,
        spacing: f64,
        bold: bool,
        mono: bool,
    }

    fn wrap_text(text: &str, width: usize) -> Vec<String> {
        let mut lines = Vec::new();
        for raw in text.split('\n') {
            if raw.is_empty() {
                lines.push(String::new());
                continue;
            }
            let mut cur = String::new();
            let mut count = 0usize;
            let mut first = true;
            for word in raw.split(' ') {
                let wlen = word.chars().count();
                if count > 0 && count + 1 + wlen > width {
                    lines.push(std::mem::take(&mut cur));
                    count = 0;
                    first = true;
                }
                if !first {
                    cur.push(' ');
                    count += 1;
                }
                for ch in word.chars() {
                    if count >= width {
                        lines.push(std::mem::take(&mut cur));
                        count = 0;
                    }
                    cur.push(ch);
                    count += 1;
                }
                first = false;
            }
            if !cur.is_empty() {
                lines.push(cur);
            }
        }
        if lines.is_empty() {
            lines.push(String::new());
        }
        lines
    }

    fn push_lines(
        out: &mut Vec<PdfLine>,
        text: &str,
        width: usize,
        size: f64,
        spacing: f64,
        bold: bool,
        mono: bool,
    ) {
        for line in wrap_text(text, width) {
            out.push(PdfLine {
                text: line,
                size,
                spacing,
                bold,
                mono,
            });
        }
    }

    fn collect(blocks: &[Block], out: &mut Vec<PdfLine>) {
        for block in blocks {
            match block {
                Block::Heading { level, inlines } => {
                    let size = match level {
                        1 => 18.0,
                        2 => 15.0,
                        _ => 12.5,
                    };
                    push_lines(
                        out,
                        &inlines_to_text(inlines),
                        64,
                        size,
                        size + 4.0,
                        true,
                        false,
                    );
                    out.push(PdfLine {
                        text: String::new(),
                        size,
                        spacing: 4.0,
                        bold: false,
                        mono: false,
                    });
                }
                Block::Paragraph(inlines) => {
                    push_lines(out, &inlines_to_text(inlines), 96, 10.0, 4.8, false, false);
                    out.push(PdfLine {
                        text: String::new(),
                        size: 10.0,
                        spacing: 3.0,
                        bold: false,
                        mono: false,
                    });
                }
                Block::Code { text, .. } => {
                    push_lines(out, text, 92, 9.0, 4.2, false, true);
                    out.push(PdfLine {
                        text: String::new(),
                        size: 9.0,
                        spacing: 3.0,
                        bold: false,
                        mono: false,
                    });
                }
                Block::Quote(inner) => {
                    let text = prefix_lines(&blocks_to_text(inner), "> ");
                    push_lines(out, &text, 92, 10.0, 4.8, false, false);
                    out.push(PdfLine {
                        text: String::new(),
                        size: 10.0,
                        spacing: 3.0,
                        bold: false,
                        mono: false,
                    });
                }
                Block::List { ordered, items } => {
                    let text = list_to_text(items, *ordered);
                    push_lines(out, &text, 92, 10.0, 4.8, false, false);
                    out.push(PdfLine {
                        text: String::new(),
                        size: 10.0,
                        spacing: 3.0,
                        bold: false,
                        mono: false,
                    });
                }
                Block::Table { header, rows } => {
                    let text = table_to_text(header.as_ref(), rows);
                    push_lines(out, &text, 92, 9.0, 4.4, false, true);
                    out.push(PdfLine {
                        text: String::new(),
                        size: 9.0,
                        spacing: 3.0,
                        bold: false,
                        mono: false,
                    });
                }
                Block::HorizontalRule => out.push(PdfLine {
                    text: "────────────────────────────────────────".to_string(),
                    size: 10.0,
                    spacing: 6.0,
                    bold: false,
                    mono: false,
                }),
            }
        }
    }

    struct PageState {
        doc: PdfDocumentReference,
        pages: Vec<(PdfPageIndex, PdfLayerIndex)>,
        current: usize,
        y: Mm,
    }

    impl PageState {
        fn layer(&self) -> PdfLayerReference {
            let (pi, li) = self.pages[self.current];
            self.doc.get_page(pi).get_layer(li)
        }

        fn new_page(&mut self) {
            let (pi, li) = self.doc.add_page(PAGE_W, PAGE_H, "Layer 1".to_string());
            self.pages.push((pi, li));
            self.current = self.pages.len() - 1;
            self.y = Mm(PAGE_H.0 - MARGIN.0);
        }
    }

    let (pdf, first_page, first_layer) =
        PdfDocument::new("ntc fmt".to_string(), PAGE_W, PAGE_H, "Layer 1".to_string());
    let font_regular = pdf.add_builtin_font(BuiltinFont::Helvetica)?;
    let font_bold = pdf.add_builtin_font(BuiltinFont::HelveticaBold)?;
    let font_mono = pdf.add_builtin_font(BuiltinFont::Courier)?;

    let mut st = PageState {
        pages: vec![(first_page, first_layer)],
        current: 0,
        y: Mm(PAGE_H.0 - MARGIN.0),
        doc: pdf,
    };

    let mut lines = Vec::new();
    collect(&doc.blocks, &mut lines);

    for line in lines {
        if st.y.0 - line.spacing < BOTTOM {
            st.new_page();
        }
        let font = if line.mono {
            &font_mono
        } else if line.bold {
            &font_bold
        } else {
            &font_regular
        };
        st.layer()
            .use_text(line.text, line.size, MARGIN, st.y, font);
        st.y = Mm(st.y.0 - line.spacing);
    }

    st.doc.save(&mut BufWriter::new(File::create(path)?))?;
    Ok(())
}

#[cfg(test)]
mod fmt_doc_tests {
    use super::*;
    #[test]
    fn debug_xml_tree() {
        let input = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><w:document xmlns:w="http://x"><w:body><w:p><w:pPr><w:pStyle w:val="Heading1"/></w:pPr><w:r><w:t xml:space="preserve">Test</w:t></w:r></w:p></w:body></w:document>"#;
        let tree = parse_xml_tree(input).unwrap();
        for c in &tree {
            if let XmlChild::Elem(e) = c {
                assert_eq!(e.name, "w:document");
            }
        }

        // End-to-end docx round trip.
        let doc = Document {
            blocks: vec![
                Block::List {
                    ordered: false,
                    items: vec![vec![Block::Paragraph(vec![Inline::Text(
                        "item one".into(),
                    )])]],
                },
                Block::List {
                    ordered: true,
                    items: vec![vec![Block::Paragraph(vec![Inline::Text("first".into())])]],
                },
            ],
        };
        let tmp = std::env::temp_dir().join("ntc_fmttest_rt.docx");
        write_docx(&doc, &tmp).unwrap();
        let back = read_docx(&tmp).unwrap();
        println!("round trip blocks: {:#?}", back.blocks);
        assert!(matches!(back.blocks.first(), Some(Block::List { .. })));
        assert!(matches!(
            back.blocks.get(1),
            Some(Block::List { ordered: true, .. })
        ));
    }

    #[test]
    fn txt_parses_to_paragraphs_and_back() {
        let doc = doc_from_text("hello\nworld\n\nsecond block\n");
        assert_eq!(doc.blocks.len(), 2);
        let text = document_to_text(&doc);
        assert!(text.contains("hello\nworld"), "got:\n{}", text);
        assert!(text.contains("second block"), "got:\n{}", text);
    }

    #[test]
    fn txt_converts_to_markdown_html_and_xml() {
        let doc = doc_from_text("Title\n\nBody text");
        let md = serialize_document_text(&doc, "md").unwrap();
        assert!(md.contains("Body text"), "got:\n{}", md);
        let html = serialize_document_text(&doc, "html").unwrap();
        assert!(html.contains("<p>"), "got:\n{}", html);
        let xml = serialize_document_text(&doc, "xml").unwrap();
        assert!(xml.contains("<p>"), "got:\n{}", xml);
    }

    #[test]
    fn document_json_round_trips() {
        let doc = Document {
            blocks: vec![
                Block::Heading {
                    level: 2,
                    inlines: vec![Inline::Text("Title".into())],
                },
                Block::Paragraph(vec![
                    Inline::Text("Hello ".into()),
                    Inline::Bold(vec![Inline::Text("bold".into())]),
                ]),
                Block::List {
                    ordered: true,
                    items: vec![vec![Block::Paragraph(vec![Inline::Text("one".into())])]],
                },
                Block::Table {
                    header: Some(vec![vec![Inline::Text("a".into())]]),
                    rows: vec![vec![vec![Inline::Text("1".into())]]],
                },
                Block::HorizontalRule,
            ],
        };
        let json = document_to_json(&doc).unwrap();
        let back = doc_from_json(&json).unwrap();
        assert_eq!(doc, back, "json:\n{}", json);
    }

    #[cfg(not(target_os = "android"))]
    #[test]
    fn pdf_write_smoke() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("out.pdf");
        let doc = doc_from_text("Hello PDF\n\nSecond paragraph");
        write_pdf(&doc, &path).unwrap();
        let len = std::fs::metadata(&path).unwrap().len();
        assert!(len > 100, "pdf too small: {}", len);
    }
}
