use std::{fs::OpenOptions, io::Write};

pub fn save_html_content(data: Vec<String>, filename: &str) {
    for i in data {
        let line_break = i + "\n";
        let mut f = OpenOptions::new()
            .append(true)
            .create(true) // Optionally create the file if it doesn't already exist
            .open(filename)
            .expect("Unable to open file");
        f.write_all(line_break.as_bytes())
            .expect("Unable to write data");
    }
}
