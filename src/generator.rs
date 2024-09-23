use std::fs;
use quick_xml::reader::Reader;
use xml_schema_generator::{into_struct, Options};

pub fn generate_structs_from_xml(xml_filename: &str) -> String {
    let xml_content = fs::read_to_string(xml_filename)
        .unwrap_or_else(|_| panic!("Unable to read XML file: {}", xml_filename));

    let mut reader = Reader::from_str(&xml_content);

    match into_struct(&mut reader) {
        Ok(root) => root.to_serde_struct(&Options::quick_xml_de()),
        Err(e) => panic!("Error generating structs: {:?}", e),
    }
}