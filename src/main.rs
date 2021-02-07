use std::io;

#[derive(Copy, Clone, Debug, PartialEq)]
enum States {
    Empty,
    Cross,
    Circle,
}

fn main() {
    let mut board_state: [States; 9] = [States::Empty; 9];
    let mut current_player_state: States = States::Cross;

    println!("Input your move\n|1|2|3|\n|4|5|6|\n|7|8|9|");

    for mut _i in 0..9 {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        board_state[(input - 1) as usize] = current_player_state;
        display_board(board_state);

        current_player_state = switch_current_state(current_player_state);
    }
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