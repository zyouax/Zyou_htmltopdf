use crate::css::styles::Color;
use crate::layout::box_model::{BoxContent, LayoutBox};
use crate::image::decoder::decode_image;

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
    write_box(layout, &mut stream, &mut links);

    let mut pdf = Vec::new();
    let mut offsets = Vec::new();

    // PDF Header
    pdf.extend(b"%PDF-1.4\n% zyHTMLtoPDF\n");

    // Catalog
    offsets.push(pdf.len());
    pdf.extend(b"1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n");

    // Pages (1)
    let page_id = 3;
    let content_id = 4;

    // Pages (2)
    pdf.extend(
        format!(
            "2 0 obj\n<< /Type /Pages /Kids [{} 0 R] /Count 1 >>\nendobj\n",
            page_id
        )
        .as_bytes(),
    );

    // Pages (3)
    let annot_ids: Vec<String> = (0..links.len()).map(|i| format!("{} 0 R", 5 + i)).collect();
    offsets.push(pdf.len());
    pdf.extend(format!(
        "3 0 obj\n<< /Type /Page /Parent 2 0 R /Resources << /Font << /F1 << /Type /Font /Subtype /Type1 /BaseFont /Helvetica >> /F2 << /Type /Font /Subtype /Type1 /BaseFont /Times-Roman >> /F3 << /Type /Font /Subtype /Type1 /BaseFont /Courier >> >> >> /Contents {} 0 R /MediaBox [0 0 595 842]",
        content_id
    ).as_bytes());
    if !annot_ids.is_empty() {
        pdf.extend(format!(" /Annots [{}]", annot_ids.join(" ")).as_bytes());
    }
    pdf.extend(b" >>\nendobj\n");

    // Content stream (4)
    offsets.push(pdf.len());
    pdf.extend(format!("4 0 obj\n<< /Length {} >>\nstream\n", stream.len()).as_bytes());
    pdf.extend(&stream);
    pdf.extend(b"\nendstream\nendobj\n");
    // Annotation objects starting at 5
    for (i, l) in links.iter().enumerate() {
        let id = 5 + i;
        offsets.push(pdf.len());
        pdf.extend(
            format!(
                "{} 0 obj\n<< /Type /Annot /Subtype /Link /Rect [{} {} {} {}] /Border [0 0 0] /A << /S /URI /URI ({}) >> >>\nendobj\n",
                id, l.x1, l.y1, l.x2, l.y2, l.url
            )
            .as_bytes(),
        );
    }

    let obj_count = 5 + links.len();

    // Cross-reference
    let xref_offset = pdf.len();
    pdf.extend(format!("xref\n0 {}\n", obj_count).as_bytes());
    pdf.extend(b"0000000000 65535 f \n");
    for offset in offsets {
        pdf.extend(format!("{:010} 00000 n \n", offset).as_bytes());
    }

    // Trailer
    pdf.extend(
        format!(
            "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{}\n%%EOF\n",
            obj_count, xref_offset
        )
        .as_bytes(),
    );
    pdf
}

fn write_box(b: &LayoutBox, stream: &mut Vec<u8>, links: &mut Vec<LinkInfo>) {
    let y_rect = 842.0 - b.y - b.height;
    if let Some(bg) = &b.style.background {
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
            let Color { r, g, b: b_ } = b.style.color;
            let font = match b.style.font_family.as_deref() {
                Some("Times") | Some("Times-Roman") | Some("Times New Roman") => "F2",
                Some("Courier") => "F3",
                _ => "F1",
            };
            stream.extend(
                format!(
                    "BT\n/{} {} Tf\n{} {} Td\n{} {} {} rg\n({}) Tj\nET\n",
                    font,
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
                stream.extend(format!("q\n{} 0 0 {} {} {} cm\n", b.width, b.height, b.x, y_rect).as_bytes());
                stream.extend(format!("BI /W {} /H {} /CS /RGB /BPC 8 ID\n", img.width, img.height).as_bytes());
                stream.extend(&img.data);
                stream.extend(b"\nEI\nQ\n");
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
        write_box(child, stream, links);
    }
}

fn escape_text(text: &str) -> String {
    text.replace("(", "\\(").replace(")", "\\)")
}
