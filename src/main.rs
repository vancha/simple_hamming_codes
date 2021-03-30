#![feature(iter_advance_by)]
///given n bits of input, how many parity bits do i need?
fn calculate_nr_of_parity_bits(word_length: usize) -> u32 {
    //superior way: binlog n+1, and then taking the floor
    let mut required_parity: u32 = 0;

    while required_parity.pow(2) < required_parity + word_length as u32 + 1 {
        required_parity += 1;
    }
    required_parity
    //(word_length as f64).log2().ceil() as u32
}

///given a bit sequence, return the sequence with included parity bits
fn create_parity_for_data(data: &str) -> String {
    let mut data_iter = data.chars();
    let mut nr_of_parity_bits = calculate_nr_of_parity_bits(data.len()) as i32;
    let format = (1..(data.len() + 1) as i32 + nr_of_parity_bits)
        .map(|c| {
            if (c & (c - 1)) == 0 {
                'p'
            } else {
                data_iter.next().unwrap()
            }
        }) //(c & (c -1))==0 means: if c is a power of two
        .collect::<String>();

    let mut parity_bits: Vec<char> = vec![];

    for (i, p) in format.chars().enumerate() {
        //i is the index of the character in format, p is the actual char at that index
        match p {
            //match the char
            'p' => {
                //if it's p, that means it stands for a parity bit.
                let mut word_iter = format[i..].chars(); //chop off the first part of the word
                let mut sum = 0; //count the number of 1 bits
                'inner: loop {
                    for _check in 0..i + 1 {
                        //do the checks
                        match word_iter.next() {
                            //to the the next character
                            Some(x) => {
                                //check if it's not out of range
                                println!("checking {},{}", x, i + 1);
                                match x {
                                    //what's the character here
                                    '1' => {
                                        sum += 1;
                                    } //if one, that it's something we want to count
                                    _ => {} //if it's a p or a 0, we discard it.
                                }
                            }
                            None => {
                                println!("done for position {},result is {}", i + 1, sum);
                                if sum % 2 != 0 {
                                    parity_bits.push('1');
                                } else {
                                    parity_bits.push('0');
                                }
                                break 'inner;
                            }
                        }
                    }

                    word_iter.advance_by(i + 1);
                }
            }
            _ => {}
        }
    }

    let mut final_word_buffer: Vec<char> = vec![];
    let mut index = 0;
    for c in format.chars() {
        if c == 'p' {
            final_word_buffer.push(parity_bits[index]);
            index += 1;
        } else {
            final_word_buffer.push(c);
        }
    }
    final_word_buffer.iter().collect::<String>()
}

fn main() {
    let data = "10011010";
    let data_with_parity = create_parity_for_data(data);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_parity_works() {
        assert_eq!("011100101010", create_parity_for_data("10011010"));
    }
}
