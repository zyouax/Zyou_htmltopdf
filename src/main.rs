use zyou_htmltopdf::{collect_stylesheets, compute_layout, parse_html, write_pdf};

fn main() {
    let html = std::fs::read_to_string("input.html").unwrap();
    let dom = parse_html(&html);
    let sheet = collect_stylesheets(&dom.borrow());
    let layout = compute_layout(&dom.borrow(), 595.0, 842.0, Some(&sheet));
    let pdf = write_pdf(&layout);
    std::fs::create_dir_all("output").unwrap();
    std::fs::write("output/output.pdf", pdf).unwrap();
}
