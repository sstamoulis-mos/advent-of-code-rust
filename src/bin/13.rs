advent_of_code::solution!(13);

mod part_one {
    use std::sync::OnceLock;

    use regex::Regex;

    pub enum Button {
        A,
        B,
    }

    impl Button {
        pub const fn cost(&self) -> u64 {
            match self {
                Button::A => 3,
                Button::B => 1,
            }
        }
    }

    pub const MAX_BUTTON_PRESSES: u64 = 100;

    #[derive(Debug, PartialEq)]
    pub struct Position {
        x: u64,
        y: u64,
    }

    pub struct Machine {
        a_move: Position,
        b_move: Position,
        prize_position: Position,
    }

    static MACHINE_REGEX: OnceLock<Regex> = OnceLock::new();

    // const NEW_LINE: &str = if cfg!(windows) { "\r\n" } else { "\n" };
    const NEW_LINE: &str = "\n";
    fn compile_machine_regex() -> Regex {
        Regex::new(
            &[
                r"Button A: X\+(?<ax>\d+), Y\+(?<ay>\d+)",
                r"Button B: X\+(?<bx>\d+), Y\+(?<by>\d+)",
                r"Prize: X=(?<px>\d+), Y=(?<py>\d+)",
            ]
            .join(NEW_LINE),
        )
        .unwrap()
    }

    impl Machine {
        pub fn from_input(value: &str) -> Vec<Machine> {
            let machine_regex = MACHINE_REGEX.get_or_init(compile_machine_regex);
            machine_regex
                .captures_iter(value)
                // .inspect(|cap| println!("{:#?}", cap))
                .map(|cap| {
                    let ax = cap["ax"].parse::<u64>().unwrap();
                    let ay = cap["ay"].parse::<u64>().unwrap();
                    let bx = cap["bx"].parse::<u64>().unwrap();
                    let by = cap["by"].parse::<u64>().unwrap();
                    let px = cap["px"].parse::<u64>().unwrap();
                    let py = cap["py"].parse::<u64>().unwrap();
                    let a_move = Position { x: ax, y: ay };
                    let b_move = Position { x: bx, y: by };
                    let prize_position = Position { x: px, y: py };
                    Machine {
                        a_move,
                        b_move,
                        prize_position,
                    }
                })
                .collect()
        }

        pub fn min_cost(&self) -> Option<u64> {
            let mut min_cost = None;

            for a_presses in 0..=MAX_BUTTON_PRESSES {
                for b_presses in 0..=MAX_BUTTON_PRESSES {
                    let position_result = Position {
                        x: self.a_move.x * a_presses + self.b_move.x * b_presses,
                        y: self.a_move.y * a_presses + self.b_move.y * b_presses,
                    };
                    if position_result == self.prize_position {
                        let a_cost = Button::A.cost() * a_presses;
                        let b_cost = Button::B.cost() * b_presses;
                        let total_cost = a_cost + b_cost;
                        if total_cost < min_cost.unwrap_or(u64::MAX) {
                            min_cost = Some(total_cost);
                        }
                    }
                }
            }

            min_cost
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_machine_regex() {
            let test_text = [
                "Button A: X+94, Y+34",
                "Button B: X+22, Y+67",
                "Prize: X=8400, Y=5400",
            ]
            .join(NEW_LINE);
            let machine_regex = MACHINE_REGEX.get_or_init(compile_machine_regex);
            let cap = machine_regex.captures(&test_text).unwrap();
            println!("{test_text}");
            println!("{:#?}", cap);
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    use part_one::Machine;

    let mut total_cost = 0;

    let machines = Machine::from_input(input);
    for machine in machines {
        if let Some(min_cost) = machine.min_cost() {
            total_cost += min_cost;
        }
    }

    Some(total_cost)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
