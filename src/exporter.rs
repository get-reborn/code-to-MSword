

use docx_rs::*;

pub(crate) struct Exporter {}

impl Exporter {
    pub fn export(base_path: &String, output_file_name: &String, context: &String) {
        let path = base_path.clone() + "/" + output_file_name;
        let path = std::path::Path::new(path.as_str());
        let file = std::fs::File::create(path).unwrap();
        let mut docx = Docx::new();
        for line in context.lines() {
            docx = docx.add_paragraph(Paragraph::new().add_run(Run::new().add_text(line)));
        }
        docx.build().pack(file).unwrap();
    }
}
