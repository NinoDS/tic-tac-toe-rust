use std::io;
use std::cmp::max;
use std::cmp::min;
use rand::Rng;

#[derive(Copy, Clone, Debug, PartialEq)]
enum States {
    Empty,
    Cross,
    Circle,
}


fn main() {

    //START GAME
    start_game()
}


//########################################################
//                     BASIC GAME
//########################################################


fn start_game(){

    //INITIALIZE VARIABLES
    let board_state: [States;9] = [ States::Empty, States::Empty, States::Empty,
                                    States::Empty, States::Empty, States::Empty,
                                    States::Empty, States::Empty, States::Empty];

    //will be implemented later
    //SET DEFAULT SETTINGS
    //let _send_debug_messages: bool = true;

    //START GAME LOOP
    game_loop(board_state);
}

fn game_loop(mut board_state: [States;9]){
    loop {
        display_board(board_state);
        while evaluate_board_state(board_state) == 0 {
            let mut current_possible_moves = calculate_current_possible_moves(board_state);

            //get input from AI
            let mut input: usize = find_best_move(board_state) as usize;
            println!("**Debug Message**: the evaluation of the board state was : {}", evaluate_board_state(board_state));

            board_state[input] = States::Cross;

            display_board(board_state);

            current_possible_moves = calculate_current_possible_moves(board_state);

            if current_possible_moves.len() == 0 {
                break;
            }

            //get input from player
            input = get_human_input(board_state);

            if input != 0 {
                board_state[input - 1] = States::Circle;
            }

            display_board(board_state);

            if evaluate_board_state(board_state) == -1 {
                panic!("**Debug Message**:Something went wrong! Please check the Minimax code! Line:70") //Line number can change
            }

        }
        board_state = [States::Empty; 9]
    }

}

fn get_human_input(mut board_state: [States;9]) -> usize {
    let current_possible_moves: Vec<usize> = calculate_current_possible_moves(board_state);
    while evaluate_board_state(board_state) == 0 && current_possible_moves.len() != 0 {
        println!("Please input your move.");

        let mut input_string = String::new();

        io::stdin()
            .read_line(&mut input_string)
            .expect("Failed to read line");

        let input: usize = match input_string.trim().parse() {
            Ok(num) => num,
            Err(_) => {println!("An error has occurred. Please input an integer between {} and {}",u32::MIN, u32::MAX); continue},
        };

        if current_possible_moves.contains(&input) {
            return input;
        }
    }
    return 0;
}

fn calculate_current_possible_moves(board_state: [States; 9]) -> Vec<usize>{
    let mut current_possible_moves: Vec<usize> = vec![];
    for i in 0..9 {
        if board_state[i] == States::Empty {
            current_possible_moves.push(i + 1);
        }
    }
    return current_possible_moves;
}

fn evaluate_board_state(board_state:[States;9]) -> i8{

    //CHECK FOR COLUMNS
    for i in 0..3 {

        if board_state[i] == board_state[i + 3] &&  board_state[i] == board_state[(i + 6)] && board_state[i] != States::Empty{
            return if board_state[i * 3] == States::Cross {
                1
            } else {
                -1
            }
        }

    }

    //CHECK FOR ROWS
    for i in 0..3 {

        if board_state[i*3] == board_state[i*3 + 1] &&  board_state[i*3] == board_state[i*3 + 2] && board_state[i*3] != States::Empty{
            return if board_state[i * 3] == States::Cross {
                1
            } else {
                -1
            }
        }

    }

    //CHECK FOR DIAGONALS
    if board_state[0] == board_state[4] &&  board_state[0] == board_state[8] && board_state[0] != States::Empty{
        return if board_state[0] == States::Cross {
            1
        } else {
            -1
        }
    }
    if board_state[2] == board_state[4] &&  board_state[2] == board_state[6] && board_state[2] != States::Empty{
        return if board_state[2] == States::Cross {
            1
        } else {
            -1
        }
    }


    return 0;
}

fn moves_left(board_state:[States;9]) -> bool {
    for i in 0..9 {
        if board_state[i] == States::Empty {
            return true;
        }
    }
    return false;
}

//########################################################
//                BASIC GAME: DISPLAY
//########################################################

fn display_board(board_state: [States; 9]) {
    display_row([board_state[0], board_state[1], board_state[2]]);
    println!("------|------|------");
    display_row([board_state[3], board_state[4], board_state[5]]);
    println!("------|------|------");
    display_row([board_state[6], board_state[7], board_state[8]]);
    println!();
}

fn display_row(row: [States; 3]) {
    for i in 0..4 {
        println!(
            "{}|{}|{}",
            get_state_display_parts(row[0])[i],
            get_state_display_parts(row[1])[i],
            get_state_display_parts(row[2])[i],
        )
    }
}

fn get_state_display_parts(state: States) -> [&'static str; 4] {
    let field_type_empty: [&str; 4] = ["      "; 4];
    let field_type_cross: [&str; 4] = [" \\  / ", "  \\/  ", "  /\\  ", " /  \\ "];
    let field_type_circle: [&str; 4] = ["  __  ", " /  \\ ", " \\__/ ", "      "];

    return if state == States::Empty {
        field_type_empty
    } else if state == States::Cross {
        field_type_cross
    } else {
        //states == States:Circle
        field_type_circle
    };
}




//########################################################
//                      MINIMAX
//########################################################


fn find_best_move(mut board_state:[States;9]) -> i16{

    let mut best_move: i16 = -1;
    let mut best_valuation: i8 = i8::MIN;

    for i in 0..9 {

        if board_state[i] == States::Empty {

            //do the move
            board_state[i] = States::Cross;

            //let minimax compute valuation
            let move_valuation: i8 = minimax(board_state, 0, false);

            //undo move
            board_state[i] = States::Empty;

            if move_valuation > best_valuation {
                best_move = i as i16;
                best_valuation = move_valuation;
            }
        }

    }
    println!("**Debug Message**: The value of the best move is: {} \n", best_valuation);
    println!("**Debug Message**: The best move is: {} \n", best_move);


    return best_move;
}

fn minimax(mut board_state: [States;9], depth: i8, is_maximizing_player: bool) -> i8 {
    let score: i8 = evaluate_board_state(board_state);

    //maximizer won
    if score == 1 {
        return score;
    }

    //minimizer won
    if score == -1 {
        return score;
    }

    //if nobody has won and there are no more moves available, it is a tie
    if !moves_left(board_state) {
        return 0;
    }

    return if is_maximizing_player {
        let mut best: i8 = i8::MIN;

        for i in 0..9 {
            if board_state[i] == States::Empty {
                //do move
                board_state[i] = States::Cross;

                best = max(best, minimax(board_state, depth + 1, !is_maximizing_player));

                //undo move
                board_state[i] = States::Empty;
            }
        }
        best

    } else {

        let mut best: i8 = i8::MAX;

        for i in 0..9 {
            if board_state[i] == States::Empty {
                //do move
                board_state[i] = States::Circle;

                best = min(best, minimax(board_state, depth + 1, !is_maximizing_player));

                //undo move
                board_state[i] = States::Empty;
            }
        }
        best
    };
}
