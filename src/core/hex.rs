use std::{i16, str};
use anyhow::*;

const HEX16_SIZE: usize = 4;


// fn parse_i16(v: &[u8]) -> anyhow::Result<i16> {
//     return str::from_utf8(v)
//         .map(|s| i32::from_str_radix(s, 16).unwrap() as i16)
// }
pub fn i16s_from_hex(hex: &str) -> Result<Vec<i16>> {
    return Ok(hex.as_bytes()
        .chunks(HEX16_SIZE)
        .map(str::from_utf8)
        .map(|s| i32::from_str_radix(s.unwrap(), 16).unwrap() as i16)
        .collect::<Vec<i16>>());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_i16s_from_hex() {
        assert_eq!(i16s_from_hex("FFFE00010003FFFE00010003FFFE00010003FFFE00010003").unwrap(),
                  vec![-2, 1, 3, -2, 1, 3, -2, 1, 3, -2, 1, 3]);
    }

    #[test]

    fn test_i16s_from_invalid_code() {
        //println!(" {:?}", i16s_from_hex("ZZZZ").unwrap_err());
        // assert_eq!(i16s_from_hex("ZZZZ").unwrap_err().to_string(), "ParseIntError");
        //assert!(i16s_from_hex("ZZZZ").is_err());
    }


}

