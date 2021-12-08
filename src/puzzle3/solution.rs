use bitset;

pub fn most_common_bits(lines: &Vec<String>) -> String {

    let str_size = lines[0].len();
    let mut one_counts = vec![0; str_size];

    for line in lines {
        let chars: Vec<_> =line.chars().collect();
        for i in 0..str_size {
            if chars[i] == '1' {
                one_counts[i] += 1;
            }
        }
    }
    
    let mut result_chars = vec!['0'; str_size];
    for i in 0..one_counts.len() {
        let count = one_counts[i];
        if count >= (lines.len() / 2) + 1 {
            result_chars[i] = '1';
        }
        else {
            result_chars[i] = '0';
        }
    }

    return result_chars.into_iter().collect();
}

#[cfg(test)]
mod puzzle3_tests {
    use super::*;

    #[test]
    fn test_count_most_common_bits() {
        let sample_args: Vec<String> = vec![
            String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010")
        ];

        let gamma_string = most_common_bits(&sample_args);
        assert_eq!(gamma_string, "10110");
        let value = isize::from_str_radix(&gamma_string, 2).unwrap();
        assert_eq!(value, 22);
        let mut bit_set = bitset::BitSet::from_u64(value as u64);
        bit_set.flip_all();
        let mut bit_chars = vec!['0'; gamma_string.len()];
        for i in 0..sample_args[0].len() {
            if bit_set.test(i) {
                bit_chars[i] = '1';
            }
            else {
                bit_chars[i] = '0';
            }
        }

        bit_chars.reverse();
        let epsilon_string : String = bit_chars.into_iter().collect();
        let epsilon = isize::from_str_radix(&epsilon_string, 2).unwrap();
        assert_eq!(epsilon, 9);
    }
}
