
use std::{i16, primitive, str};
#[test]
fn tst_hex_string() {
    let s = "FFFE00010003FFFE00010003FFFE00010003FFFE00010003";
    const HEX16_SIZE: usize = 4;
    let subs = s.as_bytes()
        .chunks(HEX16_SIZE)
        .map(str::from_utf8)
        .map(|s| i32::from_str_radix(s.unwrap(), 16).unwrap() as i16)
        //.map(|s| s.unwrap().to_string().parse::<i16>())
        //.collect::<Result<Vec<i16>, _>>()
        .collect::<Vec<i16>>();
    
    println!("{:?}", subs);

    let z1 = i32::from_str_radix("FFFE", 16);
    let z2 = u32::from_str_radix("FE", 16);
    println!("--> {} ", z1.unwrap() as i16);

}

fn foo_ref(s: &String) {
    println!("FOO {} ", s);
}

fn foo_ref2(s: &str) {
    println!("FOO {} ", s);
}
#[test]
fn test_string_ref() {
    let s = String::from("ABC");
    foo_ref(&s);
    foo_ref2(s.as_str());

    println!("BAR {} ", s);

}