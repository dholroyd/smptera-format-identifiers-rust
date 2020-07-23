//! [Format Identifier](https://smpte-ra.org/registered-mpeg-ts-ids) value on record with the
//! [SMPTE Registration Authority](https://smpte-ra.org/) for use in _MPEG Transport Stream_ data.
//!
//! (This create is automatically generated from a
//! [mirror of the SMPTE-RA data](https://github.com/dholroyd/smptera-format-identifiers)
//! by code in the
//! [smptera-format-identifiers-rust](https://github.com/dholroyd/smptera-format-identifiers-rust)
//! project.)
//!
//! There are two parts to this crate,
//!  - The `format_identifier` module contains constants for each registered four-character-code
//!    value.
//!  - The `FormatIdentifier` enum contains a variant for each registered four-character-code
//!    value, plus the `Unknown(FourCC)` variant, that can be used to represent _Format Identifiers_
//!    which are not currently registered.
//!
//! ## Warning: `Unknown` and upgrading crate version
//!
//! Suppose you match against the `FormatIdentifier::Unknown` variant in your code as follows,
//!
//! ```
//! # use smptera_format_identifers_rust::FormatIdentifier;
//! # use four_cc::FourCC;
//! # let format_id = FormatIdentifier::Ac3;
//! # fn its_ac3_time() {}
//! # fn do_this_other_thing() {}
//!
//! match format_id {
//!     FormatIdentifier::Ac3 => its_ac3_time(),
//!     // .. match additional interesting ids here ..
//!     FormatIdentifier::Unknown(fourcc) if fourcc==FourCC(*b"othr") => do_this_other_thing(),
//!     _ => {}, // ignore anything else
//! }
//! ```
//!
//! you should be aware that if the unknown FourCC value you match against gets registered with
//! SMPTE-RA and added to a future release of this crate, then the code above could silently stop
//! working, since the new enum variant (maybe `FormatIdentifier::Othr` in this example) will
//! now fall into the 'ignored' match arm rather than the `Unknown` one.

include!("generated.rs");

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
