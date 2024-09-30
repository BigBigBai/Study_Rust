use std::char;
use std::io;

fn main() {
    // Initialize the board
    let mut board = [['.'; 8]; 8];
    for i in 0..8 {
        for j in 0..8 {
            if (i == 3 && j == 3) || (i == 4 && j == 4) {
                board[i][j] = 'W';
            } else if (i == 3 && j == 4) || (i == 4 && j == 3) {
                board[i][j] = 'B';
            }
        }
    }
    print_board(&board);

    let mut count = 1;
    loop {
        // count: odd <=> color: black; count: even <=> color: white
        let mut color = 'B';
        let mut oppo_color = 'W';
        if count % 2 == 0 {
            color = 'W';
            oppo_color = 'B';
        }

        // Check if game over (both has no valid move)
        // Check if current player has valid move
        let mut count_black = 0;
        let mut count_white = 0;
        let mut exist_valid_move = false; // for color
        let mut exist_valid_move_for_oppo = false; // for oppo_color
        for i in 0..8 {
            for j in 0..8 {
                if board[i][j] == '.' {
                    if check_loc(i as i32, j as i32, &mut board, color, oppo_color, false) {
                        exist_valid_move = true;
                        break;
                    }

                    if check_loc(i as i32, j as i32, &mut board, oppo_color, color, false) {
                        exist_valid_move_for_oppo = true;
                    }
                } else if board[i][j] == 'B' {
                    count_black += 1;
                } else {
                    count_white += 1;
                }
            }
        }

        let black_minus_white = count_black - count_white;
        if !exist_valid_move && !exist_valid_move_for_oppo {
            println!("{} player has no valid move.", color);
            println!("{} player has no valid move.", oppo_color);
            if black_minus_white == 0 {
                println!("Draw!");
            } else if black_minus_white > 0 {
                println!("Black wins by {} points!", black_minus_white);
            } else {
                println!("White wins by {} points!", count_white - count_black);
            }
            break;
        } else if !exist_valid_move {
            println!("{} player has no valid move.", color);
            count += 1;
            continue;
        }

        println!("Enter move for color {} (RowCol): ", color);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input: String = match input.trim().parse() {
            Ok(s) => s,
            Err(_) => continue,
        };
        println!("{}", input);

        let input: Vec<_> = input.chars().collect();
        if input.len() != 2 {
            println!("Invalid move. Try again.");
            print_board(&board);
            continue;
        }

        let bi = input[0] as i32 - 97;
        let bj = input[1] as i32 - 97;
        if bi < 0 || bi >= 8 || bj < 0 || bj >= 8 {
            println!("Invalid move. Try again.");
            print_board(&board);
            continue;
        }

        // Check if there exist valid move
        // 1.Check if it is '.'
        // 2.Check if 8 corners have oppo_color
        // 3.Check if straight line has color
        if board[bi as usize][bj as usize] == '.'
            && check_loc(bi as i32, bj as i32, &mut board, color, oppo_color, true)
        {
            count += 1;
        } else {
            println!("Invalid mov. Try again.");
        }

        print_board(&board);
    }
}

fn check_loc(
    bi: i32,
    bj: i32,
    board: &mut [[char; 8]; 8],
    color: char,
    oppo_color: char,
    update: bool,
) -> bool {
    let mut found = false;
    let dirs = [
        [-1, 0],
        [1, 0],
        [0, -1],
        [0, 1],
        [-1, -1],
        [-1, 1],
        [1, -1],
        [1, 1],
    ];
    for dir in dirs {
        let ni = bi + dir[0];
        let nj = bj + dir[1];

        if ni >= 0 && ni < 8 && nj >= 0 && nj < 8 && board[ni as usize][nj as usize] == oppo_color {
            if check(ni, nj, board, color, oppo_color, update, &dir) {
                found = true;
                if update {
                    board[bi as usize][bj as usize] = color;
                }
            }
        }
    }

    found
}

fn check(
    ni: i32,
    nj: i32,
    board: &mut [[char; 8]; 8],
    color: char,
    oppo_color: char,
    update: bool,
    dir: &[i32; 2],
) -> bool {
    let mut flag = false;

    if ni >= 0 && ni < 8 && nj >= 0 && nj < 8 {
        if board[ni as usize][nj as usize] == color {
            return true;
        } else if board[ni as usize][nj as usize] == oppo_color {
            flag = check(
                ni + dir[0],
                nj + dir[1],
                board,
                color,
                oppo_color,
                update,
                dir,
            );
        } else {
            return false;
        }
    } else {
        return false;
    }

    if flag && update {
        board[ni as usize][nj as usize] = color;
    }
    flag
}

fn print_board(board: &[[char; 8]; 8]) {
    println!("{}", "  abcdefgh");

    for i in 0..8 {
        let c = char::from_u32((i + 97) as u32).unwrap();
        print!("{} ", c);

        for j in 0..8 {
            print!("{}", board[i][j]);
        }
        println!();
    }
}
