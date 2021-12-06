use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    a();
    b();
}

fn a() {
    println!("a)");
    let path = Path::new("input.txt");
    let file = File::open(&path).unwrap();

    let mut bufreader = io::BufReader::new(file);

    let input = parse_inputs(&mut bufreader);
    let mut state: Vec<BoardChecks> = vec![];

    for _board in &input.boards {
        state.push(BoardChecks{checks: [[false; 5]; 5]})
    }

    'outer: for num in input.number_sequence {
        for (check, board) in state.iter_mut().zip(&input.boards) {
            if update_check(check, board, num) {
                let sum = sum_marked(&board, &check);
                println!("{:?} - {} * {} = {}", &check, sum, num, sum as u32 *num as u32);
                break 'outer;
            }
        }
    }

}

fn b() {
    println!("b)");
    let path = Path::new("input.txt");
    let file = File::open(&path).unwrap();

    let mut bufreader = io::BufReader::new(file);

    let input = parse_inputs(&mut bufreader);
    let mut state: Vec<BoardChecks> = vec![BoardChecks{checks: [[false; 5]; 5]}; input.boards.len()];
    let mut boards_done: Vec<bool> = vec![false; input.boards.len()];

    /*for _board in &input.boards {
        state.push(BoardChecks{checks: [[false; 5]; 5]})
    }*/

    'outer: for num in input.number_sequence {
        for ((check, board), done) in state.iter_mut()
                                .zip(&input.boards)
                                .zip(boards_done.iter_mut())
                                .filter(|(_, done)| !**done) {
            if update_check(check, board, num) {
                let sum = sum_marked(&board, &check);
                println!("{:?} - {} * {} = {}", &check, sum, num, sum as u32 *num as u32);
                *done = true;
            }
        }
    }   
}

struct Input {
    number_sequence: Vec<u16>,
    boards: Vec<Board>,
}

#[derive(Debug)]
struct Board {
    field: [[u16; 5]; 5]
}

#[derive(Debug, Clone)]
struct BoardChecks {
    checks: [[bool; 5]; 5]
}

fn parse_board(reader: &mut dyn BufRead) -> Option<Board> {
    let mut board = Board {
        field: [[0u16; 5]; 5],
    };
    let mut buf = String::new();
    let mut i = 0; // current line number to parse

    while reader.read_line(&mut buf).unwrap() > 0 && i < 5 {
        if buf.trim().is_empty() {
            continue;
        }

        for (j, s) in buf.trim().split_whitespace().enumerate() {
            match s.parse() {
                Ok(num) => board.field[i][j] = num,
                Err(_) => continue, // not optimal..
            }
        }
        i += 1;
        buf.clear();
    }

    if i != 5 {
        None
    } else {
        Some(board)
    }
}

fn parse_boards(reader: &mut dyn BufRead) -> Vec<Board> {
    let mut boards = vec![];

    while let Some(board) = parse_board(reader) {
        boards.push(board);
    }

    boards
}

fn parse_number_seq(reader: &mut dyn BufRead) -> Vec<u16> {
    let mut numbers = vec![];
    let mut line = String::new();

    if reader.read_line(&mut line).unwrap() > 0 {

        let mut iter = line.split(',');
        while let Some(numstr) = iter.next() {
            if let Ok(num) = numstr.parse::<u16>() {
                numbers.push(num);
            }
        }
    }

    numbers
}

fn parse_inputs(reader: &mut dyn BufRead) -> Input {
    let input = Input {
        number_sequence: parse_number_seq(reader),
        boards: parse_boards(reader),
    };

    input
}

fn bingo_check(boardcheck: &BoardChecks, i: usize, j: usize)  -> bool{

    (0..5).all(|jj| boardcheck.checks[i][jj]) ||
        (0..5).all(|ii| boardcheck.checks[ii][j])
}

fn update_check(boardcheck: &mut BoardChecks, board: &Board, num: u16) -> bool {
    for i in 0..5 {
        for j in 0..5 {
            if board.field[i][j] == num {
                boardcheck.checks[i][j] = true;
                return bingo_check(boardcheck, i, j);
            }
        }
    }
    return false;    
}

fn sum_marked(board: &Board, marks: &BoardChecks) -> u16 {
    let mut sum: u16 = 0;
    for i in 0..5 {
        for j in 0..5 {
            if !marks.checks[i][j] {
                sum += board.field[i][j];
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUTSTRING: &str = 
"50,68,2,1,69,32,87,10,31,21,78,23,62,98,16,99,65,35,27,96,66,26,74,72,45,52,81,60,38,57,54,19,18,77,71,29,51,41,22,6,58,5,42,92,85,64,94,12,83,11,17,14,37,36,59,33,0,93,34,70,97,7,76,20,3,88,43,47,8,79,80,63,9,25,56,75,15,4,82,67,39,30,89,86,46,90,48,73,91,55,95,28,49,61,44,84,40,53,13,24

38 80 23 60 82
25 35 28 47 39
40  0 30 48 76
32 41 49 69  4
13 42 89 20 12

76 89 13  5 98
87 48  2 59 20
37 88 41 24 57
16 85 31 73 95
70 11 93 30 27
";

    #[test]
    fn parse_board_works() {

        let board_input = &INPUTSTRING[365..];
        let mut cursor = io::Cursor::new(board_input);
        
        let board = parse_board(&mut cursor).unwrap();
        
        // Some random checks
        assert_eq!(board.field[0][0], 76);
        assert_eq!(board.field[1][2], 2);
        assert_eq!(board.field[3][1], 85);
    }

    #[test]
    fn parse_boards_works() {
        let input = &INPUTSTRING[290..];
        let mut cursor = io::Cursor::new(input);

        let boards = parse_boards(&mut cursor);

        assert_eq!(boards.len(), 2);

    }

    #[test]
    fn parse_number_seq_works() {
        let input = &INPUTSTRING[..291];
        let mut cursor = io::Cursor::new(input);

        let number_sequence = parse_number_seq(&mut cursor);

        assert_eq!(number_sequence[0], 50);
        assert_eq!(number_sequence[4], 69);
    }

    #[test]
    fn parse_input_works() {
        let mut cursor = io::Cursor::new(INPUTSTRING);

        let input = parse_inputs(&mut cursor);
    }

    #[test]
    fn bingo_check_works() {
        let mut checks = BoardChecks{
            checks: [[false; 5]; 5],
        };

        assert_eq!(bingo_check(&checks, 0, 0), false);

        // Detect column
        for i in 0..5 {
            checks.checks[i][0] = true;
        }
        
        assert_eq!(bingo_check(&checks, 0, 0), true);
        assert_eq!(bingo_check(&checks, 1, 0), true);
        assert_eq!(bingo_check(&checks, 0, 1), false);

        // Detect row
        for j in 0..5 {
            checks.checks[2][j] = true;
        }

        assert_eq!(bingo_check(&checks, 2, 0), true);
        assert_eq!(bingo_check(&checks, 2, 3), true);
        assert_eq!(bingo_check(&checks, 1, 1), false);
    }

    #[test]
    fn update_check_works() {
        let board = Board {
            field: [[1, 2, 3, 4, 5], [6, 7, 8, 9, 10], [11, 12, 13, 14, 15],
                [16, 17, 18, 19, 20], [21, 22, 23, 24, 25]],
        };

        let mut checks = BoardChecks{
            checks: [[false; 5]; 5],
        };

        assert_eq!(update_check(&mut checks, &board, 0), false);

        println!("{:?}", &checks);
        assert_eq!(update_check(&mut checks, &board,  1), false);
        assert_eq!(update_check(&mut checks, &board,  6), false);
        assert_eq!(update_check(&mut checks, &board, 11), false);
        assert_eq!(update_check(&mut checks, &board, 16), false);
        assert_eq!(update_check(&mut checks, &board, 21), true);
    }
}
