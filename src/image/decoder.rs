use image::io::Reader as ImageReader;

#[derive(Debug)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>, // Données brutes (RGB)
}

pub fn decode_image(path: &str) -> Option<Image> {
    let reader = ImageReader::open(path).ok()?;
    let img = reader.decode().ok()?;
    let rgb = img.to_rgb8();
    Some(Image {
        width: rgb.width(),
        height: rgb.height(),
        data: rgb.into_raw(),
    })
}