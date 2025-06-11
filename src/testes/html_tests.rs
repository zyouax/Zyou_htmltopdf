#[cfg(test)]
mod tests {
    use crate::html::parser::parse_html;

    #[test]
    fn test_parse_html() {
        let html = r#"<div style="color: red"><p>Hello</p><img src="test.png"/></div>"#;
        let dom = parse_html(html);
        let root = dom.borrow();
        assert_eq!(root.children.len(), 1);
        let first = root.children[0].borrow();
        assert_eq!(
            first.node_type,
            crate::html::dom::NodeType::Element("div".to_string())
        );
        assert_eq!(first.children.len(), 2);
    }

    #[test]
    fn test_single_quoted_attr() {
        let html = "<img src='img.png' alt='test'/>";
        let dom = parse_html(html);
        let root = dom.borrow();
        assert_eq!(root.children.len(), 1);
        let img = root.children[0].borrow();
        assert_eq!(img.get_attribute("src"), Some("img.png"));
        assert_eq!(img.get_attribute("alt"), Some("test"));
    }
}