use std::fs;

#[derive(Debug)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>, // Données brutes (RGB)
}

pub fn decode_image(path: &str) -> Option<Image> {
    // Simulation de décodage (PNG/JPEG simplifié)
    // Dans une implémentation réelle, utiliser un décodeur PNG/JPEG maison
    let data = fs::read(path).ok()?;
    Some(Image {
        width: 100, // Placeholder
        height: 100,
        data,
    })
}