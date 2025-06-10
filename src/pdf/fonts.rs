pub struct Font {
    pub name: String,
    pub size: f32,
}

pub fn default_font() -> Font {
    Font {
        name: "Helvetica".to_string(),
        size: 12.0,
    }
}