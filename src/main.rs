use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};

const CSV_FILENAME: &'static str = "sample.csv";

fn main() {
    // Register handlebars template
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("header_template", "./header.hbs.h")
        .unwrap();
    // Clean up output directory
    fs::remove_dir_all("output").ok();
    fs::create_dir("output").expect("Couldn't create the output directory");
    let mut output_file = File::create("output/header.h").expect("Couldn't create header file");

    // Read from CSV
    let file_contents =
        fs::read_to_string(CSV_FILENAME).expect("Something went wrong reading the file");

    // Parse into Struct format
    let mut items = vec![];
    let mut rdr = csv::Reader::from_reader(file_contents.as_bytes());
    for result in rdr.deserialize() {
        let row: Row = result.expect("Couldn't deserialize row");
        items.push(row);
    }

    let template_context = TemplateContext::new(items);

    // Render in template to file
    handlebars
        .render_to_write("header_template", &template_context, &mut output_file)
        .expect("Couldn't write template to file");

    println!("Finished!");
}

#[derive(Debug, Deserialize, Serialize)]
struct Row {
    part_id: i64,
    some_value: String,
    some_other_value: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct TemplateContext {
    items: Vec<Row>,
}

impl TemplateContext {
    fn new(items: Vec<Row>) -> TemplateContext {
        TemplateContext { items }
    }
}
