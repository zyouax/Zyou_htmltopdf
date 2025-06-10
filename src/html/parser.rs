use crate::html::dom::{Node, NodeType};
use std::cell::RefCell;
use std::rc::Rc;

pub type NodeRef = Rc<RefCell<Node>>;

/// Parse un document HTML brut en un arbre DOM
pub fn parse_html(html: &str) -> NodeRef {
    let root = Rc::new(RefCell::new(Node {
        node_type: NodeType::Element("html".to_string()),
        attributes: vec![],
        children: vec![],
    }));

    let mut stack = vec![Rc::clone(&root)];
    let mut chars = html.chars().peekable();
    let mut buffer = String::new();

    while let Some(c) = chars.next() {
        match c {
            '<' => {
                flush_text_buffer(&mut buffer, stack.last());

                // Gestion des commentaires <!-- -->
                if chars.peek() == Some(&'!') {
                    let mut it = chars.clone();
                    it.next();
                    if it.next() == Some('-') && it.next() == Some('-') {
                        // Consommer '!-' '-'
                        chars.next();
                        chars.next();
                        chars.next();
                        let mut comment = String::new();
                        while let Some(ch) = chars.next() {
                            comment.push(ch);
                            if comment.ends_with("--") && chars.peek() == Some(&'>') {
                                comment.truncate(comment.len() - 2);
                                chars.next();
                                break;
                            }
                        }
                        if let Some(parent) = stack.last() {
                            parent.borrow_mut().children.push(Rc::new(RefCell::new(Node {
                                node_type: NodeType::Comment(comment),
                                attributes: vec![],
                                children: vec![],
                            })));
                        }
                        continue;
                    }
                }

                if chars.peek() == Some(&'/') {
                    chars.next(); // Skip '/'
                    let mut tag = String::new();
                    while let Some(ch) = chars.next() {
                        if ch == '>' {
                            break;
                        }
                        tag.push(ch);
                    }
                    stack.pop(); // Ferme la balise
                } else {
                    let (tag, attributes, self_closing) = parse_tag(&mut chars);
                    let node = Rc::new(RefCell::new(Node {
                        node_type: NodeType::Element(tag.clone()),
                        attributes,
                        children: vec![],
                    }));

                    if let Some(parent) = stack.last() {
                        parent.borrow_mut().children.push(Rc::clone(&node));
                    }

                    if !self_closing && !is_self_closing(&tag) {
                        stack.push(node);
                    }
                }
            }
            _ => buffer.push(c),
        }
    }

    flush_text_buffer(&mut buffer, stack.last());
    root
}

/// Ajoute le texte en attente comme nœud Text dans le DOM
fn flush_text_buffer(buffer: &mut String, parent: Option<&NodeRef>) {
    let text = decode_entities(buffer.trim());
    if !text.is_empty() {
        if let Some(p) = parent {
            p.borrow_mut().children.push(Rc::new(RefCell::new(Node {
                node_type: NodeType::Text(text),
                attributes: vec![],
                children: vec![],
            })));
        }
    }
    buffer.clear();
}

/// Parse une balise HTML ouvrante : tag, attributs, fermeture
fn parse_tag<I>(chars: &mut std::iter::Peekable<I>) -> (String, Vec<(String, String)>, bool)
where
    I: Iterator<Item = char> + Clone,
{
    let mut tag = String::new();
    let mut attributes = vec![];
    let mut self_closing = false;

    // Lire le nom de la balise
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() || c == '/' || c == '>' {
            break;
        }
        tag.push(c);
        chars.next();
    }

    // Lecture des attributs
    loop {
        // Ignorer les espaces
        while let Some(&c) = chars.peek() {
            if c.is_whitespace() {
                chars.next();
            } else {
                break;
            }
        }

        match chars.peek() {
            Some('>') => {
                chars.next();
                break;
            }
            Some('/') => {
                let mut it = chars.clone();
                it.next();
                if it.next() == Some('>') {
                    chars.next();
                    chars.next();
                    self_closing = true;
                    break;
                }
            }
            Some(_) => {
                let mut name = String::new();

                // Lire le nom de l’attribut
                while let Some(&c) = chars.peek() {
                    if c == '=' || c.is_whitespace() || c == '>' {
                        break;
                    }
                    name.push(c);
                    chars.next();
                }

                // Ignorer les espaces avant '='
                while let Some(&c) = chars.peek() {
                    if c.is_whitespace() {
                        chars.next();
                    } else {
                        break;
                    }
                }

                let mut value = String::new();

                if chars.peek() == Some(&'=') {
                    chars.next(); // skip '='

                    // Espaces après =
                    while let Some(&c) = chars.peek() {
                        if c.is_whitespace() {
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    if let Some(&quote) = chars.peek() {
                        if quote == '"' || quote == '\'' {
                            chars.next(); // skip quote
                            while let Some(ch) = chars.next() {
                                if ch == quote {
                                    break;
                                }
                                value.push(ch);
                            }
                        } else {
                            while let Some(&ch) = chars.peek() {
                                if ch.is_whitespace() || ch == '>' {
                                    break;
                                }
                                value.push(ch);
                                chars.next();
                            }
                        }
                    }
                }

                attributes.push((name.trim().to_string(), value.trim().to_string()));
            }
            None => break,
        }
    }

    (tag.trim().to_string(), attributes, self_closing)
}

/// Liste des balises HTML auto-fermantes
fn is_self_closing(tag: &str) -> bool {
    matches!(
        tag.to_ascii_lowercase().as_str(),
        "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" |
        "link" | "meta" | "param" | "source" | "track" | "wbr"
    )
}

/// Décodage des entités HTML
fn decode_entities(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '&' {
            let mut entity = String::new();
            while let Some(&next) = chars.peek() {
                if next == ';' {
                    chars.next();
                    break;
                }
                entity.push(next);
                chars.next();
            }

            let replacement = match entity.as_str() {
                "amp" => "&".to_string(),
                "lt" => "<".to_string(),
                "gt" => ">".to_string(),
                "quot" => "\"".to_string(),
                "apos" => "'".to_string(),
                "nbsp" => "\u{00A0}".to_string(),
                "copy" => "©".to_string(),
                "reg" => "®".to_string(),
                "euro" => "€".to_string(),
                "mdash" => "—".to_string(),
                "ndash" => "–".to_string(),
                "hellip" => "…".to_string(),
                "eacute" => "é".to_string(),
                _ if entity.starts_with("#x") => u32::from_str_radix(&entity[2..], 16)
                    .ok()
                    .and_then(std::char::from_u32)
                    .map(|c| c.to_string())
                    .unwrap_or_default(),
                _ if entity.starts_with('#') => entity[1..]
                    .parse::<u32>()
                    .ok()
                    .and_then(std::char::from_u32)
                    .map(|c| c.to_string())
                    .unwrap_or_default(),
                _ => String::new(),
            };

            result.push_str(&replacement);
        } else {
            result.push(c);
        }
    }

    result
}
