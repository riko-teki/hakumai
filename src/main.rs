use std::{
    env::args,
    io::{Write, *},
    ops::{Shl, Shr},
    string::FromUtf8Error,
};

const BASE64_TABLE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const BASE64URL_TABLE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_"; // URLを扱う際に安全な文字列でエンコードするためのテーブル
const BASE32_TABLE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
const BASE32HEX_TABLE: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUV";
const BASE16_TABLE: &str = "0123456789ABCDEF";

const USAGE: &str = "USAGE:\n hakumai <mode> <method> <string>";

enum Mode {
    Encode,
    Decode,
}

enum Method {
    Base64,
    Base64Url,
    Base32,
    Base32Hex,
    Base16,
}

fn main() {
    if args().len() < 4 {
        println!("{}", USAGE);
        return;
    }

    let mode = match args().nth(1).unwrap().as_str() {
        "-e" => Mode::Encode,
        "-d" => Mode::Decode,
        _ => unreachable!(),
    };

    let method = match args().nth(2).unwrap().as_str() {
        "64" => {}
        "64u" => {}
        "32" => {}
        "32h" => {}
        "16" => {}
        _ => {}
    };

    let input = args().nth(3).unwrap();

    match mode {
        Mode::Encode => println!("{}", encode(input)),
        Mode::Decode => match decode_as_utf8(input.clone()) {
            Ok(v) => println!("{}", &v),
            Err(_) => {
                let mut writer = BufWriter::new(stdout());
                writer.write(&decode_as_binary(input)).unwrap();
            }
        },
    }

    fn encode(raw: String) -> String {
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
        encoded
    }

    fn decode_as_binary(encoded: String) -> Vec<u8> {
        // "="の削除
        let filterd: String = encoded.chars().filter(|c| *c != '=').collect();

        // 文字数を4で割った商と余りを計算
        let quot = filterd.len() / 4;
        let rem = filterd.len() % 4;

        // 変換テーブルを使って逆変換
        let mut transrated: Vec<u8> = vec![];

        for f in filterd.chars() {
            transrated.push(BASE64_TABLE.find(|c: char| c == f).unwrap() as u8);
        }

        let mut decoded: Vec<u8> = vec![];

        for i in 0..quot {
            let idx = i * 4;
            let one = transrated[idx].shl(2) + (transrated[idx + 1] & 0b_11110000).shr(4);
            let two = (transrated[idx + 1] & 0b_00001111).shl(4)
                + (transrated[idx + 2] & 0b_11111100).shr(2);
            let three = transrated[idx + 2].shl(6) + transrated[idx + 3];

            decoded.push(one);
            decoded.push(two);
            decoded.push(three);
        }

        match rem {
            3 => {
                let padding = 2;
                let last = transrated[transrated.len() - 1];
                let second_to_last = transrated[transrated.len() - 2];
                let third_to_last = transrated[transrated.len() - 3];

                let one = third_to_last.shl(2) + (second_to_last & 0b_11110000);
                let two = second_to_last.shl(4) + last.shr(padding);

                decoded.push(one);
                decoded.push(two);
            }
            2 => {
                let padding = 4;

                let last = transrated[transrated.len() - 1];
                let second_to_last = transrated[transrated.len() - 2];

                let one = second_to_last.shl(2) + last.shr(padding);
                decoded.push(one);
            }
            _ => {}
        }

        decoded
    }

    fn decode_as_utf8(input: String) -> std::result::Result<String, FromUtf8Error> {
        String::from_utf8(decode_as_binary(input))
    }
}
