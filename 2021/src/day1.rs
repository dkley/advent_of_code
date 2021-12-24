use std::fs;

pub fn parser() -> Vec<i64> {
    fs::read_to_string("input/day1").unwrap()
        .lines()
        .map(|x| x.parse().unwrap())
        .collect()
}

pub fn part1(input: &Vec<i64>) -> u64 {
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

pub fn part2(input: &Vec<i64>) -> u64 {
    let mut count = 0;
    let mut iter_back = input.iter();
    let iter_front = input.iter().skip(3);

    for x in iter_front {
        if x > iter_back.next().unwrap() {
            count = count + 1
        }
    }

    println!("{}", count);
    count
}
