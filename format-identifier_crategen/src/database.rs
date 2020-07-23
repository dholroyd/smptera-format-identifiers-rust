
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Record {
    #[serde(rename = "ASCII Value")]
    pub ascii_value: String,
    #[serde(rename = "Hex value")]
    pub hex_value: String,
    #[serde(rename = "Organization Name")]
    pub org_name: String,
    #[serde(rename = "Organization Address")]
    pub org_address: String,
    #[serde(rename = "Phone")]
    pub phone: String,
    #[serde(rename = "Fax")]
    pub fax: String,
    #[serde(rename = "Statement of intention to apply the assigned RID")]
    pub intention: String,
    #[serde(rename = "Date of intended implementation of the RID")]
    pub implementation_date: String,
    #[serde(rename = "Name of Authorized Representative")]
    pub representative_name: String,
    #[serde(rename = "Authorized Representative Title")]
    pub representative_title: String,
    #[serde(rename = "Authorized Representative Address")]
    pub representative_address: String,
    #[serde(rename = "Date Identifier Registered")]
    pub date_registered: String,
}

pub fn load(name: &Path) -> Result<Vec<Record>, csv::Error> {
    let input = std::fs::File::open(name)
        .expect(name.to_str().unwrap());
    let mut rdr = csv::Reader::from_reader(input);
    rdr.deserialize().collect()
}
