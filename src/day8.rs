use std::collections::HashSet;
use std::collections::HashMap;  

#[allow(dead_code)]
pub fn handheld_halting(instructions : &[String]) -> i32 {
    let mut acc = 0;
    let mut lines = HashSet::<i32>::new();
    let mut instructions_map = HashMap::<i32, (String, i32)>::new();
    let mut curr_line = 0;

    for instruction in instructions {
        let operator = get_operator(instruction);
        let argument = get_argument(instruction);
        instructions_map.insert(curr_line, (operator,argument));
        curr_line += 1;
        //println!("{} {}", operator, argument);
    }

    curr_line = 0;
    let mut instruction = instructions_map.get(&0).unwrap();

    loop {
        let operator = instruction.0.as_str();
        let argument = instruction.1;

        if lines.contains(&curr_line) {
            return acc;
        }

        lines.insert(curr_line);

        match operator {
            "acc" => {acc += argument; curr_line +=1; },
            "jmp" => curr_line += argument,
            "nop" => curr_line += 1, 
            _ => ()
        }

        instruction = instructions_map.get(&curr_line).unwrap();
    }
}

#[allow(dead_code)]
pub fn handheld_halting2(instructions : &[String]) -> i32 {
    let mut acc = 0;
    let mut lines = HashSet::<i32>::new();
    let mut instructions_map = HashMap::<i32, (String, i32)>::new();
    let mut curr_line = 0;
    let mut could_be_problem = Vec::<i32>::new();

    for instruction in instructions {
        let operator = get_operator(instruction);
        let argument = get_argument(instruction);
        instructions_map.insert(curr_line, (operator.clone(),argument));
        if operator == "jmp".to_string() || operator == "nop".to_string() {
            could_be_problem.push(curr_line);
        }
        curr_line += 1;
        //println!("{} {}", operator, argument);
    }

    curr_line = 0;
    let mut instruction = instructions_map.get(&0).unwrap();
    let mut problem_line = -1;

    loop {

        if lines.contains(&curr_line) {
            problem_line = could_be_problem.pop().unwrap();
            instruction = instructions_map.get(&0).unwrap();
            curr_line = 0;
            lines.clear();
            acc = 0;

        }

        let mut operator = instruction.0.as_str();
        let argument = instruction.1;

        lines.insert(curr_line);

        if curr_line == problem_line {
            
            match operator {
                "jmp" => operator = "nop",
                "nop" => operator = "jmp", 
                _ => ()
            } 
        }

        match operator {
            "acc" => {acc += argument; curr_line +=1; },
            "jmp" => curr_line += argument,
            "nop" => curr_line += 1, 
            _ => ()
        }

        if curr_line as usize  == instructions_map.len() {
            return acc;
        }

        instruction = instructions_map.get(&curr_line).unwrap();
    }
}


fn get_operator(instruction : &String) -> String {
    let operator = instruction.split_whitespace().collect::<Vec<_>>();
    operator[0].to_string()
}

fn get_argument(instruction : &String) -> i32 {
    let argument = instruction.split_whitespace().collect::<Vec<_>>()[1];
    let sign = &argument[0..1];
    let number = &argument[1..];
    match sign {
        "+" => number.parse::<i32>().unwrap(),
        "-" => -1 * number.parse::<i32>().unwrap(),
        _ => -1
    }
}


#[test]
fn test_handheld_halting() {
    assert_eq!(handheld_halting(&["nop +0".to_string(),
                                "acc +1".to_string(),
                                "jmp +4".to_string(),
                                "acc +3".to_string(),
                                "jmp -3".to_string(),
                                "acc -99".to_string(),
                                "acc +1".to_string(),
                                "jmp -4".to_string(),
                                "acc +6".to_string(),
                               ]), 5);
}

#[test]
fn test_handheld_halting2() {
    assert_eq!(handheld_halting2(&["nop +0".to_string(),
                                "acc +1".to_string(),
                                "jmp +4".to_string(),
                                "acc +3".to_string(),
                                "jmp -3".to_string(),
                                "acc -99".to_string(),
                                "acc +1".to_string(),
                                "jmp -4".to_string(),
                                "acc +6".to_string(),
                               ]), 8);
}