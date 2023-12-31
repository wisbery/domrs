use domrs::HtmlDocument;

mod attributes;
mod document;
mod elements;
mod head;
mod link;

// Test files for HTML document
const D001: &str = include_str!("files/D001.html");
const D002: &str = include_str!("files/D002.html");
const D003: &str = include_str!("files/D003.html");
const D004: &str = include_str!("files/D004.html");
const D005: &str = include_str!("files/D005.html");
const D006: &str = include_str!("files/D006.html");
const D007: &str = include_str!("files/D007.html");
const D008: &str = include_str!("files/D008.html");
const D009: &str = include_str!("files/D009.html");
const D010: &str = include_str!("files/D010.html");
const D011: &str = include_str!("files/D011.html");
const D012: &str = include_str!("files/D012.html");

// Test files for <head> element inside HTML document
const H001: &str = include_str!("files/H001.html");
const H002: &str = include_str!("files/H002.html");
const H003: &str = include_str!("files/H003.html");
const H004: &str = include_str!("files/H004.html");
const H005: &str = include_str!("files/H005.html");
const H006: &str = include_str!("files/H006.html");

// Test files for HTML elements
const E001: &str = include_str!("files/E001.html");
const E002: &str = include_str!("files/E002.html");

/// Utility function for comparing HTML documents.
fn eq(expected: &str, html: HtmlDocument) {
  assert_eq!(expected, html.to_string());
}

/// Utility function that return default HTML document.
fn doc() -> HtmlDocument {
  HtmlDocument::default().default_doctype().default_language().default_namespace()
}
