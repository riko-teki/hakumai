use std::{
    env::args,
    ops::{Shl, Shr},
};

const BASE64_TABLE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 { return; } 
    println!("{:?}", &args[1].len());
    println!("{:?}", &args[1].as_bytes());
    println!("{}", encode(&args[1]));
}

fn encode(raw: &str) -> String {
    let byte = raw.as_bytes();

    if byte.len() == 0 {
        return "".to_string();
    }

    let quot = byte.len() / 3;
    let rem = byte.len() % 3;

    let mut encoded = String::from("");

    for i in 0..quot {
        let idx = i * 3;
        let one = byte[idx].shr(2);
        let two = (byte[idx] & 0b_00000011).shl(4) + (byte[idx + 1] & 0b_11110000).shr(4);
        let three = (byte[idx + 1] & 0b_00001111).shl(2) + (byte[idx + 2] & 0b_11000000).shr(6);
        let four = byte[idx + 2] & 0b_00111111;
        encoded.push(BASE64_TABLE.chars().nth(one as usize).unwrap());
        encoded.push(BASE64_TABLE.chars().nth(two as usize).unwrap());
        encoded.push(BASE64_TABLE.chars().nth(three as usize).unwrap());
        encoded.push(BASE64_TABLE.chars().nth(four as usize).unwrap());
    }

    match rem {
        1 => {
            let zero_padding = 4;
            let eq_padding = 2;

            let last = byte.last().unwrap();
            let one = last.shr(2);
            let two = (last & 0b_00000011).shl(zero_padding);

            encoded.push(BASE64_TABLE.chars().nth(one as usize).unwrap());
            encoded.push(BASE64_TABLE.chars().nth(two as usize).unwrap());

            for _ in 0..eq_padding {
               encoded.push('='); 
            }
        }
        2 => {
            let zero_padding = 2;
            let eq_padding = 1;

            let last = byte[byte.len() - 1];
            let second_to_last = byte[byte.len() - 2];

            let one = second_to_last.shr(2);
            let two = (second_to_last & 0b_00000011).shl(4) + (last & 0b_11110000).shr(4);
            let three = (last & 0b_00001111).shl(zero_padding);

            encoded.push(BASE64_TABLE.chars().nth(one as usize).unwrap());
            encoded.push(BASE64_TABLE.chars().nth(two as usize).unwrap());
            encoded.push(BASE64_TABLE.chars().nth(three as usize).unwrap());

            for _ in 0..eq_padding {
               encoded.push('='); 
            }
        }
        _ => (),
    }
    return encoded;
}
