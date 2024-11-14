//! Internal module for [`idna`] newtypes.
//!
//! Required for [`FromStr`] impl.

use std::io;
use std::str::FromStr;

use idna::uts46::{AsciiDenyList, DnsLength, Hyphens};

// TODO: macro/strum these if possible

/// [`AsciiDenyList`] newtype.
// these should always match the inner type's
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct MyAsciiDenyList(AsciiDenyList);

impl MyAsciiDenyList {
    #[allow(dead_code)]
    pub const fn new(adl: AsciiDenyList) -> Self {
        Self(adl)
    }

    pub const fn inner(&self) -> &AsciiDenyList {
        &self.0
    }
}

impl FromStr for MyAsciiDenyList {
    type Err = io::Error;

    fn from_str(input: &str) -> io::Result<Self> {
        match input {
            "empty" => Ok(Self(AsciiDenyList::EMPTY)),
            "std3" => Ok(Self(AsciiDenyList::STD3)),
            "url" => Ok(Self(AsciiDenyList::URL)),
            _ => Err(io::Error::from(io::ErrorKind::InvalidInput)),
        }
    }
}

/// [`Hyphens`] newtype.
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct MyHyphens(Hyphens);

impl MyHyphens {
    #[allow(dead_code)]
    pub const fn new(h: Hyphens) -> Self {
        Self(h)
    }

    pub const fn inner(&self) -> &Hyphens {
        &self.0
    }
}

impl FromStr for MyHyphens {
    type Err = io::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "allow" => Ok(Self(Hyphens::Allow)),
            "check_first_last" => Ok(Self(Hyphens::CheckFirstLast)),
            "check" => Ok(Self(Hyphens::Check)),
            _ => Err(io::Error::from(io::ErrorKind::InvalidInput)),
        }
    }
}

/// [`DnsLength`] newtype.
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct MyDnsLength(DnsLength);

impl MyDnsLength {
    #[allow(dead_code)]
    pub const fn new(dl: DnsLength) -> Self {
        Self(dl)
    }

    pub const fn inner(&self) -> &DnsLength {
        &self.0
    }
}

impl FromStr for MyDnsLength {
    type Err = io::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "ignore" => Ok(Self(DnsLength::Ignore)),
            "verify_allow_root_dot" => Ok(Self(DnsLength::VerifyAllowRootDot)),
            "verify" => Ok(Self(DnsLength::Verify)),
            _ => Err(io::Error::from(io::ErrorKind::InvalidInput)),
        }
    }
}
