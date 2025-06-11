use crate::css::styles::Color;
use crate::image::decoder::decode_image;
use crate::layout::box_model::{BoxContent, LayoutBox};
use crate::pdf::fonts::{load_font, Font};
use crate::pdf::images::embed_image;
use std::collections::HashMap;

struct LinkInfo {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    url: String,
}

pub fn write_pdf(layout: &LayoutBox) -> Vec<u8> {
    let mut stream = Vec::new();
    let mut links = Vec::new();
    let mut alphas: Vec<(f32, String)> = Vec::new();
    let mut fonts: HashMap<String, Font> = HashMap::new();
    let mut images: Vec<(usize, String)> = Vec::new();
    let mut next_obj_id = 5;

    write_box(
        layout,
        &mut stream,
        &mut links,
        &mut images,
        &mut alphas,
        &mut fonts,
        &mut next_obj_id,
    );

    let mut pdf = Vec::new();
    let mut offsets = Vec::new();

    pdf.extend(b"%PDF-1.5\n% zyHTMLtoPDF\n");

    offsets.push(pdf.len());
    pdf.extend(b"1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n");

    let page_id = 3;
    let content_id = 4;

    offsets.push(pdf.len());
    pdf.extend(
        format!(
            "2 0 obj\n<< /Type /Pages /Kids [{} 0 R] /Count 1 >>\nendobj\n",
            page_id
        )
        .as_bytes(),
    );

    let ext_base = next_obj_id;
    let annot_base = ext_base + alphas.len();
    let font_base = annot_base + links.len();
    let image_base = font_base + fonts.len() * 2;

    offsets.push(pdf.len());
    let mut font_resources = String::new();
    let mut font_id = 1;
    for _font_name in fonts.keys() {
        font_resources.push_str(&format!(
            "/F{} {} 0 R ",
            font_id,
            font_base + (font_id - 1) * 2
        ));
        font_id += 1;
    }
    pdf.extend(
        format!(
            "3 0 obj\n<< /Type /Page /Parent 2 0 R /Resources << /Font << {} /F1 << /Type /Font /Subtype /Type1 /BaseFont /Helvetica >> /F2 << /Type /Font /Subtype /Type1 /BaseFont /Times-Roman >> /F3 << /Type /Font /Subtype /Type1 /BaseFont /Courier >> >>",
            font_resources
        )
        .as_bytes(),
    );
    if !alphas.is_empty() {
        let gs: Vec<String> = alphas
            .iter()
            .enumerate()
            .map(|(i, (_, name))| format!("/{} {} 0 R", name, ext_base + i))
            .collect();
        pdf.extend(format!(" /ExtGState << {} >>", gs.join(" ")).as_bytes());
    }
    if !images.is_empty() {
        let img_resources: Vec<String> = images
            .iter()
            .enumerate()
            .map(|(i, _)| format!("/Img{} {} 0 R", i + 1, image_base + i))
            .collect();
        pdf.extend(format!(" /XObject << {} >>", img_resources.join(" ")).as_bytes());
    }
    pdf.extend(
        format!(" >> /Contents {} 0 R /MediaBox [0 0 595 842]", content_id).as_bytes(),
    );
    if !links.is_empty() {
        let annot_ids: Vec<String> = (0..links.len())
            .map(|i| format!("{} 0 R", annot_base + i))
            .collect();
        pdf.extend(format!(" /Annots [{}]", annot_ids.join(" ")).as_bytes());
    }
    pdf.extend(b"\nendobj\n");

    offsets.push(pdf.len());
    pdf.extend(
        format!("4 0 obj\n<< /Length {} >>\nstream\n", stream.len()).as_bytes(),
    );
    pdf.extend(&stream);
    pdf.extend(b"\nendstream\nendobj\n");

    for (i, (alpha, _name)) in alphas.iter().enumerate() {
        let id = ext_base + i;
        offsets.push(pdf.len());
        pdf.extend(
            format!(
                "{} 0 obj\n<< /Type /ExtGState /ca {} /CA {} >>\nendobj\n",
                id, *alpha, *alpha
            )
            .as_bytes(),
        );
    }

    for (i, l) in links.iter().enumerate() {
        let id = annot_base + i;
        offsets.push(pdf.len());
        pdf.extend(
            format!(
                "{} 0 obj\n<< /Type /Annot /Subtype /Link /Rect [{} {} {} {}] /Border [0 0 0] /A << /S /URI /URI ({}) >> >>\nendobj\n",
                id, l.x1, l.y1, l.x2, l.y2, l.url
            )
            .as_bytes(),
        );
    }

    let mut font_obj_id = font_base;
    for (font_name, _font) in fonts.iter() {
        offsets.push(pdf.len());
        pdf.extend(
            format!(
                "{} 0 obj\n<< /Type /Font /Subtype /TrueType /BaseFont /{} /FontDescriptor {} 0 R >>\nendobj\n",
                font_obj_id, font_name, font_obj_id + 1
            )
            .as_bytes(),
        );

        offsets.push(pdf.len());
        pdf.extend(
            format!(
                "{} 0 obj\n<< /Type /FontDescriptor /FontName /{} /Flags 32 /FontBBox [-1000 -1000 2000 2000] /ItalicAngle 0 /Ascent 1000 /Descent -200 /CapHeight 800 /StemV 80 >>\nendobj\n",
                font_obj_id + 1, font_name
            )
            .as_bytes(),
        );
        font_obj_id += 2;
    }

    for (_id, img_obj) in &images {
        offsets.push(pdf.len());
        pdf.extend(img_obj.as_bytes());
    }

    let obj_count = image_base + images.len();

    let xref_offset = pdf.len();
    pdf.extend(format!("xref\n0 {}\n", obj_count).as_bytes());
    pdf.extend(b"0000000000 65535 f \n");
    for offset in offsets {
        pdf.extend(format!("{:010} 00000 n \n", offset).as_bytes());
    }

    pdf.extend(
        format!(
            "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{}\n%%EOF\n",
            obj_count, xref_offset
        )
        .as_bytes(),
    );
    pdf
}

fn write_box(
    b: &LayoutBox,
    stream: &mut Vec<u8>,
    links: &mut Vec<LinkInfo>,
    images: &mut Vec<(usize, String)>,
    alphas: &mut Vec<(f32, String)>,
    fonts: &mut HashMap<String, Font>,
    next_obj_id: &mut usize,
) {
    let y_rect = 842.0 - b.y - b.height;
    if let Some(bg) = &b.style.background {
        if bg.a < 1.0 {
            stream.extend(format!("/{} gs\n", ensure_alpha(bg.a, alphas)).as_bytes());
        }
        stream.extend(
            format!(
                "{} {} {} rg\n",
                bg.r as f32 / 255.0,
                bg.g as f32 / 255.0,
                bg.b as f32 / 255.0
            )
            .as_bytes(),
        );
        stream.extend(format!("{} {} {} {} re f\n", b.x, y_rect, b.width, b.height).as_bytes());
    }
    if b.style.border_width.top > 0.0 {
        let bw = b.style.border_width.top;
        stream.extend(b"0 0 0 RG\n");
        stream.extend(
            format!(
                "{} w\n{} {} {} {} re S\n",
                bw, b.x, y_rect, b.width, b.height
            )
            .as_bytes(),
        );
    }
    match &b.content {
        BoxContent::Text(text) => {
            let y = 842.0 - b.y - b.style.font_size;
            let Color { r, g, b: b_, a } = b.style.color;
            let font_name = b
                .style
                .font_family
                .as_deref()
                .unwrap_or("Helvetica")
                .to_string();
            let _font = if font_name != "Helvetica"
                && font_name != "Times-Roman"
                && font_name != "Times New Roman"
                && font_name != "Courier"
            {
                fonts
                    .entry(font_name.clone())
                    .or_insert_with(|| {
                        load_font(&format!("fonts/{}.ttf", font_name), b.style.font_size)
                            .unwrap_or_else(|_| default_font())
                    })
                    .clone()
            } else {
                default_font()
            };
            let font_ref = match font_name.as_str() {
                "Times" | "Times-Roman" | "Times New Roman" => "F2",
                "Courier" => "F3",
                _ => {
                    let font_id = fonts
                        .keys()
                        .position(|k| k == &font_name)
                        .map(|i| i + 1)
                        .unwrap_or(1);
                    &format!("F{}", font_id)[..]
                }
            };
            if a < 1.0 {
                stream.extend(format!("/{} gs\n", ensure_alpha(a, alphas)).as_bytes());
            }
            stream.extend(
                format!(
                    "BT\n/{} {} Tf\n{} {} Td\n{} {} {} rg\n({}) Tj\nET\n",
                    font_ref,
                    b.style.font_size,
                    b.x,
                    y,
                    r as f32 / 255.0,
                    g as f32 / 255.0,
                    b_ as f32 / 255.0,
                    escape_text(text)
                )
                .as_bytes(),
            );
        }
        BoxContent::Image(path) => {
            if let Some(img) = decode_image(path) {
                let obj_id = *next_obj_id;
                *next_obj_id += 1;
                images.push((obj_id, embed_image(&img, obj_id)));
                let img_index = images.len();
                stream.extend(
                    format!(
                        "q\n{} 0 0 {} {} {} cm\n/Img{} Do\nQ\n",
                        b.width, b.height, b.x, y_rect, img_index
                    )
                    .as_bytes(),
                );
            }
        }
        BoxContent::Element(_) => {}
    }

    if let Some(url) = &b.link {
        links.push(LinkInfo {
            x1: b.x,
            y1: y_rect,
            x2: b.x + b.width,
            y2: y_rect + b.height,
            url: url.clone(),
        });
    }

    for child in &b.children {
        write_box(child, stream, links, images, alphas, fonts, next_obj_id);
    }
}

fn escape_text(text: &str) -> String {
    let mut out = String::new();
    for ch in text.chars() {
        let c = match ch {
            '(' => "\\(".to_string(),
            ')' => "\\)".to_string(),
            '\\' => "\\\\".to_string(),
            _ => {
                if ch as u32 <= 0xFF {
                    (ch as u8 as char).to_string()
                } else {
                    "?".to_string()
                }
            }
        };
        out.push_str(&c);
    }
    out
}

fn ensure_alpha(value: f32, map: &mut Vec<(f32, String)>) -> String {
    if let Some((_, name)) = map.iter().find(|(v, _)| (*v - value).abs() < f32::EPSILON) {
        return name.clone();
    }
    let name = format!("GS{}", map.len() + 1);
    map.push((value, name.clone()));
    name
}

fn default_font() -> Font {
    Font {
        name: "Helvetica".to_string(),
        data: None,
        size: 12.0,
    }
}