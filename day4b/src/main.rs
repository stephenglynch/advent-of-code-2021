
#[derive(Debug, Copy, Clone)]
struct BingoNum {
    num: u32, 
    marked: bool
}

type Board = [[BingoNum; 5]; 5];

fn bingo_num(n: u32) -> BingoNum {
    BingoNum {num: n, marked: false}
}

fn mark_number(board: &mut Board, n: u32) {
    for line in board.iter_mut() {
        for bingo_num in line.iter_mut() {
            if bingo_num.num == n {
                bingo_num.marked = true
            }
        }
    }
}

fn check_bingo(board: &Board) -> bool {

    // Check rows and cols
    for i in 0..board.len() {

        let mut row_bingo = true;
        let mut col_bingo = true;

        for j in 0..board[0].len() {
            row_bingo &= board[i][j].marked;
            col_bingo &= board[j][i].marked;
        }
        
        if row_bingo | col_bingo {
            return true;
        }
    }

    // No bingo found
    false
}

fn score_board(board: &Board, bingo_num: u32) -> u32 {
    let mut total = 0;
    for row in board {
        for BingoNum { num, marked } in row {
            if !marked {
                total += num;
            }
        }
    }

    total * bingo_num
}


fn parse_input(inp: &str) -> (Vec<u32>, Vec<Board>) {
    let chunks: Vec<&str> = inp.split("\n\n").collect();

    // First chunk is number series
    let series = chunks[0]
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    // Remaining chunks are boards
    let boards = chunks[1..].iter().map(|chunk| {
        parse_board(chunk)
    }).collect();

    (series, boards)
}

fn parse_board(chunk: &str) -> Board {
    // Create empty board
    let mut board = [[bingo_num(0); 5]; 5];

    // Fill out board
    chunk.lines().enumerate().for_each(|(i, l)| {
        l.split_whitespace().enumerate().for_each(|(j, s)| {
            let num = s.parse().unwrap();
            board[i][j] = bingo_num(num);
        })
    });

    board
}

fn main() {
    let raw = include_str!("input.txt");
    let (series, mut boards) = parse_input(raw);

    let mut boards_in_play: Vec<&Board> = boards.iter().collect();

    for n in series {
        
        for board in boards.iter_mut() {
            mark_number(board, n);
        }

        boards_in_play.retain(|board| {
            !check_bingo(board)
        });

        if boards_in_play.len() == 1 {
            println!("answer = {}", score_board(boards_in_play[0], n))
        }
    }
}
