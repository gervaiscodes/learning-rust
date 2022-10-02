enum Action {
    Drive,
    Turn(Direction),
    Stop
}

enum Direction {
    Left,
    Right
}

fn print_action(a: Action) {
    match a {
        Action::Drive => println!("Drive forward"),
        Action::Turn(direction) => match direction {
            Direction::Left => println!("Turn left"),
            Direction::Right => println!("Turn right"),
        }
        Action::Stop => println!("Stop!"),
    }
}

fn main() {
    let program = vec![
        Action::Drive,
        Action::Turn(Direction::Left),
        Action::Turn(Direction::Right),
        Action::Drive,
        Action::Stop
    ];

    for action in program {
        print_action(action)
    }
}
