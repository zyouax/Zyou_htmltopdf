use crate::image::decoder::Image;

pub fn embed_image(image: &Image, obj_id: usize) -> String {
    format!(
        "{} 0 obj\n<< /Type /XObject /Subtype /Image /Width {} /Height {} /ColorSpace /DeviceRGB /BitsPerComponent 8 /Length {} >>\nstream\n{}\nendstream\nendobj\n",
        obj_id,
        image.width,
        image.height,
        image.data.len(),
        String::from_utf8_lossy(&image.data)
    )
}