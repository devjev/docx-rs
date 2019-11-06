#[macro_use]
mod macros;
mod core_properties;
mod declaration;
mod properties;
mod relationship;

use xml::common::XmlVersion;
use xml::writer::{EmitterConfig, EventWriter, XmlEvent};

pub struct XMLBuilder {
    writer: EventWriter<Vec<u8>>,
}

impl XMLBuilder {
    pub(crate) fn new() -> XMLBuilder {
        let buf = Vec::new();
        let writer = EmitterConfig::new()
            .write_document_declaration(false)
            .perform_indent(true)
            .create_writer(buf);
        XMLBuilder { writer }
    }

    // Build types element
    // i.e. <Types xmlns="http://...">
    pub(crate) fn open_types(mut self, uri: &str) -> Self {
        self.writer
            .write(XmlEvent::start_element("Types").attr("xmlns", uri))
            .expect("should write to buf");
        self
    }

    // Build Override element
    // i.e. <Override PartName="/_rels/.rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
    pub(crate) fn add_override(mut self, name: &str, content_type: &str) -> Self {
        self.writer
            .write(
                XmlEvent::start_element("Override")
                    .attr("PartName", &name)
                    .attr("ContentType", &content_type),
            )
            .expect("should write to buf");
        self.close()
    }

    // Close tag
    pub(crate) fn close(mut self) -> Self {
        self.writer
            .write(XmlEvent::end_element())
            .expect("should end");
        self
    }

    // Write plain text
    pub(crate) fn text(mut self, t: &str) -> Self {
        self.writer.write(t).unwrap();
        self
    }

    pub(crate) fn build(self) -> Vec<u8> {
        self.writer.into_inner()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::str;

    #[test]
    fn test_open_types() {
        let b = XMLBuilder::new();
        let r = b.open_types("http://example").text("child").close().build();
        assert_eq!(
            str::from_utf8(&r).unwrap(),
            r#"<Types xmlns="http://example">child</Types>"#
        );
    }
}