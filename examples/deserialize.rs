use std::{fs::File, io::BufReader};

use msc_citypage::SiteData;
use xsd_parser::quick_xml::{DeserializeSync, IoReader, XmlReader};

/// An example of deserializing a SiteData XML.
fn main() {
    let input_file = File::open("examples/data.xml").unwrap();
    let reader = BufReader::new(input_file);
    let mut reader = IoReader::new(reader).with_error_info();

    let site_data = SiteData::deserialize(&mut reader).unwrap();

    println!("Site data: {:#?}", site_data);
}
