use std::ffi::{c_char, CStr};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum UrlDecodeError {
    #[error("invalid URL encoding: incomplete escape sequence")]
    IncompleteEscape,
    
    #[error("invalid hexadecimal digit in escape sequence")]
    InvalidHexDigit,
    
    #[error("invalid UTF-8 sequence in decoded bytes")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),
}

#[unsafe(no_mangle)]
pub extern "C" fn url_decode_c(
    input: *const  c_char,
    input_len: usize,
    output: *mut c_char,
    output_len: usize
)->usize{
    if input.is_null() || output.is_null(){
        return 0;
    }

    let c_str = unsafe { CStr::from_ptr(input) };
    let input_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return 0,
    };
    let input_slice = &input_str[..input_len.min(input_str.len())];
    // url decode
    let result = url_decode(input_slice);
    if result.is_err(){
        return 0;
    }

    let decoded = result.unwrap();
    let bytes = decoded.as_bytes();
    // leave empty character position
    let copy_len = bytes.len().min(output_len.saturating_sub(1));
    unsafe{
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), output as *mut u8, copy_len);
        *output.add(copy_len) = 0;
    }

    return copy_len;
}

pub fn url_decode(input: &str) -> Result<String, UrlDecodeError> {
    let mut output = Vec::with_capacity(input.len());
    let mut bytes = input.bytes().peekable();

    while let Some(byte) = bytes.next() {
        match byte {
            b'+' => output.push(b' '),
            b'%' => {
                let hex1 = bytes.next().ok_or(UrlDecodeError::IncompleteEscape)?;
                let hex2 = bytes.next().ok_or(UrlDecodeError::IncompleteEscape)?;

                let decoded = decode_hex_pair(hex1, hex2)?;
                output.push(decoded);
            }
            _ => output.push(byte),
        }
    }

    String::from_utf8(output).map_err(UrlDecodeError::InvalidUtf8)
}

fn decode_hex_pair(hex1: u8, hex2: u8) -> Result<u8, UrlDecodeError> {
    fn hex_digit_value(digit: u8) -> Result<u8, UrlDecodeError> {
        match digit {
            b'0'..=b'9' => Ok(digit - b'0'),
            b'A'..=b'F' => Ok(digit - b'A' + 10),
            b'a'..=b'f' => Ok(digit - b'a' + 10),
            _ => Err(UrlDecodeError::InvalidHexDigit),
        }
    }

    let high = hex_digit_value(hex1)?;
    let low = hex_digit_value(hex2)?;
    Ok((high << 4) | low)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_url_encoding() {
        let encoded = "Hello%2C%20World%21%2B%3D%25";
        let decoded = url_decode(encoded).unwrap();
        assert_eq!(decoded, "Hello, World!+=%");
    }

    #[test]
    fn test_invalid_hex_digit() {
        let encoded = "invalid%ZZ%sequence";
        let result = url_decode(encoded);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), UrlDecodeError::InvalidHexDigit));
    }
}  