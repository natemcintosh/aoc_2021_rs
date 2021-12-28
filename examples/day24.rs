use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct State(i64, i64, i64, i64);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Instruction {
    Inp { field_idx: u8, val: Option<i64> },
    Add { field_idx: u8, scnd_arg: IdxOrVal },
    Mul { field_idx: u8, scnd_arg: IdxOrVal },
    Div { field_idx: u8, scnd_arg: IdxOrVal },
    Mod { field_idx: u8, scnd_arg: IdxOrVal },
    Eql { field_idx: u8, scnd_arg: IdxOrVal },
}

fn parse_program(s: &str) -> Vec<Instruction> {
    s.lines().map(parse_line).collect()
}

fn run_program(program: &[Instruction], arguments: &VecDeque<i64>) -> State {
    // Initiate State
    let mut state = State::default();

    // Make a copy of the input arguments
    let mut args = arguments.clone();

    // Run each Instruction in p
    for instruction in program {
        match instruction {
            Instruction::Inp { field_idx, val: _ } => {
                // Pop an instruction from the front of the arguments deque, and put it
                // in the proper location
                let val_to_input = args.pop_front().expect("Could not pop argument from front");

                let mut w = state.0;
                let mut x = state.1;
                let mut y = state.2;
                let mut z = state.3;
                match field_idx {
                    0 => {
                        w = val_to_input;
                    }
                    1 => {
                        x = val_to_input;
                    }
                    2 => {
                        y = val_to_input;
                    }
                    3 => {
                        z = val_to_input;
                    }
                    _ => unreachable!(),
                }
                state = State(w, x, y, z);
            }
            Instruction::Add {
                field_idx,
                scnd_arg,
            } => {
                // Add the item at `field_idx` and the `scnd_arg`, and store the result
                // in the item at `field_idx`

                // Get all the items
                let mut w = state.0;
                let mut x = state.1;
                let mut y = state.2;
                let mut z = state.3;

                // Get the item at field_idx
                let first_val = match field_idx {
                    0 => w,
                    1 => x,
                    2 => y,
                    3 => z,
                    _ => unreachable!(),
                };

                // Add the scnd_arg to the item at the field_idx.
                let second_val = match scnd_arg {
                    IdxOrVal::Idx(idx) => match idx {
                        0 => w,
                        1 => x,
                        2 => y,
                        3 => z,
                        _ => unreachable!(),
                    },
                    IdxOrVal::Val(val) => *val,
                };

                // Calculate the result
                let result = first_val + second_val;

                // Alter the proper field
                match field_idx {
                    0 => w = result,
                    1 => x = result,
                    2 => y = result,
                    3 => z = result,
                    _ => unreachable!(),
                };
                // Store the result in the place of the first
                state = State(w, x, y, z);
            }
            Instruction::Mul {
                field_idx,
                scnd_arg,
            } => {
                // Multiply the item at `field_idx` and the `scnd_arg`, and store the result
                // in the item at `field_idx`

                // Get all the items
                let mut w = state.0;
                let mut x = state.1;
                let mut y = state.2;
                let mut z = state.3;

                // Get the item at field_idx
                let first_val = match field_idx {
                    0 => w,
                    1 => x,
                    2 => y,
                    3 => z,
                    _ => unreachable!(),
                };

                // Add the scnd_arg to the item at the field_idx.
                let second_val = match scnd_arg {
                    IdxOrVal::Idx(idx) => match idx {
                        0 => w,
                        1 => x,
                        2 => y,
                        3 => z,
                        _ => unreachable!(),
                    },
                    IdxOrVal::Val(val) => *val,
                };

                // Calculate the result
                let result = first_val * second_val;

                // Alter the proper field
                match field_idx {
                    0 => w = result,
                    1 => x = result,
                    2 => y = result,
                    3 => z = result,
                    _ => unreachable!(),
                };
                // Store the result in the place of the first
                state = State(w, x, y, z);
            }
            Instruction::Div {
                field_idx,
                scnd_arg,
            } => {
                // Divide the item at `field_idx` and the `scnd_arg`, and store the result
                // in the item at `field_idx`

                // Get all the items
                let mut w = state.0;
                let mut x = state.1;
                let mut y = state.2;
                let mut z = state.3;

                // Get the item at field_idx
                let first_val = match field_idx {
                    0 => w,
                    1 => x,
                    2 => y,
                    3 => z,
                    _ => unreachable!(),
                };

                // Add the scnd_arg to the item at the field_idx.
                let second_val = match scnd_arg {
                    IdxOrVal::Idx(idx) => match idx {
                        0 => w,
                        1 => x,
                        2 => y,
                        3 => z,
                        _ => unreachable!(),
                    },
                    IdxOrVal::Val(val) => *val,
                };

                // Calculate the result
                let result = first_val / second_val;

                // Alter the proper field
                match field_idx {
                    0 => w = result,
                    1 => x = result,
                    2 => y = result,
                    3 => z = result,
                    _ => unreachable!(),
                };
                // Store the result in the place of the first
                state = State(w, x, y, z);
            }
            Instruction::Mod {
                field_idx,
                scnd_arg,
            } => {
                // Modulus the item at `field_idx` and the `scnd_arg`, and store the result
                // in the item at `field_idx`

                // Get all the items
                let mut w = state.0;
                let mut x = state.1;
                let mut y = state.2;
                let mut z = state.3;

                // Get the item at field_idx
                let first_val = match field_idx {
                    0 => w,
                    1 => x,
                    2 => y,
                    3 => z,
                    _ => unreachable!(),
                };

                // Add the scnd_arg to the item at the field_idx.
                let second_val = match scnd_arg {
                    IdxOrVal::Idx(idx) => match idx {
                        0 => w,
                        1 => x,
                        2 => y,
                        3 => z,
                        _ => unreachable!(),
                    },
                    IdxOrVal::Val(val) => *val,
                };

                // Calculate the result
                let result = first_val % second_val;

                // Alter the proper field
                match field_idx {
                    0 => w = result,
                    1 => x = result,
                    2 => y = result,
                    3 => z = result,
                    _ => unreachable!(),
                };
                // Store the result in the place of the first
                state = State(w, x, y, z);
            }
            Instruction::Eql {
                field_idx,
                scnd_arg,
            } => {
                // If the two items are equal, store the value 1 in the field_idx, otherwise 0

                // Get all the items
                let mut w = state.0;
                let mut x = state.1;
                let mut y = state.2;
                let mut z = state.3;

                // Get the item at field_idx
                let first_val = match field_idx {
                    0 => w,
                    1 => x,
                    2 => y,
                    3 => z,
                    _ => unreachable!(),
                };

                // Add the scnd_arg to the item at the field_idx.
                let second_val = match scnd_arg {
                    IdxOrVal::Idx(idx) => match idx {
                        0 => w,
                        1 => x,
                        2 => y,
                        3 => z,
                        _ => unreachable!(),
                    },
                    IdxOrVal::Val(val) => *val,
                };

                // Calculate the result
                let result = if first_val == second_val { 1 } else { 0 };

                // Alter the proper field
                match field_idx {
                    0 => w = result,
                    1 => x = result,
                    2 => y = result,
                    3 => z = result,
                    _ => unreachable!(),
                };
                // Store the result in the place of the first
                state = State(w, x, y, z);
            }
        }
    }

    state
}

fn parse_line(s: &str) -> Instruction {
    // Split it by spaces.
    let mut items = s.trim().split_ascii_whitespace();

    // The first item is the instruction
    match items.next() {
        Some(ins) => match ins {
            "inp" => {
                let field_name = items
                    .next()
                    .expect("Could not retrieve w,x,y,z for 'inp' instruction");
                let field_idx: u8 = match field_name {
                    "w" => 0,
                    "x" => 1,
                    "y" => 2,
                    "z" => 3,
                    _ => panic!("input field, {:?}, was not w,x,y,z", field_name),
                };
                Instruction::Inp {
                    field_idx,
                    val: None,
                }
            }
            "add" => {
                let field_name = items
                    .next()
                    .expect("Could not retrieve w,x,y,z for 'add' instruction");
                let field_idx: u8 = match field_name {
                    "w" => 0,
                    "x" => 1,
                    "y" => 2,
                    "z" => 3,
                    _ => panic!("input field, {:?}, was not w,x,y,z", field_name),
                };

                let scnd_arg = items
                    .next()
                    .expect("Could not get second arg for 'add' instruction");
                let scnd_arg = if ["w", "x", "y", "z"].contains(&scnd_arg) {
                    IdxOrVal::Idx(match scnd_arg {
                        "w" => 0,
                        "x" => 1,
                        "y" => 2,
                        "z" => 3,
                        _ => panic!("input field, {:?}, was not w,x,y,z", field_name),
                    })
                } else {
                    IdxOrVal::Val(i64::from_str_radix(scnd_arg, 10).expect("Could not parse value"))
                };

                Instruction::Add {
                    field_idx,
                    scnd_arg,
                }
            }
            "mul" => {
                let field_name = items
                    .next()
                    .expect("Could not retrieve w,x,y,z for 'mul' instruction");
                let field_idx: u8 = match field_name {
                    "w" => 0,
                    "x" => 1,
                    "y" => 2,
                    "z" => 3,
                    _ => panic!("input field, {:?}, was not w,x,y,z", field_name),
                };

                let scnd_arg = items
                    .next()
                    .expect("Could not get second arg for 'mul' instruction");
                let scnd_arg = if ["w", "x", "y", "z"].contains(&scnd_arg) {
                    IdxOrVal::Idx(match scnd_arg {
                        "w" => 0,
                        "x" => 1,
                        "y" => 2,
                        "z" => 3,
                        _ => panic!("input field, {:?}, was not w,x,y,z", field_name),
                    })
                } else {
                    IdxOrVal::Val(i64::from_str_radix(scnd_arg, 10).expect("Could not parse value"))
                };

                Instruction::Mul {
                    field_idx,
                    scnd_arg,
                }
            }
            "div" => {
                let field_name = items
                    .next()
                    .expect("Could not retrieve w,x,y,z for 'div' instruction");
                let field_idx: u8 = match field_name {
                    "w" => 0,
                    "x" => 1,
                    "y" => 2,
                    "z" => 3,
                    _ => panic!("input field, {:?}, was not w,x,y,z", field_name),
                };

                let scnd_arg = items
                    .next()
                    .expect("Could not get second arg for 'div' instruction");
                let scnd_arg = if ["w", "x", "y", "z"].contains(&scnd_arg) {
                    IdxOrVal::Idx(match scnd_arg {
                        "w" => 0,
                        "x" => 1,
                        "y" => 2,
                        "z" => 3,
                        _ => panic!("input field, {:?}, was not w,x,y,z", field_name),
                    })
                } else {
                    IdxOrVal::Val(i64::from_str_radix(scnd_arg, 10).expect("Could not parse value"))
                };

                Instruction::Div {
                    field_idx,
                    scnd_arg,
                }
            }
            "mod" => {
                let field_name = items
                    .next()
                    .expect("Could not retrieve w,x,y,z for 'mod' instruction");
                let field_idx: u8 = match field_name {
                    "w" => 0,
                    "x" => 1,
                    "y" => 2,
                    "z" => 3,
                    _ => panic!("input field, {:?}, was not w,x,y,z", field_name),
                };

                let scnd_arg = items
                    .next()
                    .expect("Could not get second arg for 'mod' instruction");
                let scnd_arg = if ["w", "x", "y", "z"].contains(&scnd_arg) {
                    IdxOrVal::Idx(match scnd_arg {
                        "w" => 0,
                        "x" => 1,
                        "y" => 2,
                        "z" => 3,
                        _ => panic!("input field, {:?}, was not w,x,y,z", field_name),
                    })
                } else {
                    IdxOrVal::Val(i64::from_str_radix(scnd_arg, 10).expect("Could not parse value"))
                };

                Instruction::Mod {
                    field_idx,
                    scnd_arg,
                }
            }
            "eql" => {
                let field_name = items
                    .next()
                    .expect("Could not retrieve w,x,y,z for 'eql' instruction");
                let field_idx: u8 = match field_name {
                    "w" => 0,
                    "x" => 1,
                    "y" => 2,
                    "z" => 3,
                    _ => panic!("input field, {:?}, was not w,x,y,z", field_name),
                };

                let scnd_arg = items
                    .next()
                    .expect("Could not get second arg for 'eql' instruction");
                let scnd_arg = if ["w", "x", "y", "z"].contains(&scnd_arg) {
                    IdxOrVal::Idx(match scnd_arg {
                        "w" => 0,
                        "x" => 1,
                        "y" => 2,
                        "z" => 3,
                        _ => panic!("input field, {:?}, was not w,x,y,z", field_name),
                    })
                } else {
                    IdxOrVal::Val(i64::from_str_radix(scnd_arg, 10).expect("Could not parse value"))
                };

                Instruction::Eql {
                    field_idx,
                    scnd_arg,
                }
            }
            _ => panic!("instructions string {:?} was unexpected", ins),
        },
        None => panic!("Found nothing on this line of instructions"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IdxOrVal {
    Idx(u8),
    Val(i64),
}

fn digits(n: usize) -> VecDeque<i64> {
    fn inner(n: usize, xs: &mut VecDeque<i64>) {
        if n >= 10 {
            inner(n / 10, xs);
        }
        xs.push_back((n % 10) as i64);
    }
    let mut xs = VecDeque::new();
    inner(n, &mut xs);
    xs
}

fn digits_to_number(digits: &VecDeque<i64>) -> i64 {
    digits
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, val)| *val * 10_i64.pow(idx as u32))
        .sum()
}

fn part1(monad: &[Instruction]) -> usize {
    // Need to run increasingly smaller numbers through the monad until receiving a 0 in
    // the z space. Return that number

    // Start at 99999999999999 and count downwards
    let answer = (11111111111111..99999999999999)
        .rev()
        // Get the digits of each number
        .inspect(|x| {
            if x % 10_000_000 == 0 {
                println!("{}", x)
            }
        })
        .map(digits)
        // Remove any that have 0 in them
        .filter(|v| !v.contains(&0))
        // Run the program
        .map(|v| (v.clone(), run_program(monad, &v)))
        // Find one with 0 in the z space
        .find(|(_, state)| state.3 == 0)
        .expect("Could not valid monad input number");

    digits_to_number(&answer.0)
        .try_into()
        .expect("Could not convert i64 to usize")
}

fn main() {
    let setup_time = std::time::Instant::now();

    let input_str =
        std::fs::read_to_string("input/day24.txt").expect("Failed to read day 24 input");
    let monad: Vec<Instruction> = parse_program(&input_str);
    println!("Setup took {:.6} µs", setup_time.elapsed().as_micros());

    // Part 1
    let part1_time = std::time::Instant::now();
    let part1_result = part1(&monad);
    println!("Part 1 took {:.6} ms", part1_time.elapsed().as_millis());

    // Part 2
    // let part2_time = std::time::Instant::now();
    // let part2_result = solve(&input_image, &algo, 50);
    // println!("Part 2 took {:.6} ms", part2_time.elapsed().as_millis());

    println!();
    println!("Part 1 result: {}", part1_result);
    // println!("Part 2 result: {}", part2_result);
}

#[test]
fn test_parse_inp() {
    let input_str = "inp x";
    let expected = Instruction::Inp {
        field_idx: 1,
        val: None,
    };
    let got = parse_line(input_str);
    assert_eq!(expected, got);
}

#[test]
fn test_parse_inp_2() {
    let input_str = "inp z";
    let expected = Instruction::Inp {
        field_idx: 3,
        val: None,
    };
    let got = parse_line(input_str);
    assert_eq!(expected, got);
}

#[test]
fn test_parse_add() {
    let input_str = "add x 34";
    let expected = Instruction::Add {
        field_idx: 1,
        scnd_arg: IdxOrVal::Val(34),
    };
    let got = parse_line(input_str);
    assert_eq!(expected, got);
}

#[test]
fn test_parse_mod() {
    let input_str = "mod z w";
    let expected = Instruction::Mod {
        field_idx: 3,
        scnd_arg: IdxOrVal::Idx(0),
    };
    let got = parse_line(input_str);
    assert_eq!(expected, got);
}

#[test]
fn test_read_and_negate_1() {
    let program_str = "inp x
mul x -1";
    let program = parse_program(program_str);
    let args = VecDeque::from([3]);
    let expected = State(0, -3, 0, 0);

    let got = run_program(&program, &args);
    assert_eq!(expected, got);
}

#[test]
fn test_read_and_negate_2() {
    let program_str = "inp z
mul z -1";
    let program = parse_program(program_str);
    let args = VecDeque::from([-10]);
    let expected = State(0, 0, 0, 10);

    let got = run_program(&program, &args);
    assert_eq!(expected, got);
}

#[test]
fn test_is_3_times_larger_1() {
    let program_str = "inp z
inp x
mul z 3
eql z x";
    let program = parse_program(program_str);
    let args = VecDeque::from([-10, -30]);
    let expected = State(0, -30, 0, 1);

    let got = run_program(&program, &args);
    assert_eq!(expected, got);
}

#[test]
fn test_is_3_times_larger_2() {
    let program_str = "inp z
inp x
mul z 3
eql z x";
    let program = parse_program(program_str);
    let args = VecDeque::from([12, 36]);
    let expected = State(0, 36, 0, 1);

    let got = run_program(&program, &args);
    assert_eq!(expected, got);
}

#[test]
fn test_is_3_times_larger_3() {
    let program_str = "inp z
inp x
mul z 3
eql z x";
    let program = parse_program(program_str);
    let args = VecDeque::from([12, 12]);
    let expected = State(0, 12, 0, 0);

    let got = run_program(&program, &args);
    assert_eq!(expected, got);
}

#[test]
fn test_binary_converter_1() {
    let program_str = "inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2";
    let program = parse_program(program_str);
    let args = VecDeque::from([2]);
    let expected = State(0, 0, 1, 0);

    let got = run_program(&program, &args);
    assert_eq!(expected, got);
}

#[test]
fn test_binary_converter_2() {
    let program_str = "inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2";
    let program = parse_program(program_str);
    let args = VecDeque::from([13]);
    let expected = State(1, 1, 0, 1);

    let got = run_program(&program, &args);
    assert_eq!(expected, got);
}
