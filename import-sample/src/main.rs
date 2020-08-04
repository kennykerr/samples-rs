winrt::import!(
    dependencies
        os
    types
        windows::data::xml::dom::*
);

use windows::data::xml::dom::*;

fn main() -> winrt::Result<()> {
    let doc = XmlDocument::new()?;
    doc.load_xml("<html>hello world</html>")?;

    let root = doc.document_element()?;
    assert!(root.node_name()? == "html");
    assert!(root.inner_text()? == "hello world");

    Ok(())
}
