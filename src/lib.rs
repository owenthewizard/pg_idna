//! [IDNA](https://crates.io/crates/idna) helpers for `PostgreSQL`.

use std::str::FromStr;

use idna::uts46::Uts46;
use pgrx::prelude::*;

mod my_config;
use my_config::{MyAsciiDenyList, MyDnsLength, MyHyphens};

::pgrx::pg_module_magic!();

const UTS46: Uts46 = Uts46::new();

/// Checks if `input` is ASCII.
#[pg_extern]
#[must_use]
pub const fn idna_is_ascii(input: &str) -> bool {
    input.is_ascii()
}

/// Copyright (c) 2013-2022 The rust-url developers
///
/// This code was copied from [rust-url](https://crates.io/crates/url)
/// and is used under the terms of the
/// [MIT license](https://github.com/servo/rust-url/blob/main/LICENSE-MIT).
const PUNYCODE_PREFIX: u32 =
    ((b'-' as u32) << 24) | ((b'-' as u32) << 16) | ((b'N' as u32) << 8) | b'X' as u32;

/// Copyright (c) 2013-2022 The rust-url developers
///
/// This code was copied from [rust-url](https://crates.io/crates/url)
/// and is used under the terms of the
/// [MIT license](https://github.com/servo/rust-url/blob/main/LICENSE-MIT).
const PUNYCODE_PREFIX_MASK: u32 = (0xFF << 24) | (0xFF << 16) | (0xDF << 8) | 0xDF;

/// Checks if `input` is Punycode.
///
/// This is merely a hint, as it only checks for the `xn--` prefix.
/// Thus, invalid punycode, or even non-ASCII, could return `true`.
///
/// Copyright (c) 2013-2022 The rust-url developers
///
/// This code was copied from [rust-url](https://crates.io/crates/url)
/// and is used under the terms of the
/// [MIT license](https://github.com/servo/rust-url/blob/main/LICENSE-MIT).
#[pg_extern]
#[must_use]
#[allow(clippy::many_single_char_names)]
pub const fn idna_is_punycode(slice: &[u8]) -> bool {
    if slice.len() < 4 {
        return false;
    }
    // Sadly, the optimizer doesn't figure out that more idiomatic code
    // should compile to masking on 32-bit value.
    let a = slice[0];
    let b = slice[1];
    let c = slice[2];
    let d = slice[3];
    // `as` instead of From for `const`
    let u = ((d as u32) << 24) | ((c as u32) << 16) | ((b as u32) << 8) | (a as u32);
    (u & PUNYCODE_PREFIX_MASK) == PUNYCODE_PREFIX
}

/// Encodes `input` as ASCII Punycode.
///
/// # Panics
///
/// This function will panic under any of the following conditions:
/// - `input` is not well-formed UTF-8.
/// - [`AsciiDenyList`](MyAsciiDenyList), [`Hyphens`](MyHyphens), or [`DnsLength`](MyDnsLength) are invalid.
/// - The [`AsciiDenyList`](idna::uts46::AsciiDenyList), [`Hyphens`](idna::uts46::Hyphens), or [`DnsLength`](idna::uts46::DnsLength) constraints are violated.
///
/// # Defaults
///
/// - If unspecified, the default [`AsciiDenyList`](idna::uts46::AsciiDenyList) is [`URL`](idna::uts46::AsciiDenyList::URL).
/// - If unspecified, the default [`Hyphens`](idna::uts46::Hyphens) is [`Allow`](idna::uts46::Hyphens::Allow).
/// - If unspecified, the default [`DnsLength`](idna::uts46::DnsLength) is [`Verify`](idna::uts46::DnsLength::Verify).
///
/// For more information, see [`Uts46::to_ascii`].
#[pg_extern]
#[must_use]
pub fn idna_to_ascii(
    input: &str,
    adl: default!(&str, "'url'"),
    h: default!(&str, "'allow'"),
    dl: default!(&str, "'verify'"),
) -> String {
    let adl = MyAsciiDenyList::from_str(adl).expect("invalid argument");
    let h = MyHyphens::from_str(h).expect("invalid argument");
    let dl = MyDnsLength::from_str(dl).expect("invalid argument");

    UTS46
        .to_ascii(input.as_bytes(), *adl.inner(), *h.inner(), *dl.inner())
        .expect("ToAscii conversion failed")
        .into_owned()
}

/// Encodes `input` as UTF-8.
///
/// # Panics
///
/// This function will panic under any of the following conditions:
/// - `input` is not well-formed UTF-8.
/// - `input` Punycode is invalid.
/// - [`AsciiDenyList`](MyAsciiDenyList) or [`Hyphens`](MyHyphens) are invalid.
/// - The [`AsciiDenyList`](idna::uts46::AsciiDenyList) or [`Hyphens`](idna::uts46::Hyphens) constraints are violated.
///
/// # Defaults
///
/// - If unspecified, the default [`AsciiDenyList`](idna::uts46::AsciiDenyList) is [`URL`](idna::uts46::AsciiDenyList::URL).
/// - If unspecified, the default [`Hyphens`](idna::uts46::Hyphens) is [`Allow`](idna::uts46::Hyphens::Allow).
///
/// For more information, see [`Uts46::to_unicode`].
#[pg_extern]
#[must_use]
pub fn idna_to_unicode(
    input: &str,
    adl: default!(&str, "'url'"),
    h: default!(&str, "'allow'"),
) -> String {
    let adl = MyAsciiDenyList::from_str(adl).expect("invalid argument");
    let h = MyHyphens::from_str(h).expect("invalid argument");

    idna_to_unicode_internal(input, adl, h, false)
}

/// Attemps to encode `input` as UTF-8.
///
/// If `input` is not valid Punycode, errors are denoted by U+FFFD REPLACEMENT CHARACTERs in the output.
///
/// # Panics
///
/// This function will panic under any of the following conditions:
/// - [`AsciiDenyList`](MyAsciiDenyList) or [`Hyphens`](MyHyphens) are invalid.
/// - The [`AsciiDenyList`](idna::uts46::AsciiDenyList) or [`Hyphens`](idna::uts46::Hyphens) constraints are violated.
///
/// # Defaults
///
/// - If unspecified, the default [`AsciiDenyList`](idna::uts46::AsciiDenyList) is [`URL`](idna::uts46::AsciiDenyList::URL).
/// - If unspecified, the default [`Hyphens`](idna::uts46::Hyphens) is [`Allow`](idna::uts46::Hyphens::Allow).
///
/// For more information, see [`Uts46::to_unicode`].
#[pg_extern]
#[must_use]
pub fn idna_to_unicode_lossy(
    input: &str,
    adl: default!(&str, "'url'"),
    h: default!(&str, "'allow'"),
) -> String {
    let adl = MyAsciiDenyList::from_str(adl).expect("invalid argument");
    let h = MyHyphens::from_str(h).expect("invalid argument");

    idna_to_unicode_internal(input, adl, h, true)
}

/// Internal function used to perform the `ToUnicode` conversion.
#[must_use]
fn idna_to_unicode_internal(
    input: &str,
    adl: MyAsciiDenyList,
    h: MyHyphens,
    lossy: bool,
) -> String {
    let (out, res) = UTS46.to_unicode(input.as_bytes(), *adl.inner(), *h.inner());

    if res.is_ok() || lossy {
        out.into_owned()
    } else {
        panic!("ToUnicode conversion failed");
    }
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {

    /*
    #[pg_test]
    fn test_hello_pg_idna() {
        assert_eq!("Hello, pg_idna", crate::hello_pg_idna());
    }
    */
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
