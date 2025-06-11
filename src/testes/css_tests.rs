#[cfg(test)]
mod tests {
    use crate::css::parser::{parse_css, parse_stylesheet};
    use crate::css::styles::Position;
    use crate::html::dom::{Node, NodeType};

    #[test]
    fn test_parse_css() {
        let node = Node {
            node_type: NodeType::Element("div".to_string()),
            attributes: vec![(
                "style".to_string(),
                "color: #ff0000; font-size: 16px; background: #00ff00; border: 2px".to_string(),
            )],
            children: vec![],
        };
        let style = parse_css(&node, None, None, None);
        assert_eq!(style.font_size, 16.0);
        assert_eq!(style.color.r, 255);
        assert_eq!(style.background.unwrap().g, 255);
        assert_eq!(style.border_width.top, 2.0);
    }

    #[test]
    fn test_stylesheet_priority() {
        let css = "div { color: #0000ff; } .title { font-size: 20px; } #main { color: #00ff00; }";
        let sheet = parse_stylesheet(css);
        let node = Node {
            node_type: NodeType::Element("div".to_string()),
            attributes: vec![
                ("class".to_string(), "title".to_string()),
                ("id".to_string(), "main".to_string()),
            ],
            children: vec![],
        };
        let style = parse_css(&node, Some(&sheet), None, None);
        assert_eq!(style.font_size, 20.0);
        assert_eq!(style.color.r, 0);
        assert_eq!(style.color.g, 255);
    }

    #[test]
    fn test_child_selector_and_inherit() {
        let css = "div > p { color: #ff0000; }";
        let sheet = parse_stylesheet(css);
        let parent = Node {
            node_type: NodeType::Element("div".to_string()),
            attributes: vec![("style".to_string(), "font-size: 14px".to_string())],
            children: vec![],
        };
        let child = Node {
            node_type: NodeType::Element("p".to_string()),
            attributes: vec![],
            children: vec![],
        };
        let parent_style = parse_css(&parent, Some(&sheet), None, None);
        let p_style = parse_css(&child, Some(&sheet), Some(&parent), Some(&parent_style));
        assert_eq!(p_style.color.r, 255);
        assert_eq!(p_style.font_size, 14.0);
    }

    #[test]
    fn test_font_family_parse() {
        let node = Node {
            node_type: NodeType::Element("p".to_string()),
            attributes: vec![("style".to_string(), "font-family: Courier".to_string())],
            children: vec![],
        };
        let style = parse_css(&node, None, None, None);
        assert_eq!(style.font_family.as_deref(), Some("Courier"));
    }

    #[test]
    fn test_position_parse() {
        let node = Node {
            node_type: NodeType::Element("div".to_string()),
            attributes: vec![(
                "style".to_string(),
                "position: absolute; top: 10px; left: 20px".to_string(),
            )],
            children: vec![],
        };
        let style = parse_css(&node, None, None, None);
        assert_eq!(style.position, Position::Absolute);
        assert_eq!(style.top, Some(10.0));
        assert_eq!(style.left, Some(20.0));
    }
}