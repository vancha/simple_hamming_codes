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

    //let mut word_iter = format[i+1..].chars();

    for (i, p) in format.chars().enumerate() {
        match p {
            'p' => {
                let mut word_iter = format[i..].chars();
                println!(
                    "starting at position {} with word {}",
                    i + 1,
                    word_iter.clone().collect::<String>()
                );
                'outer: loop {
                    'inner: loop {
                        for check in 0..i + 1 {
                            match word_iter.next() {
                                Some(x) => {
                                    println!("checking {},{}", x, i + 1);
                                }
                                None => {
                                    println!("done for position {}", i + 1);
                                    break 'inner;
                                }
                            }
                        }
                        for skip in 0..i + 1 {
                            match word_iter.next() {
                                Some(x) => {
                                    //println!("skipping {}", x);
                                }
                                None => {
                                    println!("done for position {}", i + 1);
                                    break 'inner;
                                }
                            }
                        }
                    }
                    break;
                }
            }
            _ => {
                print!("");
            }
        }
    }

    format
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
