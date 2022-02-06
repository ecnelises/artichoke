use alloc::vec::Vec;
use core::fmt;
use core::ops::Range;
use core::slice::SliceIndex;

use bstr::{BStr, ByteSlice, ByteVec};

use crate::codepoints::InvalidCodepointError;
use crate::encoding::Encoding;
use crate::iter::{Bytes, IntoIter, Iter, IterMut};
use crate::ord::OrdError;

#[derive(Default, Clone)]
pub struct BinaryString {
    inner: Vec<u8>,
}

// Constructors
impl BinaryString {
    pub fn new(buf: Vec<u8>) -> Self {
        Self { inner: buf }
    }
}

impl fmt::Debug for BinaryString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Utf8String")
            .field("buf", &self.as_bstr())
            .field("encoding", &Encoding::Binary)
            .finish()
    }
}

// Debug
impl BinaryString {
    #[inline]
    #[must_use]
    pub fn as_bstr(&self) -> &BStr {
        self.inner.as_bstr()
    }
}

// Raw
impl BinaryString {
    #[inline]
    #[must_use]
    pub fn as_vec(&self) -> &Vec<u8> {
        &self.inner
    }

    #[inline]
    #[must_use]
    pub fn as_mut_vec(&mut self) -> &mut Vec<u8> {
        &mut self.inner
    }

    #[inline]
    #[must_use]
    pub fn into_vec(self) -> Vec<u8> {
        self.inner
    }

    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        &self.inner
    }

    #[inline]
    #[must_use]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.inner
    }

    #[inline]
    #[must_use]
    pub fn as_ptr(&self) -> *const u8 {
        self.inner.as_ptr()
    }

    #[inline]
    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.inner.as_mut_ptr()
    }
}

// Core Iterators
impl BinaryString {
    #[inline]
    #[must_use]
    pub fn iter(&self) -> Iter<'_> {
        Iter(self.inner.iter())
    }

    #[inline]
    #[must_use]
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        IterMut(self.inner.iter_mut())
    }

    #[inline]
    #[must_use]
    pub fn bytes(&self) -> Bytes<'_> {
        Bytes(self.inner.iter())
    }

    #[inline]
    #[must_use]
    pub fn into_iter(self) -> IntoIter {
        IntoIter(self.inner.into_iter())
    }
}

// Size and Capacity
impl BinaryString {
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    pub unsafe fn set_len(&mut self, len: usize) {
        self.inner.set_len(len);
    }

    #[inline]
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[inline]
    pub fn truncate(&mut self, len: usize) {
        self.inner.truncate(len);
    }

    #[inline]
    #[must_use]
    pub fn char_len(&self) -> usize {
        self.len()
    }
}

// Memory management
impl BinaryString {
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    #[inline]
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), alloc::collections::TryReserveError> {
        self.inner.try_reserve(additional)
    }

    #[inline]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.inner.reserve_exact(additional);
    }

    #[inline]
    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), alloc::collections::TryReserveError> {
        self.inner.try_reserve_exact(additional)
    }

    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit();
    }

    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.inner.shrink_to(min_capacity);
    }
}

// Indexing
impl BinaryString {
    #[inline]
    #[must_use]
    pub fn get<I>(&self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<[u8]>,
    {
        self.inner.get(index)
    }

    #[inline]
    #[must_use]
    pub fn get_char(&self, index: usize) -> Option<&'_ [u8]> {
        self.get(index..=index)
    }

    #[inline]
    #[must_use]
    pub fn get_char_slice(&self, range: Range<usize>) -> Option<&'_ [u8]> {
        let Range { start, end } = range;

        self.inner.get(start..end).or_else(|| self.inner.get(start..))
    }

    #[inline]
    #[must_use]
    pub fn get_mut<I>(&mut self, index: I) -> Option<&mut I::Output>
    where
        I: SliceIndex<[u8]>,
    {
        self.inner.get_mut(index)
    }

    #[inline]
    #[must_use]
    pub unsafe fn get_unchecked<I>(&self, index: I) -> &I::Output
    where
        I: SliceIndex<[u8]>,
    {
        self.inner.get_unchecked(index)
    }

    #[inline]
    #[must_use]
    pub unsafe fn get_unchecked_mut<I>(&mut self, index: I) -> &mut I::Output
    where
        I: SliceIndex<[u8]>,
    {
        self.inner.get_unchecked_mut(index)
    }
}

// Pushing and popping bytes, codepoints, and strings.
impl BinaryString {
    #[inline]
    pub fn push_byte(&mut self, byte: u8) {
        self.inner.push_byte(byte);
    }

    #[inline]
    pub fn try_push_codepoint(&mut self, codepoint: i64) -> Result<(), InvalidCodepointError> {
        if let Ok(byte) = u8::try_from(codepoint) {
            self.push_byte(byte);
            Ok(())
        } else {
            Err(InvalidCodepointError::codepoint_out_of_range(codepoint))
        }
    }

    #[inline]
    pub fn push_char(&mut self, ch: char) {
        self.inner.push_char(ch);
    }

    #[inline]
    pub fn push_str(&mut self, s: &str) {
        self.inner.push_str(s);
    }

    #[inline]
    pub fn extend_from_slice(&mut self, other: &[u8]) {
        self.inner.extend_from_slice(other);
    }
}

// Encoding
impl BinaryString {
    #[inline]
    #[must_use]
    pub fn is_ascii_only(&self) -> bool {
        self.inner.is_ascii()
    }

    #[allow(clippy::unused_self)]
    #[inline]
    #[must_use]
    pub fn is_valid_encoding(&self) -> bool {
        true
    }
}

// Casing
impl BinaryString {
    #[inline]
    pub fn make_capitalized(&mut self) {
        if let Some((head, tail)) = self.inner.split_first_mut() {
            head.make_ascii_uppercase();
            tail.make_ascii_lowercase();
        }
    }

    #[inline]
    pub fn make_lowercase(&mut self) {
        self.inner.make_ascii_lowercase();
    }

    #[inline]
    pub fn make_uppercase(&mut self) {
        self.inner.make_ascii_uppercase();
    }
}

impl BinaryString {
    #[inline]
    #[must_use]
    pub fn chr(&self) -> &[u8] {
        self.inner.get(0..1).unwrap_or_default()
    }

    #[inline]
    pub fn ord(&self) -> Result<u32, OrdError> {
        let byte = self.inner.get(0).copied().ok_or_else(OrdError::empty_string)?;
        Ok(u32::from(byte))
    }

    #[inline]
    #[must_use]
    pub fn ends_with(&self, slice: &[u8]) -> bool {
        self.inner.ends_with(slice)
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use quickcheck::quickcheck;

    use super::BinaryString;

    quickcheck! {
        #[allow(clippy::needless_pass_by_value)]
        fn fuzz_char_len_utf8_contents_binary_string(contents: alloc::string::String) -> bool {
            let expected = contents.len();
            let s = BinaryString::new(contents.into_bytes());
            s.char_len() == expected
        }

        #[allow(clippy::needless_pass_by_value)]
        fn fuzz_len_utf8_contents_binary_string(contents: alloc::string::String) -> bool {
            let expected = contents.len();
            let s = BinaryString::new(contents.into_bytes());
            s.len() == expected
        }

        #[allow(clippy::needless_pass_by_value)]
        fn fuzz_char_len_binary_contents_binary_string(contents: Vec<u8>) -> bool {
            let expected = contents.len();
            let s = BinaryString::new(contents);
            s.char_len() == expected
        }

        #[allow(clippy::needless_pass_by_value)]
        fn fuzz_len_binary_contents_binary_string(contents: Vec<u8>) -> bool {
            let expected = contents.len();
            let s = BinaryString::new(contents);
            s.len() == expected
        }
    }

    #[test]
    fn constructs_empty_buffer() {
        let s = BinaryString::new(Vec::new());
        assert_eq!(0, s.len());
    }

    #[test]
    fn casing_binary_string_empty() {
        let mut s = BinaryString::new(b"".to_vec());

        s.make_capitalized();
        assert_eq!(s, "");

        s.make_lowercase();
        assert_eq!(s, "");

        s.make_uppercase();
        assert_eq!(s, "");
    }

    #[test]
    fn casing_binary_string_ascii() {
        let lower = BinaryString::new(b"abc".to_vec());
        let mid_upper = BinaryString::new(b"aBc".to_vec());
        let upper = BinaryString::new(b"ABC".to_vec());
        let long = BinaryString::new(b"aBC, 123, ABC, baby you and me girl".to_vec());

        let capitalize: fn(&BinaryString) -> BinaryString = |value: &BinaryString| {
            let mut value = value.clone();
            value.make_capitalized();
            value
        };
        let lowercase: fn(&BinaryString) -> BinaryString = |value: &BinaryString| {
            let mut value = value.clone();
            value.make_lowercase();
            value
        };
        let uppercase: fn(&BinaryString) -> BinaryString = |value: &BinaryString| {
            let mut value = value.clone();
            value.make_uppercase();
            value
        };

        assert_eq!(capitalize(&lower), "Abc");
        assert_eq!(capitalize(&mid_upper), "Abc");
        assert_eq!(capitalize(&upper), "Abc");
        assert_eq!(capitalize(&long), "Abc, 123, abc, baby you and me girl");

        assert_eq!(lowercase(&lower), "abc");
        assert_eq!(lowercase(&mid_upper), "abc");
        assert_eq!(lowercase(&upper), "abc");
        assert_eq!(lowercase(&long), "abc, 123, abc, baby you and me girl");

        assert_eq!(uppercase(&lower), "ABC");
        assert_eq!(uppercase(&mid_upper), "ABC");
        assert_eq!(uppercase(&upper), "ABC");
        assert_eq!(uppercase(&long), "ABC, 123, ABC, BABY YOU AND ME GIRL");
    }

    #[test]
    fn casing_binary_string_utf8() {
        let sharp_s = BinaryString::from("ß");
        let tomorrow = BinaryString::from("αύριο");
        let year = BinaryString::from("έτος");
        // two-byte characters
        // https://github.com/minimaxir/big-list-of-naughty-strings/blob/894882e7/blns.txt#L198-L200
        let two_byte_chars = BinaryString::from("𐐜 𐐔𐐇𐐝𐐀𐐡𐐇𐐓 𐐙𐐊𐐡𐐝𐐓/𐐝𐐇𐐗𐐊𐐤𐐔 𐐒𐐋𐐗 𐐒𐐌 𐐜 𐐡𐐀𐐖𐐇𐐤𐐓𐐝 𐐱𐑂 𐑄 𐐔𐐇𐐝𐐀𐐡𐐇𐐓 𐐏𐐆𐐅𐐤𐐆𐐚𐐊𐐡𐐝𐐆𐐓𐐆");
        // Changes length when case changes
        // https://github.com/minimaxir/big-list-of-naughty-strings/blob/894882e7/blns.txt#L226-L232
        let varying_length = BinaryString::from("zȺȾ");
        let rtl = BinaryString::from("مرحبا الخرشوف");

        let capitalize: fn(&BinaryString) -> BinaryString = |value: &BinaryString| {
            let mut value = value.clone();
            value.make_capitalized();
            value
        };
        let lowercase: fn(&BinaryString) -> BinaryString = |value: &BinaryString| {
            let mut value = value.clone();
            value.make_lowercase();
            value
        };
        let uppercase: fn(&BinaryString) -> BinaryString = |value: &BinaryString| {
            let mut value = value.clone();
            value.make_uppercase();
            value
        };

        assert_eq!(capitalize(&sharp_s), "ß");
        assert_eq!(capitalize(&tomorrow), "αύριο");
        assert_eq!(capitalize(&year), "έτος");
        assert_eq!(
            capitalize(&two_byte_chars),
            "𐐜 𐐔𐐇𐐝𐐀𐐡𐐇𐐓 𐐙𐐊𐐡𐐝𐐓/𐐝𐐇𐐗𐐊𐐤𐐔 𐐒𐐋𐐗 𐐒𐐌 𐐜 𐐡𐐀𐐖𐐇𐐤𐐓𐐝 𐐱𐑂 𐑄 𐐔𐐇𐐝𐐀𐐡𐐇𐐓 𐐏𐐆𐐅𐐤𐐆𐐚𐐊𐐡𐐝𐐆𐐓𐐆"
        );
        assert_eq!(capitalize(&varying_length), "ZȺȾ");
        assert_eq!(capitalize(&rtl), "مرحبا الخرشوف");

        assert_eq!(lowercase(&sharp_s), "ß");
        assert_eq!(lowercase(&tomorrow), "αύριο");
        assert_eq!(lowercase(&year), "έτος");
        assert_eq!(
            lowercase(&two_byte_chars),
            "𐐜 𐐔𐐇𐐝𐐀𐐡𐐇𐐓 𐐙𐐊𐐡𐐝𐐓/𐐝𐐇𐐗𐐊𐐤𐐔 𐐒𐐋𐐗 𐐒𐐌 𐐜 𐐡𐐀𐐖𐐇𐐤𐐓𐐝 𐐱𐑂 𐑄 𐐔𐐇𐐝𐐀𐐡𐐇𐐓 𐐏𐐆𐐅𐐤𐐆𐐚𐐊𐐡𐐝𐐆𐐓𐐆"
        );
        assert_eq!(lowercase(&varying_length), "zȺȾ");
        assert_eq!(lowercase(&rtl), "مرحبا الخرشوف");

        assert_eq!(uppercase(&sharp_s), "ß");
        assert_eq!(uppercase(&tomorrow), "αύριο");
        assert_eq!(uppercase(&year), "έτος");
        assert_eq!(
            uppercase(&two_byte_chars),
            "𐐜 𐐔𐐇𐐝𐐀𐐡𐐇𐐓 𐐙𐐊𐐡𐐝𐐓/𐐝𐐇𐐗𐐊𐐤𐐔 𐐒𐐋𐐗 𐐒𐐌 𐐜 𐐡𐐀𐐖𐐇𐐤𐐓𐐝 𐐱𐑂 𐑄 𐐔𐐇𐐝𐐀𐐡𐐇𐐓 𐐏𐐆𐐅𐐤𐐆𐐚𐐊𐐡𐐝𐐆𐐓𐐆"
        );
        assert_eq!(uppercase(&varying_length), "ZȺȾ");
        assert_eq!(uppercase(&rtl), "مرحبا الخرشوف");
    }

    #[test]
    fn casing_binary_string_invalid_utf8() {
        let mut s = BinaryString::new(b"\xFF\xFE".to_vec());

        s.make_capitalized();
        assert_eq!(s, &b"\xFF\xFE"[..]);

        s.make_lowercase();
        assert_eq!(s, &b"\xFF\xFE"[..]);

        s.make_uppercase();
        assert_eq!(s, &b"\xFF\xFE"[..]);
    }

    #[test]
    fn casing_binary_string_unicode_replacement_character() {
        let mut s = BinaryString::from("�");
        s.make_capitalized();
        assert_eq!(s, "�");
    }
}
