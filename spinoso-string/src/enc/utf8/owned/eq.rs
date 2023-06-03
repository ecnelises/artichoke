use alloc::string::String;
use alloc::vec::Vec;

use bstr::BStr;

use super::{Utf8Str, Utf8String};

impl Eq for Utf8String {}

impl<'a, 'b> PartialEq<Utf8String> for Utf8String {
    fn eq(&self, other: &Utf8String) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl<'a, 'b> PartialEq<&'a Utf8String> for Utf8String {
    fn eq(&self, other: &&'a Utf8String) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl_partial_eq!(Utf8String, Utf8Str);
impl_partial_eq!(Utf8String, &'a Utf8Str);
impl_partial_eq!(Utf8String, &'a mut Utf8Str);
impl_partial_eq!(Utf8String, Vec<u8>);
impl_partial_eq!(Utf8String, &'a Vec<u8>);
impl_partial_eq!(Utf8String, [u8]);
impl_partial_eq!(Utf8String, &'a [u8]);
impl_partial_eq!(Utf8String, &'a mut [u8]);
impl_partial_eq!(Utf8String, BStr);
impl_partial_eq!(Utf8String, &'a BStr);
impl_partial_eq!(Utf8String, &'a mut BStr);
impl_partial_eq!(Utf8String, String);
impl_partial_eq!(Utf8String, &'a String);
impl_partial_eq!(Utf8String, str);
impl_partial_eq!(Utf8String, &'a str);
impl_partial_eq!(Utf8String, &'a mut str);
impl_partial_eq_array!(Utf8String, [u8; N]);
impl_partial_eq_array!(Utf8String, &'a [u8; N]);
impl_partial_eq_array!(Utf8String, &'a mut [u8; N]);
