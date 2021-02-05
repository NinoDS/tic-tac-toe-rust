#[derive(Copy, Clone, Debug, PartialEq)]
enum States {
    Empty,
    Cross,
    Circle,
}

fn main() {
    let mut board_state: [States;9] = [States::Cross; 9];

    display_board(board_state);
}

fn display_board(board_state: [States;9]){
    display_row([board_state[0], board_state[1], board_state[2]]);
    println!("----|----|----");
    display_row([board_state[3], board_state[4], board_state[5]]);
    println!("----|----|----");
    display_row([board_state[6], board_state[7], board_state[8]]);
}

fn display_row(row: [States;3]){
    for i in 0..4 {
        println!(
            "{}|{}|{}",
            get_state_display_parts(row[0])[i],
            get_state_display_parts(row[1])[i],
            get_state_display_parts(row[2])[i],
        )
    }
}

fn get_state_display_parts(state: States) -> [&'static str;4] {
    let field_type_empty: [&str;4] = ["    "; 4];
    let field_type_cross: [&str;4] = ["\\  /", " \\/ "," /\\ ", "/  \\"];
    let field_type_circle: [&str;4] = ["    "; 4];

    return if state == States::Empty {
        field_type_empty
    } else if state == States::Cross {
        field_type_cross
    } else { //states == States:Circle
        field_type_circle
    }
}
