use crate::days::Day;
use crate::input::get_data_from_file;

pub struct Day4 {}

impl Day for Day4 {
    fn part1(&self, input_root: &str) {
        println!("Value of winning board {}", play_bingo(input_root, false));
    }

    fn part2(&self, input_root: &str) {
        println!("Value of winning board {}", play_bingo(input_root, true));
    }
}

/// Calculates the product of the sum of all unmarked spots on a bingo board and the winning
/// number. If get_last is true, the value of the LAST winning board is returned. Otherwise the
/// value of the first is returned.
fn play_bingo(input_root: &str, get_last: bool) -> i32 {
    let input = get_data_from_file(input_root, "day4.txt", |r| r);
    let called_numbers: Vec<i32> = input
        .get(0)
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    let mut boards: Vec<Board> = Vec::new();
    for start in (2..input.len()).step_by(6) {
        let mut board = new_board();
        for i in start..start + 5 {
            let num_row: Vec<i32> = input
                .get(i)
                .unwrap()
                .split(" ")
                .map(|n| n.trim())
                .filter(|n| n.len() > 0)
                .map(|n| n.parse().unwrap())
                .collect();
            for j in 0..5 {
                board.numbers[(i - 2) % 5][j] = *num_row.get(j).unwrap();
            }
        }
        boards.push(board);
    }

    let mut last_win_score = -1;
    for num in called_numbers {
        for board in &mut boards {
            let won = mark(num, board);
            if won && !get_last {
                return num * get_val(&board);
            } else if won {
                last_win_score = num * get_val(&board);
            }
        }
    }
    return last_win_score;
}

struct Board {
    numbers: [[i32; 5]; 5],
    has_won: bool,
}

fn new_board() -> Board {
    Board {
        numbers: [[0; 5]; 5],
        has_won: false,
    }
}

fn mark(n: i32, board: &mut Board) -> bool {
    if board.has_won {
        return false;
    }

    for i in 0..5 {
        for j in 0..5 {
            if board.numbers[i][j] == n {
                board.numbers[i][j] = -1;
            }
        }
    }

    for i in 0..5 {
        let mut count = 0;
        for j in 0..5 {
            if board.numbers[i][j] == -1 {
                count += 1;
            }
        }
        if count == 5 {
            board.has_won = true;
            return true;
        }
    }

    //check cols
    for j in 0..5 {
        let mut count = 0;
        for i in 0..5 {
            if board.numbers[i][j] == -1 {
                count += 1;
            }
        }
        if count == 5 {
            board.has_won = true;
            return true;
        }
    }
    return false;
}

fn get_val(board: &Board) -> i32 {
    let mut sum = 0;
    for i in 0..5 {
        for j in 0..5 {
            if board.numbers[i][j] >= 0 {
                sum += board.numbers[i][j];
            }
        }
    }
    return sum;
}
