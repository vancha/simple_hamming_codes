///given n bits of input, how many parity bits do i need?
fn calculate_nr_of_parity_bits(word_length: usize) -> u32 {
    let mut required_parity: u32 = 0;

    while required_parity.pow(2) < required_parity + word_length as u32 + 1 {
        required_parity += 1;
    }
    required_parity
}

///given a bit sequence, return the sequence with included parity bits
fn create_parity_for_data(data: &str) -> String {
    let mut data_iter = data.chars();
    let format = (1..(data.len() + 1) as i32 + calculate_nr_of_parity_bits(data.len()) as i32)
        .map(|c| if (c & (c - 1)) == 0 { 'p' } else { data_iter.next().unwrap() })//(c & (c -1))==0 means: if c is a power of two
        .collect::<String>();

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
