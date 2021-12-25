#![allow(dead_code)]

use std::fs;
use std::fmt;

pub fn _parser(file: &str) -> BingoGame {
    let file = fs::read_to_string(file).unwrap();
    let mut lines = file.lines();
    let mut game = 
        BingoGame{numbers: lines.next().unwrap().split(',').map(|i| i.parse().unwrap()).collect(),
        boards: Vec::new()};

    while let Some(_line) = lines.next() {
        let mut board = BingoBoard {board: [[BingoSpot{value: 0, marked: false}; 5]; 5], winner: false};
        for row in 0..5 {
            let tmp: Vec<u64> = lines.next().unwrap().replace("  ", " ").trim().split(' ').map(|s| s.parse().unwrap()).collect();
            for col in 0..5 {
                board.board[row][col] = BingoSpot{value:tmp[col], marked: false};
            }
        }

        game.boards.push(board);
    }

    game
}

#[derive(Copy, Clone, Debug)]
pub struct BingoSpot {
    value: u64,
    marked: bool,
}

pub struct BingoBoard {
    board: [[BingoSpot; 5]; 5],
    winner: bool,
}

impl fmt::Debug for BingoBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:?}", self.winner).unwrap();
        for row in 0..5 {
            for col in 0..5 {
                if self.board[row][col].marked {
                    write!(f, "X").unwrap();
                }
                else {
                    write!(f, "O").unwrap();
                }
            }

            writeln!(f, "").unwrap();
        }

        writeln!(f, "---").unwrap();
        for row in 0..5 {
            for col in 0..5 {
                if self.board[col][row].marked {
                    write!(f, "X").unwrap();
                }
                else {
                    write!(f, "O").unwrap();
                }
            }
            
            writeln!(f, "").unwrap();
        }

        write!(f, "***")
    }
}

impl BingoBoard {
    fn mark_number(&mut self, number: u64) {
        for row in 0..5 {
            for col in 0..5 {
                if self.board[row][col].value == number {
                    self.board[row][col].marked = true;
                    if self.check_bingo(row, col) {
                        self.winner = true;
                    }
                    break;
                }
            }
        }
    }

    fn check_bingo(&self, row: usize, col: usize) -> bool {
        let mut bingo: bool = true;

        for row_ndx in 0..5 {
            if self.board[row_ndx][col].marked == false {
                bingo = false;
                break;
            }
        }

        if bingo {
            return bingo;
        }

        bingo = true;
        for col_ndx in 0..5 {
            if self.board[row][col_ndx].marked == false {
                bingo = false;
                break;
            }
        }

        bingo
    }

    fn sum_unmarked(&self) -> u64 {
        let mut ret: u64 = 0;
        for row in 0..5 {
            for col in 0..5 {
                if !self.board[row][col].marked {
                    ret = ret + self.board[row][col].value;
                }
            }
        }

        ret
    }
}

#[derive(Debug)]
pub struct BingoGame {
    numbers: Vec<u64>,
    boards: Vec<BingoBoard>,
}

pub fn _part1(input: &mut BingoGame) -> u64 {
    let mut winner_val: u64 = 0;
    for num in &input.numbers {
        for board in input.boards.iter_mut() {
            board.mark_number(*num);
            if board.winner == true {
                winner_val = board.sum_unmarked() * num;
                break;
            }
        }
        
        if winner_val > 0 {
            break;
        }
    }

    winner_val
}

pub fn _part2(input: &mut BingoGame) -> u64 {
    let mut loser_val: u64 = 0;
    for num in &input.numbers {
        for board in input.boards.iter_mut() {
            board.mark_number(*num);
        }

        if input.boards.len() > 1 {
            input.boards.retain(|x| !x.winner);
        }
        
        if input.boards.len() == 1 && input.boards[0].winner {
            loser_val = input.boards[0].sum_unmarked() * num;
            break;
        }
    }

    loser_val
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "input/day4";

    #[test]
    fn sol_day4_p1() {
        println!("{}", _part1(&mut _parser(FILE)));
    }

    #[test]
    fn sol_day4_p2() {
        println!("{}", _part2(&mut _parser(FILE)));
    }

    #[test]
    fn example_day4_p2() {
        println!("{}", _part2(&mut _parser("input/day4_example")));
    }
}
