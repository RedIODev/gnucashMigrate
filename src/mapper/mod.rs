use std::fs;

use xml_bindings::GncV2;


mod xml_bindings;
pub mod wrappers;

pub fn parse_note_xml(xml_filename: &str) -> Result<GncV2, Box<dyn std::error::Error>> {
    let xml_content = fs::read_to_string(xml_filename)?;
    let note: GncV2 = quick_xml::de::from_str(&xml_content)?;
    Ok(note)
}