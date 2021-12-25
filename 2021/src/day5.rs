#![allow(dead_code)]
use std::collections::HashMap;
use std::fs;

pub fn _parser(file: &str) -> Vec<Line> {
    fs::read_to_string(file).unwrap().lines().map(|l| l.split(" -> ")
        .map(|s| s.split(',')
            .map(|v| v.parse().unwrap())
        ).flatten().collect()
    ).map(|v: Vec<i64>| Line::new(v[0], v[1], v[2], v[3])).collect()
}

#[derive(Debug, Clone, Copy, Eq, Hash)]
pub struct Point {
    x: i64,
    y: i64,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug)]
pub struct Line {
    p0: Point,
    p1: Point,
}

impl Line {
    pub fn new(x0: i64, y0: i64, x1: i64, y1: i64) -> Line {
        Line{p0: Point{x: x0, y: y0}, p1: Point{x: x1, y: y1}}
    }

    pub fn get_points(&self) -> Vec<Point> {
        let mut ret = Vec::new();
        let xstep = (self.p1.x - self.p0.x).signum();
        let ystep = (self.p1.y - self.p0.y).signum();
        let mut p = Point{x: self.p0.x, y: self.p0.y};

        while p != self.p1 {
            ret.push(p);
            p.x += xstep;
            p.y += ystep;
        }

        ret.push(self.p1);

        ret
    }
}

pub fn _part1(input: &mut Vec<Line>) -> i64 {
    let mut map = HashMap::new();
    input.retain(|l| l.p0.x == l.p1.x || l.p0.y == l.p1.y);

    for l in input {
        for p in l.get_points() {
            *map.entry(p).or_insert(0) += 1;
        }
    }

    map.retain(|_, v| *v > 1);
    map.len().try_into().unwrap()
}

pub fn _part2(input: &mut Vec<Line>) -> i64 {
    let mut map = HashMap::new();

    for l in input {
        for p in l.get_points() {
            *map.entry(p).or_insert(0) += 1;
        }
    }

    map.retain(|_, v| *v > 1);
    map.len().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "input/day5";

    #[test]
    fn sol_day5_p1() {
        println!("{}", _part1(&mut _parser(FILE)));
    }

    #[test]
    fn sol_day5_p2() {
        println!("{}", _part2(&mut _parser(FILE)));
    }

    #[test]
    fn example_day5_p1() {
        println!("{}", _part1(&mut _parser("input/day5_example")));
    }
}
