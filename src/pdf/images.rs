use crate::image::decoder::Image;

pub fn embed_image(image: &Image) -> (String, Vec<u8>) {
    let obj = format!(
        "{} 0 obj\n<< /Type /XObject /Subtype /Image /Width {} /Height {} /ColorSpace /DeviceRGB /BitsPerComponent 8 /Length {} >>\nstream\n{}\nendstream\nendobj\n",
        "{}", // Placeholder pour l'ID
        image.width,
        image.height,
        image.data.len(),
        "{}" // Placeholder pour les données
    );
    (obj, image.data.clone())
}