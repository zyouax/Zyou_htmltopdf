use std::fs;

#[derive(Debug, Clone)]
pub struct Font {
    pub name: String,
    pub data: Option<Vec<u8>>,
    pub size: f32,
}

#[derive(Debug)]
pub enum FontError {
    IoError(std::io::Error),
    InvalidPath(String),
}

pub fn default_font() -> Font {
    Font {
        name: "Helvetica".to_string(),
        data: None,
        size: 12.0,
    }
}

pub fn load_font(path: &str, size: f32) -> Result<Font, FontError> {
    if path.is_empty() {
        return Err(FontError::InvalidPath("Le chemin du fichier de police est vide".to_string()));
    }

    let data = fs::read(path).map_err(FontError::IoError)?;
    let name = path
        .split('/')
        .last()
        .and_then(|s| s.strip_suffix(".ttf").or_else(|| s.strip_suffix(".otf")))
        .ok_or_else(|| FontError::InvalidPath("Nom de fichier invalide".to_string()))?
        .to_string();

    Ok(Font {
        name,
        data: Some(data),
        size,
    })
}