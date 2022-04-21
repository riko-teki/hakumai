use std::{env::args, ops::{Shr, Shl}};

const BASE64_TABLE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn main() {
    let args: Vec<String> = args().collect();
    let byte = &args[1].as_bytes();
    
    if byte.len() == 0 { return; }

    let rem = byte.len() % 3;
    
    let mut encoded = String::from("");

    match rem {
        0 => {
           for b in (0..byte.len()-1).step_by(3) {
              let one = byte[b].shr(2);
              let two = (byte[b] & 0b_00000011).shl(4) + (byte[b+1] & 0b_11110000).shr(4);
              let three = (byte[b+1] & 0b_00001111).shl(2) + (byte[b+2] & 0b_1100000).shr(6);
              let four = byte[b+2] & 0b_00111111;
              encoded.push(BASE64_TABLE.chars().nth(one as usize).unwrap());
              encoded.push(BASE64_TABLE.chars().nth(two as usize).unwrap());
              encoded.push(BASE64_TABLE.chars().nth(three as usize).unwrap());
              encoded.push(BASE64_TABLE.chars().nth(four as usize).unwrap());
           } 
        },
        1 => {
            let zero_padding = 4;
            let eq_padding = 2;
        },
        2 => {
            let zero_padding = 2;
            let eq_padding = 1;
        },
        _ => (),
    }

    println!("{}", encoded);
}
