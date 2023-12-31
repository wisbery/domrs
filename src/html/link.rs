use crate::HtmlElement;

/// A structure representing HTML `<link>` element.
#[derive(Debug, Default, Clone)]
pub struct HtmlLinkElement {
  rel: Option<String>,
  href: Option<String>,
}

impl HtmlLinkElement {
  pub fn rel(mut self, rel: &str) -> Self {
    self.rel = rel.to_string().into();
    self
  }

  pub fn href(mut self, href: &str) -> Self {
    self.href = href.to_string().into();
    self
  }

  pub fn stylesheet(mut self, href: &str) -> Self {
    self.rel = "stylesheet".to_string().into();
    self.href = href.to_string().into();
    self
  }
}

impl From<HtmlLinkElement> for HtmlElement {
  fn from(value: HtmlLinkElement) -> Self {
    let mut link = HtmlElement::new("link").hide_closing_tag();
    if let Some(href) = value.href {
      link.set_attribute("href", &href);
    }
    if let Some(rel) = value.rel {
      link.set_attribute("rel", &rel);
    }
    link
  }
}
