use ncurses::*;
use rand::Rng;

fn main() {
    initscr();
    raw();
    keypad(stdscr(), true);
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let game_width = 6 * 4 + 2;
    let game_height = 3 * 4 + 2;
    let mut win_width = 0;
    let mut win_height = 0;
    getmaxyx(stdscr(), &mut win_height, &mut win_width);

    let x = (win_width - game_width) / 2;
    let y = (win_height - game_height) / 2;
    let game_window = newwin(game_height, game_width, y, x);

    // init game
    let mut won = false;
    let mut game_board = vec![0; 16];
    spawn_block(&mut game_board);

    // input window
    let iw = newwin(0, 0, 0, 0);
    wrefresh(iw);

    loop {
        if game_board.iter().any(|&x| x >= 2048) {
            won = true;
            break;
        }

        box_(game_window, 0, 0);
        wrefresh(game_window);
        draw_board(&mut game_board, (x, y));

        let ch = wgetch(iw) as u8 as char;
        if ch == 'q' {
            break;
        }

        if slide_board(&mut game_board, ch) && !spawn_block(&mut game_board) {
            break;
        }
    }

    if won {
        addstr("You won!").unwrap();
    } else {
        addstr("You lost...").unwrap();
    }
    getch();
    endwin();
}

fn spawn_block(game_board: &mut [i32]) -> bool {
    let mut rng = rand::thread_rng();

    let mut positions = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    for _i in 0..16 {
        let index = rng.gen_range(0..16);
        let position = positions[index] as usize;

        if game_board[position] == 0 {
            game_board[position] = 2 << rng.gen_range(0..=1); // set game board value to 2 or 4
            return true;
        }
        positions.remove(index);
    }
    false
}

fn draw_board(game_board: &mut [i32], origin: (i32, i32)) {
    let x_orig = origin.0;
    let y_orig = origin.1;

    for (i, cell) in game_board.iter().enumerate().take(16) {
        if *cell == 0 {
            continue;
        }
        let x = ((i % 4) * 6) as i32 + x_orig;
        let y = ((i / 4) * 3) as i32 + y_orig;

        let tmp_win = newwin(3, 6, y + 1, x + 1);
        box_(tmp_win, 0, 0);
        mvwaddstr(tmp_win, 1, 1, format!("{}", *cell).as_str()).unwrap();
        wrefresh(tmp_win);
    }
}

fn slide_board(game_board: &mut [i32], input: char) -> bool {
    match input {
        'h' | 'j' | 'k' | 'l' => {}
        _ => return false,
    }

    let mut board_2d = vec![vec![0; 4]; 4];
    inflate_board(game_board, &mut board_2d);

    // flip the board so that we only need to implement the slide function for one direction
    flip_board(&mut board_2d, input, false);

    for y in 1..4 {
        for x in 0..4 {
            if board_2d[y][x] == 0 {
                continue;
            }

            let val = board_2d[y][x];

            let mut next_y = y - 1;
            let mut current_y = y;
            loop {
                if board_2d[next_y][x] == val {
                    board_2d[current_y][x] = 0;
                    board_2d[next_y][x] += val;
                    break;
                } else if board_2d[next_y][x] == 0 {
                    board_2d[current_y][x] = 0;
                    board_2d[next_y][x] = val;
                } else {
                    break;
                }

                if next_y == 0 {
                    break;
                }
                current_y = next_y;
                next_y -= 1;
            }
        }
    }

    // reversing the board fliping
    flip_board(&mut board_2d, input, true);
    flatten_board(&board_2d, game_board);
    true
}

fn inflate_board(board: &[i32], board_2d: &mut [Vec<i32>]) {
    for i in 0..16 {
        board_2d[i / 4][i % 4] = board[i];
    }
}

fn flatten_board(board_2d: &[Vec<i32>], board: &mut [i32]) {
    let mut i = 0;
    for row in board_2d {
        for cell in row {
            board[i] = *cell;
            i += 1;
        }
    }
}

fn flip_board(board: &mut [Vec<i32>], input: char, restore: bool) {
    match input {
        'h' => {
            for y in 0..4 {
                for x in y + 1..4 {
                    let tmp = board[y][x];
                    board[y][x] = board[x][y];
                    board[x][y] = tmp;
                }
            }
        }
        'j' => {
            board.reverse();
        }
        'k' => {}
        'l' => {
            if restore {
                board.reverse();
            }

            for y in 0..4 {
                for x in y + 1..4 {
                    let tmp = board[y][x];
                    board[y][x] = board[x][y];
                    board[x][y] = tmp;
                }
            }

            if !restore {
                board.reverse();
            }
        }
        _ => {}
    }
}
