use crate::css::styles::Style;

#[derive(Debug)]
pub struct LayoutBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub style: Style,
    pub link: Option<String>,
    pub content: BoxContent,
    pub children: Vec<LayoutBox>,
}

#[derive(Debug)]
pub enum BoxContent {
    Text(String),
    Element(String),
    Image(String), // Chemin de l'image
}