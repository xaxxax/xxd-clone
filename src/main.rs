use std::error::Error;
use std::fs;

// parse file in bytes (len 8, split on ' ')

// read each bit in byte from array of bytes,
// split into 2 groups of 4
// compute sum of group, add together to get 2len hex code

// create list of HEX codes, let index be the sum of N code
// then, get code for N sum

fn main() -> Result<(), Box<dyn Error>> {
    let byte_content = fs::read_to_string("bytes.txt")?;
    let byte_array1: Vec<&str> = byte_content.split_whitespace().collect();

    let msg = build_message(byte_array1);
    println!("{}", msg);

    let byte_array2: Vec<&str> = byte_content.split_whitespace().collect();

    let horner = build_message_with_horner(byte_array2);
    println!("{}", horner);

    Ok(())
}

fn calculate_binary_sum(group: &str) -> u8 {
    let mut sum: usize = 0;

    for i in 0..group.len() {
        let exp = ((group.len() - 1) - i) as u32;
        if group.chars().nth(i).unwrap() == '1' {
            sum += (2 as usize).pow(exp);
        }
    }

    return sum as u8;
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

fn build_message(data: Vec<&str>) -> String {
    let mut msg = String::new();

    for group in data.iter() {
        let mut sum = calculate_binary_sum(group);

        if sum > 126 {
            msg.push('.');
        }

        msg.push(sum as char);
        sum = 0;
    }

    return msg;
}

fn build_message_with_horner(data: Vec<&str>) -> String {
    let mut msg = String::new();

    for group in data.iter() {
        let mut sum = horner_method(group);

        if sum > 126 {
            msg.push('.');
        }

        msg.push(sum as char);
        sum = 0;
    }

    return msg;
}
