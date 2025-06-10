use super::box_model::{BoxContent, LayoutBox};
use crate::css::parser::parse_css;
use crate::css::styles::{Display, Style, Stylesheet};
use crate::html::dom::{Node, NodeType};

pub fn compute_layout(
    dom: &Node,
    page_width: f32,
    page_height: f32,
    sheet: Option<&Stylesheet>,
) -> LayoutBox {
    // Créer la boîte racine
    let root_style = parse_css(dom, sheet, None, None);
    let mut root = LayoutBox {
        x: 0.0,
        y: 0.0,
        width: page_width,
        height: page_height,
        style: root_style.clone(),
        link: None,
        content: BoxContent::Element("root".to_string()),
        children: vec![],
    };

    layout_children(
        dom,
        &mut root,
        10.0,
        10.0,
        page_width - 20.0,
        sheet,
        dom,
        &root_style,
    );

    root
}

fn layout_children(
    node: &Node,
    parent: &mut LayoutBox,
    start_x: f32,
    mut y_offset: f32,
    available_width: f32,
    sheet: Option<&Stylesheet>,
    parent_node: &Node,
    parent_style: &Style,
) {
    let mut x_inline = 0.0;
    for child_rc in &node.children {
        let child = child_rc.borrow();
        // Parser les styles CSS du nœud enfant
        if let NodeType::Element(tag) = &child.node_type {
            if tag == "style" || tag == "link" {
                continue;
            }
        }
        let style = parse_css(&child, sheet, Some(parent_node), Some(parent_style));
        let mut child_box = LayoutBox {
            x: 0.0,
            y: 0.0,
            // Largeur par défaut : page_width moins les marges
            width: style
                .width
                .unwrap_or(available_width - style.margin.left - style.margin.right),
            height: style.height.unwrap_or(20.0),
            style: style.clone(),
            link: if let NodeType::Element(t) = &child.node_type {
                if t == "a" {
                    child.get_attribute("href").map(|s| s.to_string())
                } else {
                    None
                }
            } else {
                None
            },
            content: match &child.node_type {
                NodeType::Text(t) => BoxContent::Text(t.clone()),
                NodeType::Element(tag) => {
                    if tag == "img" {
                        // Gérer les images via l'attribut src
                        BoxContent::Image(child.get_attribute("src").unwrap_or("").to_string())
                    } else {
                        BoxContent::Element(tag.clone())
                    }
                }
                NodeType::Comment(_) => continue,
            },
            children: vec![],
        };

        // Positionner la boîte selon le type d'affichage (block ou inline)
        match style.display {
            Display::Block => {
                child_box.x = start_x + style.margin.left;
                child_box.y = y_offset + style.margin.top;
                y_offset += child_box.height + style.margin.top + style.margin.bottom;
                x_inline = 0.0; // Réinitialiser pour les blocs
            }
            Display::Inline | Display::InlineBlock => {
                child_box.x = start_x + x_inline + style.margin.left;
                child_box.y = y_offset + style.margin.top;
                x_inline += child_box.width + style.margin.left + style.margin.right;
            }
            Display::None => continue,
        }

        if !child.children.is_empty() {
            let child_start_x = child_box.x + style.padding.left + style.border_width.left;
            let child_width = child_box.width
                - style.padding.left
                - style.padding.right
                - style.border_width.left
                - style.border_width.right;
            let child_y = child_box.y + style.padding.top + style.border_width.top;
            let child_style_clone = child_box.style.clone();
            layout_children(
                &child,
                &mut child_box,
                child_start_x,
                child_y,
                child_width,
                sheet,
                &child,
                &child_style_clone,
            );
        }

        parent.children.push(child_box);
    }
}
