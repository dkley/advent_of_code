#![allow(dead_code)]
use std::fs;

pub fn _parser(file: &str) -> Vec<u64> {
    fs::read_to_string(file).unwrap()
        .lines().next().unwrap()
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect()
}

pub fn _part1(input: &mut Vec<u64>) -> u64 {
    let mut arr:[u64; 9] = [0; 9];
    let mut tmp:[u64; 9] = [0; 9];
    let mut sum = 0;

    for v in input {
        arr[*v as usize] += 1;
    }

    for _ in 0..80 {
        tmp = [0; 9];
        for ndx in 1..9 {
            tmp[ndx - 1] = arr[ndx];
        }

        tmp[8] = arr[0];
        tmp[6] += arr[0];

        arr = tmp;
        
    }

    for n in arr {
        sum += n;
    }

    sum
}

pub fn _part2(input: &mut Vec<u64>) -> u64 {
    let mut arr:[u64; 9] = [0; 9];
    let mut tmp:[u64; 9] = [0; 9];
    let mut sum = 0;

    for v in input {
        arr[*v as usize] += 1;
    }

    for _ in 0..256 {
        tmp = [0; 9];
        for ndx in 1..9 {
            tmp[ndx - 1] = arr[ndx];
        }

        tmp[8] = arr[0];
        tmp[6] += arr[0];

        arr = tmp;
        
    }

    for n in arr {
        sum += n;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "input/day6";

    #[test]
    fn sol_day6_p1() {
        println!("{}", _part1(&mut _parser(FILE)));
    }

    #[test]
    fn sol_day6_p2() {
        println!("{}", _part2(&mut _parser(FILE)));
    }

    #[test]
    fn example_day6_p1() {
        println!("{}", _part1(&mut _parser("input/day6_example")));
    }
    
    #[test]
    fn example_day6_p2() {
        println!("{}", _part2(&mut _parser("input/day6_example")));
    }
}
