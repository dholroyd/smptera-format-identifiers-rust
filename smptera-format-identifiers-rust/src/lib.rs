//! [Format Identifier](https://smpte-ra.org/registered-mpeg-ts-ids) value on record with the
//! [SMPTE Registration Authority](https://smpte-ra.org/) for use in _MPEG Transport Stream_ data.
//!
//! (This create is automatically generated from a
//! [mirror of the SMPTE-RA data](https://github.com/dholroyd/smptera-format-identifiers)
//! by code in the
//! [smptera-format-identifiers-rust](https://github.com/dholroyd/smptera-format-identifiers-rust)
//! project.)
//!
//! The simple wrapper type [`FormatIdentifier`](struct.FormatIdentifier.html) defines a constant
//! for each _format identifier_ in the SMPTE-RA data,
//!
//! ```rust
//! # use smptera_format_identifers_rust::FormatIdentifier;
//! use std::io::stdout;
//! println!("{:?}", FormatIdentifier::AC_3);
//! // prints: FormatIdentifier(FourCC{AC-3})
//! ```
//!
//! ## Usage
//!
//! Create from a slice,
//!
//! ```rust
//! # use smptera_format_identifers_rust::FormatIdentifier;
//! let descriptor_data = b"\x05\x04CUEI";
//! let id = FormatIdentifier::from(&descriptor_data[2..6]);
//! assert_eq!(id, FormatIdentifier::CUEI);
//! ```
//!
//! Wrap an existing FourCC value,
//!
//! ```rust
//! # use smptera_format_identifers_rust::FormatIdentifier;
//! # use four_cc::FourCC;
//! let fcc = FourCC(*b"CUEI");
//! let id = FormatIdentifier(fcc);
//! assert_eq!(id, FormatIdentifier::CUEI);
//! ```
//!
//! Use provided constants in matches,
//!
//! ```rust
//! # use smptera_format_identifers_rust::FormatIdentifier;
//! # let descriptor_data = b"\x05\x04CUEI";
//! match FormatIdentifier::from(&descriptor_data[2..6]) {
//!     FormatIdentifier::CUEI => println!("SCTE-35 suspected"),
//!     FormatIdentifier::KLVA => println!("SMPTE RP 217-2001 KLV Packets?"),
//!     other_id => println!("Some other kinda stuff: {:?}", other_id),
//! }
//! ```
//!
//! Write bytes values,
//!
//! ```rust
//! # use smptera_format_identifers_rust::FormatIdentifier;
//! # use std::io::{Cursor, Write};
//! let mut data = vec![];
//! let mut io = Cursor::new(data);
//!
//! let id = &FormatIdentifier::ID3;
//! io.write(id.into())
//!     .expect("write failed");
//!
//! assert_eq!(io.into_inner(), [b'I', b'D', b'3', b' ']);
//! ```

use four_cc::FourCC;

/// Identifier for data formats used in MPEG Transport Streams
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FormatIdentifier(pub FourCC);

impl<'a> From<&'a[u8]> for FormatIdentifier {
    fn from(buf: &[u8]) -> Self {
        FormatIdentifier(FourCC::from(buf))
    }
}

impl<'a> From<&'a FormatIdentifier> for &'a[u8] {
    fn from(id: &'a FormatIdentifier) -> Self {
        &(id.0).0
    }
}

include!("generated.rs");

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
