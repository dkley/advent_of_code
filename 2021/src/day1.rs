use std::fs;

pub fn _parser(file: &str) -> Vec<i64> {
    fs::read_to_string(file).unwrap()
        .lines()
        .map(|x| x.parse().unwrap())
        .collect()
}

pub fn _part1(input: &Vec<i64>) -> u64 {
    let mut count = 0;
    let mut iter = input.iter();
    let mut prev = iter.next().unwrap();

    for x in iter {
        if x > prev {
            count = count + 1;
        }

        prev = x;
    }

    count
}

pub fn _part2(input: &Vec<i64>) -> u64 {
    let mut count = 0;
    let mut iter_back = input.iter();
    let iter_front = input.iter().skip(3);

    for x in iter_front {
        if x > iter_back.next().unwrap() {
            count = count + 1
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "input/day1";

    #[test]
    fn sol_day1_p1() {
        println!("{}", _part1(&_parser(FILE)));
    }

    #[test]
    fn sol_day1_p2() {
        println!("{}", _part2(&_parser(FILE)));
    }
}
