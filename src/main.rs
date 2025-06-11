use std::fs;
use zyou_htmltopdf::{collect_stylesheets, compute_layout, parse_html, write_pdf};

#[derive(Debug)]
enum PdfError {
    Io(()),
}

impl From<std::io::Error> for PdfError {
    fn from(_err: std::io::Error) -> Self {
        PdfError::Io(())
    }
}

fn main() -> Result<(), PdfError> {
    let html = fs::read_to_string("input.html")?;
    let dom = parse_html(&html);
    let sheet = collect_stylesheets(&dom.borrow());
    let layout = compute_layout(&dom.borrow(), 595.0, 842.0, Some(&sheet));
    let pdf = write_pdf(&layout);
    fs::create_dir_all("output")?;
    fs::write("output/output.pdf", pdf)?;
    Ok(())
}