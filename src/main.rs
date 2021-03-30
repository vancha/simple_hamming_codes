#![feature(iter_advance_by)]
///given n bits of input, how many parity bits do i need?
fn calculate_nr_of_parity_bits(word_length: usize) -> u32 {
    //superior way: binlog n+1, and then taking the floor
    let mut required_parity: u32 = 0;

    while required_parity.pow(2) < required_parity + word_length as u32 + 1 {
        required_parity += 1;
    }
    required_parity
    //(word_length as f64).log2().ceil() as u32 //this code is going to replace the code above,
    //eventually.. because it's faster..
}

///given a bit sequence, return the sequence with included parity bits
#[allow(unused_must_use)]
fn create_parity_for_data(data: &str) -> String {
    let mut data_iter = data.chars();
    let nr_of_parity_bits = calculate_nr_of_parity_bits(data.len()) as i32;
    let format = (1..(data.len() + 1) as i32 + nr_of_parity_bits)
        .map(|c| {
            if (c & (c - 1)) == 0 {
                'p'
            } else {
                data_iter.next().unwrap()
            }
        }) //(c & (c -1))==0 means: if c is a power of two
        .collect::<String>();

    let mut parity_bits: Vec<char> = vec![]; //vec to store the parity bits in order

    for (i, p) in format.chars().enumerate() {
        //i is the index of the character in format, p is the actual char at that index
        match p {
            'p' => {
                let mut word_iter = format[i..].chars(); //chop off the first part of the word
                let mut sum = 0; //counter for the number of 1 bits
                'inner: loop {
                    for _check in 0..i + 1 {
                        //do the checks
                        match word_iter.next() {
                            //to the the next character
                            Some(x) => {
                                match x {
                                    //what's the character here
                                    '1' => {
                                        sum += 1;
                                    } //if one, that it's something we want to count
                                    _ => {} //if it's a p or a 0, we discard it.
                                }
                            }
                            None => {
                                if sum % 2 != 0 {
                                    parity_bits.push('1'); //no more characters in the sequence, we are done and the data for this parity bit adds up to an uneven value, add 1 as a parity bit
                                } else {
                                    parity_bits.push('0'); //same, but data adds up to even value, add 0 as parity bit
                                }
                                break 'inner;
                            }
                        }
                    }
                    word_iter.advance_by(i + 1); //skips just as many chars as the _check loop checks, purposefully ignoring Err values.
                }
            }
            _ => {}
        }
    }

    let mut final_word_buffer: Vec<char> = vec![]; //this is going to be the buffer that's used to insert every character of the final word
    let mut index = 0; //index to keep track of how many parity bits we have already added to the final_word_buffer
    for c in format.chars() {
        //loop ofer format's chars (this still contains the p's that represent charity bits)
        if c == 'p' {
            //if a char in format represents a parity bit
            final_word_buffer.push(parity_bits[index]); //replace it with the correct one
            index += 1; //increase the parity bit index
        } else {
            final_word_buffer.push(c); //if not, just push the word in there.
        }
    }
    final_word_buffer.iter().collect::<String>() //return the word with the replace parity bits
}

fn main() {
    let data = "10011010";
    let _data_with_parity = create_parity_for_data(data);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_parity_works() {
        assert_eq!("011100101010", create_parity_for_data("10011010"));
    }
}
