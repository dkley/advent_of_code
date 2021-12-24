use std::fs;

pub fn _parser(file: &str) -> Vec<Vec<char>> {
    fs::read_to_string(file).unwrap()
        .lines().map(|l| l.chars().collect()).collect()
}

pub fn _part1(input: &Vec<Vec<char>>) -> i64 {
    let mut arr:[i64; 12] = [0; 12];
    let mut ndx = 0;

    for line in input {
        for c in line {
            arr[ndx] = match c {
                '0' => arr[ndx] - 1,
                '1' => arr[ndx] + 1,
                _ => panic!("No match"),
            };

            ndx = ndx + 1;
        }

        ndx = 0;
    }

    let gamma =
        i64::from_str_radix(
            arr.iter().map(
                |x| if *x > 0 {'1'} else {'0'}
            ).collect::<String>().as_str(),
            2
        ).unwrap();

    gamma * (!gamma & 0x0FFF) // Epsilon is gamma bit flipped (Values use 12 bits)
}

pub fn _part2(input: &Vec<Vec<char>>) -> i64 {
    /* If data set were larger we would use array ndxs to track what to keep,
     * instead we just copy the data to a new array
     */
    let mut common_values = input.clone();

    for ndx in 0..12 {
        let common_position = _get_most_common(&common_values, ndx);
        let mut tmp = Vec::new();
        for line in common_values {
            if line[ndx] == common_position {
                tmp.push(line);
            }
        }

        common_values = tmp;
        if common_values.len() <= 1 {
            break;
        }
    }

    let oxygen =
        i64::from_str_radix(
            common_values[0].iter().collect::<String>().as_str(),
            2
        ).unwrap();

    common_values = input.clone();
    for ndx in 0..12 {
        let common_position = _get_most_common(&common_values, ndx);
        let mut tmp = Vec::new();
        for line in common_values {
            if line[ndx] != common_position {
                tmp.push(line);
            }
        }

        common_values = tmp;
        if common_values.len() <= 1 {
            break;
        }
    }

    let co2 =
        i64::from_str_radix(
            common_values[0].iter().collect::<String>().as_str(),
            2
        ).unwrap();

    oxygen * co2
}

/*
 * mask - Position of bit to mask off
 *     b0111
 *      ^ - mask = 0
 */
fn _get_most_common(readings: &Vec<Vec<char>>, mask: usize) -> char {

    let mut num_ones = 0;

    for line in readings {
        if line[mask] == '1' {
            num_ones = num_ones + 1;
        }
    }

    if num_ones >= readings.len() - num_ones {
        return '1'
    }

    '0'
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "input/day3";

    #[test]
    fn sol_day3_p1() {
        println!("{}", _part1(&_parser(FILE)));
    }

    #[test]
    fn sol_day3_p2() {
        println!("{}", _part2(&_parser(FILE)));
    }

    #[test]
    fn commonality() {
        let test: Vec<Vec<char>> = Vec::from([
            Vec::from(['1','1','1','0','0','0','0','0','0','0','0','0']),
            Vec::from(['1','1','0','0','0','0','0','0','0','0','0','0']),
            Vec::from(['1','0','0','0','0','0','0','0','0','0','0','0']),
            Vec::from(['0','0','0','0','0','0','0','0','0','0','0','0']),
        ]);

        _part2(&test);
    }
}
