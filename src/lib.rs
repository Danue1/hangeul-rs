//! [![](http://meritbadge.herokuapp.com/hangeul)](https://crates.io/crates/hangeul)
//! 
//! [View on GitHub](https://github.com/bekker/hangeul-rs)
//!
//! Korean alphabet manipulation library for Rust.
//!
//! It is lightweight, without any unicode libraries.
//!
//! ```toml
//! [dependencies]
//! hangeul = "0.1.0"
//! ```
//!
//! ## Usage
//!
//! ```rust
//! extern crate hangeul;
//!
//! fn main() {
//!     let subjective = "피카츄";
//!     let sub_josa = match hangeul::ends_with_jongseong(subjective).unwrap() {
//!         true => "이",
//!         false => "가"
//!     };
//!     let sentence = format!("야생의 {}{} 나타났다!", subjective, sub_josa);
//!     println!("{}", sentence); // 야생의 피카츄가 나타났다!
//!     print_choseong(&sentence); // ㅇㅅㅇ ㅍㅋㅊㄱ ㄴㅌㄴㄷ!
//! }
//!
//! fn print_choseong(s:&str) {
//!     for c in s.chars() {
//!         print!("{}", hangeul::get_choseong(c).unwrap_or(c));
//!     }
//!     print!("\n");
//! }
//! ```
//!
//! ## Why not 'Hangul'?
//! 'Hangul' is from old romanization system - McCune–Reischauer system.
//!
//! 'Hangeul' is official in South Korea, since 2000
//!
//! ## License
//! Distributed under MIT License
//!

use std::fmt;
use std::error;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn decomposition() {
        let han = '한';
        let ha = '하';
        assert_eq!(get_choseong(han).unwrap(), 'ㅎ');
        assert_eq!(get_jungseong(han).unwrap(), 'ㅏ');
        assert_eq!(get_jongseong(han).unwrap(), 'ㄴ');
        assert_eq!(has_jongseong(han).unwrap(), true);
        assert_eq!(has_jongseong(ha).unwrap(), false);
        get_jongseong(ha).unwrap_err();
    }

    #[test]
    fn check_jamo() {
        assert_eq!(is_jamo('ㄱ'), true);
        assert_eq!(is_jamo('ㅣ'), true);
        assert_eq!(is_jamo('a'), false);
        assert_eq!(is_jaeum('ㄱ'), true);
        assert_eq!(is_jaeum('ㅎ'), true);
        assert_eq!(is_jaeum('ㅏ'), false);
        assert_eq!(is_choseong('ㄱ'), true);
        assert_eq!(is_choseong('ㅎ'), true);
        assert_eq!(is_choseong('ㄸ'), true);
        assert_eq!(is_choseong('ㄳ'), false);
        assert_eq!(is_choseong('ㅉ'), true);
        assert_eq!(is_choseong('ㅃ'), true);
        assert_eq!(is_choseong('ㅄ'), false);
        assert_eq!(is_choseong('\u{3130}'), false);
        assert_eq!(is_choseong('\u{314F}'), false);
        assert_eq!(is_jongseong('ㄱ'), true);
        assert_eq!(is_jongseong('ㅎ'), true);
        assert_eq!(is_jongseong('ㄸ'), false);
        assert_eq!(is_jongseong('ㄳ'), true);
        assert_eq!(is_jongseong('ㅉ'), false);
        assert_eq!(is_jongseong('ㅃ'), false);
        assert_eq!(is_jongseong('ㅄ'), true);
        assert_eq!(is_jongseong('A'), false);
        assert_eq!(is_jongseong('\u{3130}'), false);
        assert_eq!(is_jongseong('\u{314F}'), false);
    }
}

#[derive(Debug)]
pub enum HangeulError {
    NotSyllable,
    NoJongSeong,
}

impl fmt::Display for HangeulError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HangeulError::NotSyllable => write!(f, "HangeulError: Not correct Hangeul syllable"),
            HangeulError::NoJongSeong => write!(f, "HangeulError: The syllable has no jongseong")
        }
    }
}

impl error::Error for HangeulError {
    fn description(&self) -> &str {
        match *self {
            HangeulError::NotSyllable => "HangeulError: Not correct Hangeul syllable",
            HangeulError::NoJongSeong => "HangeulError: The syllable has no jongseong",
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            _ => None
        }
    }
}

const IS_CHOSEONG: [bool; 30] = [
    true,  // ㄱ 0x3131
    true,  // ㄲ
    false, // ㄳ
    true,  // ㄴ
    false, // ㄵ
    false, // ㄶ
    true,  // ㄷ
    true,  // ㄸ
    true,  // ㄹ
    false, // ㄺ
    false, // ㄻ
    false, // ㄼ
    false, // ㄽ
    false, // ㄾ
    false, // ㄿ
    false, // ㅀ
    true,  // ㅁ
    true,  // ㅂ
    true,  // ㅃ
    false, // ㅄ
    true,  // ㅅ
    true,  // ㅆ
    true,  // ㅇ
    true,  // ㅈ
    true,  // ㅉ
    true,  // ㅊ
    true,  // ㅋ
    true,  // ㅌ
    true,  // ㅍ
    true,  // ㅎ 0x314E
];

const IS_JONGSEONG: [bool; 30] = [
    true,  // ㄱ 0x3131
    true,  // ㄲ
    true,  // ㄳ
    true,  // ㄴ
    true,  // ㄵ
    true,  // ㄶ
    true,  // ㄷ
    false, // ㄸ
    true,  // ㄹ
    true,  // ㄺ
    true,  // ㄻ
    true,  // ㄼ
    true,  // ㄽ
    true,  // ㄾ
    true,  // ㄿ
    true,  // ㅀ
    true,  // ㅁ
    true,  // ㅂ
    false, // ㅃ
    true,  // ㅄ
    true,  // ㅅ
    true,  // ㅆ
    true,  // ㅇ
    true,  // ㅈ
    false, // ㅉ
    true,  // ㅊ
    true,  // ㅋ
    true,  // ㅌ
    true,  // ㅍ
    true,  // ㅎ 0x314E
];

// These tables are for converting to compatible jamo
const CHOSEONG_TABLE: [u32; 19] = [
    0x01, // ㄱ
    0x02, // ㄲ
    0x04, // ㄴ
    0x07, // ㄷ
    0x08, // ㄸ
    0x09, // ㄹ
    0x11, // ㅁ
    0x12, // ㅂ
    0x13, // ㅃ
    0x15, // ㅅ
    0x16, // ㅆ
    0x17, // ㅇ
    0x18, // ㅈ
    0x19, // ㅉ
    0x1A, // ㅊ
    0x1B, // ㅋ
    0x1C, // ㅌ
    0x1D, // ㅍ
    0x1E, // ㅎ
];

const JONGSEONG_TABLE: [u32; 27] = [
    0x01, // ㄱ
    0x02, // ㄲ
    0x03, // ㄳ
    0x04, // ㄴ
    0x05, // ㄵ
    0x06, // ㄶ
    0x07, // ㄷ
    0x09, // ㄹ
    0x0A, // ㄺ
    0x0B, // ㄻ
    0x0C, // ㄼ
    0x0D, // ㄽ
    0x0E, // ㄾ
    0x0F, // ㄿ
    0x10, // ㅀ
    0x11, // ㅁ
    0x12, // ㅂ
    0x14, // ㅄ
    0x15, // ㅅ
    0x16, // ㅆ
    0x17, // ㅇ
    0x18, // ㅈ
    0x1A, // ㅊ
    0x1B, // ㅋ
    0x1C, // ㅌ
    0x1D, // ㅍ
    0x1E, // ㅎ
];

fn _is_syllable(code:u32) -> bool {
    (code >= 0xAC00 && code <= 0xD7AF)
}

/// Check if the syllable is correct Hangeul syllable
pub fn is_syllable(c:char) -> bool {
    let code = c as u32;
    _is_syllable(code)
}

fn syllable_to_u32(c:char) -> Result<u32, HangeulError> {
    let code = c as u32;
    if _is_syllable(code) {
        Ok(code)
    } else {
        Err(HangeulError::NotSyllable)
    }
}

/// Get choseong (top) of the syllable as compatible jamo
pub fn get_choseong(c:char) -> Result<char, HangeulError> {
    let code = try!(syllable_to_u32(c));
    let x = (code - 0xAC00) / 21 / 28;
    // These unwrap()s are fail-safe
    Ok(std::char::from_u32(CHOSEONG_TABLE[x as usize] + 0x3130).unwrap())
}

/// Get jungseong (middle) of the syllable as compatible jamo
pub fn get_jungseong(c:char) -> Result<char, HangeulError> {
    let code = try!(syllable_to_u32(c));
    Ok(std::char::from_u32(((code - 0xAc00) % (21 * 28)) / 28 + 0x314F).unwrap())
}

/// Get jongseong (bottom) of the syllable as compatible jamo
pub fn get_jongseong(c:char) -> Result<char, HangeulError> {
    let code = try!(syllable_to_u32(c));
    // x should be i32, can be negative
    let x:i32 = (code - 0xAC00) as i32 % 28 - 1;
    if x >= 0 {
        Ok(std::char::from_u32(JONGSEONG_TABLE[x as usize] + 0x3130).unwrap())
    } else {
        Err(HangeulError::NoJongSeong)
    }
}

/// Check if the syllable has jongseong (bottom)
pub fn has_jongseong(c:char) -> Result<bool, HangeulError> {
    let code = try!(syllable_to_u32(c));
    Ok((code - 0xAC00) % 28 != 0)
}

/// Check if the end syllable of the string has jongseong (bottom)
pub fn ends_with_jongseong(s:&str) -> Result<bool, HangeulError> {
    let c = match s.chars().last() {
        Some(x) => x,
        None => return Err(HangeulError::NotSyllable),
    };
    has_jongseong(c)
}

/// Check if the char is compatible jamo
pub fn is_jamo(c:char) -> bool {
    let code = c as u32;
    (code >= 0x3131 && code <= 0x3163)
}

fn _is_jaeum(code:u32) -> bool {
    (code >= 0x3131 && code <= 0x314E)
}

/// Check if the char is compatible jaeum
pub fn is_jaeum(c:char) -> bool {
    let code = c as u32;
    _is_jaeum(code)
}

/// Check if the char is compatible jamo which can be a choseong (top)
pub fn is_choseong(c:char) -> bool {
    let code = c as u32;
    _is_jaeum(code) && IS_CHOSEONG[(code - 0x3131) as usize]
}

/// Check if the char is compatible jamo which can be a jongseong (bottom)
pub fn is_jongseong(c:char) -> bool {
    let code = c as u32;
    _is_jaeum(code) && IS_JONGSEONG[(code - 0x3131) as usize]
}
