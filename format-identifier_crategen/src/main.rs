use std::path::Path;
use std::io::Write;
use crate::database::Record;

mod database;
mod generate;

fn main() {
    let mut records = database::load(Path::new("../smptera-format-identifiers/Public.csv"))
        .expect("Public.csv");
    fix_space(&mut records);
    let tokens = generate::gen(&records);
    let file = syn::parse2(tokens).unwrap();
    let formatted = prettyplease::unparse(&file);
    let warning = "// --------
// WARNING
// This is generated code.
// If you need changes, alter the format-identifier_crategen project, not this file.
// --------\n\n";
    let mut out = std::fs::File::create("../smptera-format-identifiers-rust/src/generated.rs").unwrap();
    out.write_all(warning.as_bytes()).expect("failed to write output");
    out.write_all(formatted.as_bytes()).expect("failed to write output");
}

fn fix_space(records: &mut [Record]) {
    // the tag "ID3" is in the CSV without a trailing space, decode the hex representations instead
    for r in records {
        if r.ascii_value.len() < 4 {
            r.ascii_value = decode_hex(&r.hex_value);
        }
    }
}

fn decode_hex(hex_value: &str) -> String {
    String::from_utf8(
        hex_value
            .split("-")
            .map(|s| u8::from_str_radix(s, 16).unwrap() )
            .collect::<Vec<u8>>()
    )
        .expect(&format!("Could not decode {:?}", hex_value))
        .to_string()
}
