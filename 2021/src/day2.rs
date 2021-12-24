use std::fs;

#[derive(Debug)]
pub enum _Direction {
    Forward,
    Up,
    Down
}

pub struct _Command {
    direction: _Direction,
    amount: i64,

}

pub fn _parser(file: &str) -> Vec<_Command> {
    let mut commands = Vec::new();
    for line in fs::read_to_string(file).unwrap().lines() {
        let mut iter = line.split(' ');
        let dir = match iter.next().unwrap() {
            "forward" => _Direction::Forward,
            "up" => _Direction::Up,
            "down" => _Direction::Down,
            _ => panic!("Unexpected direction")
        };
        let amount: i64 = iter.next().unwrap().parse().unwrap();
        commands.push(_Command{direction:dir, amount:amount});
    }

    commands
}

pub fn _part1(input: &Vec<_Command>) -> i64 {
    let mut pos = (0, 0); // 1 - horizontal, 2 - depth

    for command in input {
        pos = match command.direction {
            _Direction::Forward => (pos.0 + command.amount, pos.1),
            _Direction::Up => (pos.0, pos.1 - command.amount),
            _Direction::Down => (pos.0, pos.1 + command.amount),
        };
    }

    pos.0 * pos.1
}

pub fn _part2(input: &Vec<_Command>) -> i64 {
    let mut pos = (0, 0, 0); // 3 - aim

    for command in input {
        pos = match command.direction {
            _Direction::Forward => (pos.0 + command.amount, pos.1 + pos.2 * command.amount, pos.2),
            _Direction::Up => (pos.0, pos.1, pos.2 - command.amount),
            _Direction::Down => (pos.0, pos.1, pos.2 + command.amount),
        };
    }

    pos.0 * pos.1
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "input/day2";

    #[test]
    fn sol_day2_p1() {
        println!("{}", _part1(&_parser(FILE)));
    }

    #[test]
    fn sol_day2_p2() {
        println!("{}", _part2(&_parser(FILE)));
    }
}
