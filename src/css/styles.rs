use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Style {
    pub display: Display,
    pub margin: Sides,
    pub padding: Sides,
    pub border_width: Sides,
    pub font_size: f32,
    pub color: Color,
    pub background: Option<Color>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub font_family: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Display {
    Block,
    Inline,
    InlineBlock,
    None,
}

impl Default for Display {
    fn default() -> Self {
        Display::Block
    }
}

#[derive(Debug, Clone, Default)]
pub struct Sides {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

impl Default for Color {
    fn default() -> Self {
        Color { r: 0, g: 0, b: 0, a: 1.0 }
    }
}

pub type Stylesheet = HashMap<String, Style>;
