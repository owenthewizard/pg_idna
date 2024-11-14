//! [WHATWG URL IDNA](https://url.spec.whatwg.org/#idna) helpers for `PostgreSQL`.

use std::str::FromStr;

use idna::uts46::Uts46;
use pgrx::prelude::*;

mod my_config;
use my_config::{MyAsciiDenyList, MyDnsLength, MyHyphens};

::pgrx::pg_module_magic!();

/// Checks if `input` is ASCII.
#[pg_extern]
const fn idna_is_ascii(input: &str) -> bool {
    input.is_ascii()
}

/// Checks if `input` is Punycode.
#[pg_extern]
fn idna_is_punycode(input: &str) -> bool {
    input.starts_with("xn--")
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
/// For more information, see [Uts46::to_ascii].
#[pg_extern]
fn idna_to_ascii(
    input: &str,
    adl: default!(&str, "'url'"),
    h: default!(&str, "'allow'"),
    dl: default!(&str, "'verify'"),
) -> String {
    let uts46 = Uts46::new();
    let adl = MyAsciiDenyList::from_str(adl).expect("invalid argument");
    let h = MyHyphens::from_str(h).expect("invalid argument");
    let dl = MyDnsLength::from_str(dl).expect("invalid argument");

    uts46
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
/// - If unspecified, the default [`AsciiDenyList`](idna::uts46::AsciiDenyList) is [URL](idna::uts46::AsciiDenyList::URL).
/// - If unspecified, the default [`Hyphens`](idna::uts46::Hyphens) is [Allow](idna::uts46::Hyphens::Allow).
///
/// For more information, see [Uts46::to_unicode].
#[pg_extern]
fn idna_to_unicode(
    input: &str,
    adl: default!(&str, "'url'"),
    h: default!(&str, "'allow'"),
) -> String {
    let uts46 = Uts46::new();
    let adl = MyAsciiDenyList::from_str(adl).expect("invalid argument");
    let h = MyHyphens::from_str(h).expect("invalid argument");

    idna_to_unicode_internal(input, &uts46, adl, h, false)
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
/// - If unspecified, the default [`AsciiDenyList`](idna::uts46::AsciiDenyList) is [URL](idna::uts46::AsciiDenyList::URL).
/// - If unspecified, the default [`Hyphens`](idna::uts46::Hyphens) is [Allow](idna::uts46::Hyphens::Allow).
///
/// For more information, see [`Uts46::to_unicode`].
#[pg_extern]
fn idna_to_unicode_lossy(
    input: &str,
    adl: default!(&str, "'url'"),
    h: default!(&str, "'allow'"),
) -> String {
    let uts46 = Uts46::new();
    let adl = MyAsciiDenyList::from_str(adl).expect("invalid argument");
    let h = MyHyphens::from_str(h).expect("invalid argument");

    idna_to_unicode_internal(input, &uts46, adl, h, true)
}

/// Internal function used to perform the `ToUnicode` conversion.
fn idna_to_unicode_internal(
    input: &str,
    uts46: &Uts46,
    adl: MyAsciiDenyList,
    h: MyHyphens,
    lossy: bool,
) -> String {
    let (out, res) = uts46.to_unicode(input.as_bytes(), *adl.inner(), *h.inner());

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
