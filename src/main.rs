use std::io;
use rand::Rng;

#[derive(Copy, Clone, Debug, PartialEq)]
enum States {
    Empty,
    Cross,
    Circle
}

#[derive(Debug)]
struct SituationInMemory {
    board_state_in_memory: [States; 9],
    move_with_valuation: Vec<[u8; 2]>
}

#[derive(Debug)]
struct TempName {
    index: usize,
    possible_moves_length: usize,
    move_done: u8,
}

fn main() {
    let mut board_state: [States; 9] = [States::Empty; 9];
    let mut current_player_state: States = States::Cross;
    let mut current_possible_moves: Vec<u8> = vec![1,2,3,4,5,6,7,8,9];

    println!("Input your move\n|1|2|3|\n|4|5|6|\n|7|8|9|");

    let_ai_give_input([States::Empty; 9],vec![1,2,3,4,5,6,7,8,9], true);

    for mut _i in 0..9 {
        let mut input = 0;

        input = let_ai_give_input(board_state,current_possible_moves.to_owned(), false);
        //io::stdin()
        //    .read_line(&mut input)
        //    .expect("Failed to read line");

        //let input: u8 = match input.trim().parse() {
        //    Ok(num) => num,
        //    Err(_) => continue,
        //};

        if current_possible_moves.contains(&input) {
            let index = current_possible_moves.iter().position(|&r| r == (input)).unwrap();

            current_possible_moves.remove((index) as usize);
            board_state[(input - 1) as usize] = current_player_state;
            display_board(board_state);

            current_player_state = switch_current_state(current_player_state);
        } else {
            _i -= 1;
            println!("test");
        }
        println!("{:?}", current_possible_moves);
    }

    board_state = [States::Empty; 9];
    current_possible_moves = vec![1,2,3,4,5,6,7,8,9];

    let_ai_give_input([States::Empty; 9],vec![1,2,3,4,5,6,7,8,9], true);

    for mut _i in 0..9 {
        let mut input = 0;

        input = let_ai_give_input(board_state,current_possible_moves.to_owned(), false);
        //io::stdin()
        //    .read_line(&mut input)
        //    .expect("Failed to read line");

        //let input: u8 = match input.trim().parse() {
        //    Ok(num) => num,
        //    Err(_) => continue,
        //};

        if current_possible_moves.contains(&input) {
            let index = current_possible_moves.iter().position(|&r| r == (input)).unwrap();

            current_possible_moves.remove((index) as usize);
            board_state[(input - 1) as usize] = current_player_state;
            display_board(board_state);

            current_player_state = switch_current_state(current_player_state);
        } else {
            _i -= 1;
            println!("test");
        }
        println!("{:?}", current_possible_moves);
    }
}

fn run_game_by_ai(){

}


fn let_ai_give_input(board_state: [States; 9], current_possible_moves: Vec<u8>, restart: bool) -> u8{
    let mut situations_in_memory: Vec<SituationInMemory> = Vec::new();
    let mut temp: Vec<TempName> = Vec::new();

    if restart {
        temp = Vec::new();
        return 0;
    }

    //situations_in_memory.push(SituationInMemory{board_state_in_memory: [States::Empty; 9], move_with_valuation: vec![[1,1], [2,1], [3,1],[4,1],[5,1],[6,1],[7,1],[8,1],[9,2]]});
    //situations_in_memory.push(SituationInMemory{board_state_in_memory: [States::Empty,States::Empty,States::Empty,States::Empty,States::Empty,States::Empty,States::Empty,States::Empty,States::Cross], move_with_valuation: vec![[1,1], [2,1], [3,1],[4,1],[5,1],[6,1],[7,3],[8,2]]});
    //situations_in_memory.push(SituationInMemory{board_state_in_memory: [States::Empty,States::Empty,States::Empty,States::Empty,States::Empty,States::Empty,States::Circle,States::Empty,States::Cross], move_with_valuation: vec![[1,1], [2,1], [3,1],[4,1],[5,1],[6,1]]});

    let mut _move: u8 = 0;
    let index = find_situation_in_vector(&situations_in_memory, board_state);
    println!("{:?}", board_state);
    println!("{:?}", situations_in_memory);
    println!("{:?}", current_possible_moves.len());

    if index > -1 {
        println!("situations_in_memory[index as usize].move_with_valuation.len()) {:?}", situations_in_memory[index as usize].move_with_valuation.len());
        println!("current_possible_moves_length {:?}", current_possible_moves.len());
        if situations_in_memory[index as usize].move_with_valuation.len() == current_possible_moves.len() {
            println!("Test");
            for _i in 0..current_possible_moves.len() {
                if situations_in_memory[index as usize].move_with_valuation[_i][1] > _move{
                    _move = situations_in_memory[index as usize].move_with_valuation[_i][0];
                    println!("move: {:?}", _move);
                }
            }
        }else {
            _move = current_possible_moves[rand::thread_rng().gen_range(0, current_possible_moves.len() as u8) as usize] as u8;
            temp.push(TempName{ index: index as usize, possible_moves_length: current_possible_moves.len(), move_done: _move});
            println!("test: {:?}", temp);
            println!("move: {:?}", _move);
        }
    }else {
        _move = current_possible_moves[rand::thread_rng().gen_range(0, current_possible_moves.len() as u8) as usize] as u8;
        situations_in_memory.push(SituationInMemory{board_state_in_memory:board_state, move_with_valuation: vec![]});
        temp.push(TempName{ index: situations_in_memory.len() +1, possible_moves_length: current_possible_moves.len(), move_done: _move});
        println!("test: {:?}", temp);
        println!("move: {:?}", _move);

    }
    println!("{}", _move);
    println!("move: {:?}", _move);
    return _move;
}

fn find_situation_in_vector(situations_in_memory: &Vec<SituationInMemory>, board_state_in_memory: [States; 9])  -> i32{
    if situations_in_memory.len() > 0 {
        for i in 0..situations_in_memory.len() {
            if situations_in_memory[i].board_state_in_memory == board_state_in_memory {
                println!("{}", i);
                return i as i32
            }
        }
    }
    println!("{}", -1);
    return -1;
}

fn display_board(board_state: [States; 9]) {
    display_row([board_state[0], board_state[1], board_state[2]]);
    println!("------|------|------");
    display_row([board_state[3], board_state[4], board_state[5]]);
    println!("------|------|------");
    display_row([board_state[6], board_state[7], board_state[8]]);
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

fn switch_current_state(current_player_state: States) -> States{
    return if current_player_state == States::Cross {
        States::Circle
    } else if current_player_state == States::Circle {
        States::Cross
    } else {
        //will never happen
        States::Empty
    }
}