use super::styles::{Color, Display, Position, Sides, Style, Stylesheet};
use crate::html::dom::{Node, NodeType};

pub fn parse_css(
    node: &Node,
    sheet: Option<&Stylesheet>,
    parent_node: Option<&Node>,
    parent_style: Option<&Style>,
) -> Style {
    let mut style = Style::default();

    if let NodeType::Element(tag) = &node.node_type {
        if let Some(sheet) = sheet {
            if let Some(s) = sheet.get(tag) {
                merge_styles(&mut style, s);
            }
            if let Some(parent_node) = parent_node {
                if let NodeType::Element(p_tag) = &parent_node.node_type {
                    let selector = format!("{} > {}", p_tag, tag);
                    if let Some(s) = sheet.get(&selector) {
                        merge_styles(&mut style, s);
                    }
                }
            }
            if let Some(class_attr) = node.get_attribute("class") {
                for c in class_attr.split_whitespace() {
                    if let Some(s) = sheet.get(&format!(".{}", c)) {
                        merge_styles(&mut style, s);
                    }
                }
            }
            if let Some(id) = node.get_attribute("id") {
                if let Some(s) = sheet.get(&format!("#{}", id)) {
                    merge_styles(&mut style, s);
                }
            }
        }

        match tag.as_str() {
            "img" | "span" | "a" | "strong" | "em" | "b" | "i" | "u" | "small" | "abbr"
            | "code" | "kbd" | "mark" | "s" | "sub" | "sup" | "var" | "time" | "cite" | "q" => {
                style.display = Display::InlineBlock;
            }
            "p" | "div" | "section" | "article" | "aside" | "main" | "nav" | "header"
            | "footer" | "address" => {
                style.display = Display::Block;
                style.margin = Sides {
                    top: 10.0,
                    bottom: 10.0,
                    left: 0.0,
                    right: 0.0,
                };
            }
            "h1" => {
                style.display = Display::Block;
                style.font_size = 32.0;
                style.margin = Sides {
                    top: 20.0,
                    bottom: 10.0,
                    left: 0.0,
                    right: 0.0,
                };
            }
            "h2" => {
                style.display = Display::Block;
                style.font_size = 28.0;
                style.margin = Sides {
                    top: 18.0,
                    bottom: 10.0,
                    left: 0.0,
                    right: 0.0,
                };
            }
            "h3" => {
                style.display = Display::Block;
                style.font_size = 24.0;
            }
            "h4" => {
                style.display = Display::Block;
                style.font_size = 20.0;
            }
            "h5" => {
                style.display = Display::Block;
                style.font_size = 18.0;
            }
            "h6" => {
                style.display = Display::Block;
                style.font_size = 16.0;
            }
            "form" => {
                style.display = Display::Block;
                style.margin = Sides {
                    top: 10.0,
                    bottom: 10.0,
                    left: 0.0,
                    right: 0.0,
                };
            }
            "ul" | "ol" => {
                style.display = Display::Block;
                style.margin = Sides {
                    top: 10.0,
                    bottom: 10.0,
                    left: 20.0,
                    right: 0.0,
                };
            }
            "li" => {
                style.display = Display::Block;
                style.margin = Sides {
                    top: 4.0,
                    bottom: 4.0,
                    left: 10.0,
                    right: 0.0,
                };
            }
            "iframe" => {
                style.display = Display::Block;
                style.margin = Sides {
                    top: 10.0,
                    bottom: 10.0,
                    left: 0.0,
                    right: 0.0,
                };
            }
            "table" => {
                style.display = Display::Block;
                style.margin = Sides::default();
            }
            "caption" | "colgroup" | "col" | "thead" | "tbody" | "tfoot" | "tr" => {
                style.display = Display::Block;
            }
            "td" | "th" => {
                style.display = Display::InlineBlock;
                style.padding = Sides {
                    top: 4.0,
                    bottom: 4.0,
                    left: 6.0,
                    right: 6.0,
                };
                style.border_width = Sides {
                    top: 1.0,
                    bottom: 1.0,
                    left: 1.0,
                    right: 1.0,
                };
            }
            "input" | "label" | "textarea" | "select" | "option" | "button" => {
                style.display = Display::InlineBlock;
                style.margin = Sides {
                    top: 4.0,
                    bottom: 4.0,
                    left: 2.0,
                    right: 2.0,
                };
            }
            "video" | "audio" | "canvas" => {
                style.display = Display::Block;
                style.margin = Sides {
                    top: 10.0,
                    bottom: 10.0,
                    left: 0.0,
                    right: 0.0,
                };
            }
            _ => {}
        }
    }

    if let Some(css) = node.get_attribute("style") {
        apply_declarations(css, &mut style);
    }

    if let Some(parent) = parent_style {
        if style.font_size == 0.0 {
            style.font_size = parent.font_size;
        }
        if style.color == Color::default() {
            style.color = parent.color.clone();
        }
        if style.font_family.is_none() {
            style.font_family = parent.font_family.clone();
        }
    }

    style
}

pub fn parse_stylesheet(css: &str) -> Stylesheet {
    let mut sheet = Stylesheet::new();
    for rule in css.split('}') {
        let rule = rule.trim();
        if rule.is_empty() {
            continue;
        }
        if let Some((selector, body)) = rule.split_once('{') {
            let mut style = Style::default();
            apply_declarations(body, &mut style);
            sheet.insert(selector.trim().to_string(), style);
        }
    }
    sheet
}

pub fn collect_stylesheets(node: &Node) -> Stylesheet {
    fn collect(node: &Node, sheet: &mut Stylesheet) {
        if let NodeType::Element(tag) = &node.node_type {
            if tag == "style" {
                let mut css_text = String::new();
                for child in &node.children {
                    if let NodeType::Text(t) = &child.borrow().node_type {
                        css_text.push_str(t);
                    }
                }
                for (sel, st) in parse_stylesheet(&css_text) {
                    sheet.insert(sel, st);
                }
            } else if tag == "link" {
                if node.get_attribute("rel") == Some("stylesheet") {
                    if let Some(href) = node.get_attribute("href") {
                        if let Ok(content) = std::fs::read_to_string(href) {
                            for (sel, st) in parse_stylesheet(&content) {
                                sheet.insert(sel, st);
                            }
                        }
                    }
                }
            }
        }
        for child in &node.children {
            collect(&child.borrow(), sheet);
        }
    }

    let mut sheet = Stylesheet::new();
    collect(node, &mut sheet);
    sheet
}

fn merge_styles(base: &mut Style, other: &Style) {
    base.display = other.display.clone();
    base.margin = other.margin.clone();
    base.padding = other.padding.clone();
    base.border_width = other.border_width.clone();
    if other.font_size != 0.0 {
        base.font_size = other.font_size;
    }
    if other.color != Color::default() {
        base.color = other.color.clone();
    }
    if let Some(bg) = &other.background {
        base.background = Some(bg.clone());
    }
    if other.width.is_some() {
        base.width = other.width;
    }
    if other.height.is_some() {
        base.height = other.height;
    }
    if other.font_family.is_some() {
        base.font_family = other.font_family.clone();
    }
    base.position = other.position.clone();
    base.top = other.top;
    base.left = other.left;
}

fn apply_declarations(css: &str, style: &mut Style) {
    for decl in css.split(';').filter(|s| !s.trim().is_empty()) {
        let parts: Vec<&str> = decl.split(':').map(|s| s.trim()).collect();
        if parts.len() != 2 {
            continue;
        }
        let property = parts[0];
        let value = parts[1];
        match property {
            "display" => {
                style.display = match value {
                    "inline" => Display::Inline,
                    "inline-block" => Display::InlineBlock,
                    "none" => Display::None,
                    _ => Display::Block,
                };
            }
            "margin" => style.margin = parse_sides(value),
            "padding" => style.padding = parse_sides(value),
            "border" | "border-width" => style.border_width = parse_sides(value),
            "font-size" => {
                if let Some(size) = parse_unit(value) {
                    style.font_size = size;
                }
            }
            "font-family" => {
                let clean = value.trim_matches(['"', '\''].as_ref());
                style.font_family = Some(clean.to_string());
            }
            "color" => style.color = parse_color(value),
            "background" | "background-color" => style.background = Some(parse_color(value)),
            "width" => style.width = parse_unit(value),
            "height" => style.height = parse_unit(value),
            "position" => {
                style.position = match value {
                    "relative" => Position::Relative,
                    "absolute" => Position::Absolute,
                    _ => Position::Static,
                };
            }
            "top" => style.top = parse_unit(value),
            "left" => style.left = parse_unit(value),
            _ => {}
        }
    }
}

fn parse_sides(value: &str) -> Sides {
    let values: Vec<f32> = value.split_whitespace().filter_map(parse_unit).collect();
    match values.len() {
        1 => Sides {
            top: values[0],
            right: values[0],
            bottom: values[0],
            left: values[0],
        },
        2 => Sides {
            top: values[0],
            right: values[1],
            bottom: values[0],
            left: values[1],
        },
        4 => Sides {
            top: values[0],
            right: values[1],
            bottom: values[2],
            left: values[3],
        },
        _ => Sides::default(),
    }
}

fn parse_unit(value: &str) -> Option<f32> {
    if value.ends_with("px") {
        value.trim_end_matches("px").parse().ok()
    } else if value.ends_with("pt") {
        value.trim_end_matches("pt").parse().ok()
    } else if value.ends_with('%') {
        value.trim_end_matches('%').parse().ok()
    } else if value.ends_with("em") {
        value.trim_end_matches("em").parse().ok()
    } else if value.ends_with("rem") {
        value.trim_end_matches("rem").parse().ok()
    } else {
        value.parse().ok()
    }
}

fn parse_color(value: &str) -> Color {
    if value.starts_with('#') {
        if value.len() == 7 {
            if let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&value[1..3], 16),
                u8::from_str_radix(&value[3..5], 16),
                u8::from_str_radix(&value[5..7], 16),
            ) {
                return Color { r, g, b, a: 1.0 };
            }
        } else if value.len() == 9 {
            if let (Ok(r), Ok(g), Ok(b), Ok(a)) = (
                u8::from_str_radix(&value[1..3], 16),
                u8::from_str_radix(&value[3..5], 16),
                u8::from_str_radix(&value[5..7], 16),
                u8::from_str_radix(&value[7..9], 16),
            ) {
                return Color { r, g, b, a: a as f32 / 255.0 };
            }
        }
    } else if value.starts_with("rgba(") && value.ends_with(')') {
        let inner = &value[5..value.len() - 1];
        let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
        if parts.len() == 4 {
            if let (Ok(r), Ok(g), Ok(b), Ok(a)) = (
                parts[0].parse::<u8>(),
                parts[1].parse::<u8>(),
                parts[2].parse::<u8>(),
                parts[3].parse::<f32>(),
            ) {
                return Color { r, g, b, a };
            }
        }
    }
    Color::default()
}