pub fn config_file_template() -> String {
    let template = r#"{
  "websites": [
    {
      "id": "example",
      "name": "example of a website object",
      "save_file": "example.txt",
      "urls": [
        "https://example.com",
        "https://example2.com"
      ]
    },
    {
      "id": "example",
      "name": "example of a website object",
      "save_file": "example.csv",
      "urls": [
        "https://example.com",
        "https://example2.com"
      ]
    }
  ]
}
  "#;
    template.to_string()
}
