use std::error::Error;
use std::fs;

// parse file in bytes (len 8, split on ' ')

// read each bit in byte from array of bytes,
// split into 2 groups of 4
// compute sum of group, add together to get 2len hex code

// create list of HEX codes, let index be the sum of N code
// then, get code for N sum

const HEX: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];

fn main() -> Result<(), Box<dyn Error>> {
    let byte_content = fs::read_to_string("bytes.txt")?;
    let byte_array1: Vec<&str> = byte_content.split_whitespace().collect();

    let msg = build_message(byte_array1);
    println!("{}", msg);

    Ok(())
}

fn horner_method(group: &str) -> u8 {
    let mut sum: usize = 0;
    for c in group.chars() {
        if c == '1' {
            sum = (sum * 2) + 1;
        } else {
            sum = (sum * 2) + 0;
        }
    }

    return sum as u8;
}

fn to_hex(sum: usize) -> String {
    // if sum <= 15 (0X), take sum look up return 0X
    // if sum > 16, sum / 16, take remainder and mod

    if sum <= 15 {
        let mut msg = String::from("0");

        msg.push(HEX[sum]);
        msg.push(' ');

        return msg;
    }

    let mut msg = String::new();

    msg.push(HEX[sum / 16]);
    msg.push(HEX[sum % 16]);
    msg.push(' ');

    return msg;
}

fn build_message(data: Vec<&str>) -> String {
    let mut msg = String::new();
    let mut hex = String::new();

    for group in data.iter() {
        let mut sum = horner_method(group);

        if (sum < 32) | (sum > 126) {
            msg.push('.');
        } else {
            msg.push(sum as char);
        }

        hex.push_str(&to_hex(sum as usize));

        sum = 0;
    }

    println!("{}", hex);

    return msg;
}
