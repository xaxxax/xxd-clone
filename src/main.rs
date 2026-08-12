use std::error::Error;
use std::fs;

// parse file in bytes (len 8, split on ' ')

// read each bit in byte from array of bytes,
// split into 2 groups of 4
// compute sum of group, add together to get 2len hex code

// create list of HEX codes, let index be the sum of N code
// then, get code for N sum

const HEX: [&str; 16] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F",
];

fn main() -> Result<(), Box<dyn Error>> {
    let mut h_sum = 0;
    let mut l_sum = 0;
    let mut hex_message: Vec<String> = Vec::new();

    let byte_content = fs::read_to_string("bytes.txt")?;

    let byte_array: Vec<&str> = byte_content.split_whitespace().collect();
    for byte in byte_array.iter() {
        let (high, low) = byte.split_at(4);
        for bit in 0..4 {
            let index = 3 - bit;
            let h = high.chars().nth(bit).unwrap();
            let l = low.chars().nth(bit).unwrap();
            if h == '1' {
                let base: usize = 2;
                h_sum = h_sum + base.pow(index as u32);
            }
            if l == '1' {
                let base: usize = 2;
                l_sum = l_sum + base.pow(index as u32);
            }
        }
        let mut str = String::new();
        str.push_str(HEX[h_sum]);
        str.push_str(HEX[l_sum]);
        hex_message.push(str);
        h_sum = 0;
        l_sum = 0;
    }

    let mut msg = String::new();

    for code in hex_message.iter() {
        let num = u8::from_str_radix(code, 16).unwrap();
        let c = num as char;

        msg.push(c);
    }

    println!("{:?}", hex_message);
    println!("{:?}", msg);
    Ok(())
}
