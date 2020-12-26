use std::collections::HashMap;

#[allow(dead_code)]
pub fn docking_data(program : &[String]) -> i64 {
    let mut mask = "";
    let mut mem_map = HashMap::<i64,i64>::new();
    for line in program {
        let line = line.split(" ").collect::<Vec<_>>();
        let command = line[0];
        let argument = line[2];
        if command == "mask" {
            mask = argument;
        } else {
            let mem_cell = command.get(4..command.len()-1).unwrap().parse::<i64>().unwrap();
            let cell_val = get_val(mask, argument);
            if mem_map.contains_key(&mem_cell) {
                mem_map.remove(&mem_cell);
            }
            mem_map.insert(mem_cell, cell_val);
        }
    }
    mem_map.values().sum()
}

fn get_val(mask: &str, value: &str) -> i64{
    let bin_value = format!("{:b}",value.parse::<i64>().unwrap());
    let mut mask_value = "".to_string();

    for i in 0..mask.len() {
        let ag_i = i as i32 - (mask.len() - bin_value.len()) as i32;
        match mask.get(i..i+1).unwrap() {
            "X" => {
                if ag_i < 0 {
                    mask_value.push('0');    
                } else {
                    mask_value.push_str(bin_value.get(ag_i as usize..ag_i as usize +1).unwrap());
                }   
            },
            "1" => mask_value.push('1'),
            "0" => mask_value.push('0'),
            _   => (),
        }
    }
    
    let intval = isize::from_str_radix(&mask_value, 2).unwrap();
    intval as i64
}

#[allow(dead_code)]
pub fn docking_data2(program : &[String]) -> i64 {
    let mut mask = "";
    let mut mem_map = HashMap::<i64,i64>::new();
    for line in program {
        let line = line.split(" ").collect::<Vec<_>>();
        let command = line[0];
        let argument = line[2];
        if command == "mask" {
            mask = argument;
        } else {
            let mem_cells = get_val2(mask, command.get(4..command.len()-1).unwrap().parse::<i64>().unwrap());
            let cell_val = argument.parse::<i64>().unwrap();
            for cell in mem_cells {
                if mem_map.contains_key(&cell) {
                mem_map.remove(&cell);
                }
                mem_map.insert(cell, cell_val);
            }

        }
    }
    mem_map.values().sum()
}

fn get_val2(mask: &str, value: i64) -> Vec<i64>{
    let bin_value = format!("{:b}",value);
    let mut mask_value = "".to_string();
    let mut res = vec![];

    for i in 0..mask.len() {
        let ag_i = i as i32 - (mask.len() - bin_value.len()) as i32;
        match mask.get(i..i+1).unwrap() {
            "0" => {
                if ag_i < 0 {
                    mask_value.push('0');    
                } else {
                    mask_value.push_str(bin_value.get(ag_i as usize..ag_i as usize +1).unwrap());
                }   
            },
            "1" => mask_value.push('1'),
            "X" => mask_value.push('X'),
            _   => (),
        }
    }
    let mut bin_res = vec![];
    bin_res.push(mask_value);
    loop {
        let val_with_x = bin_res.first().unwrap();
        let mut end = true;
        for (i,ch) in val_with_x.chars().enumerate() {
            if ch == 'X' {
                let val1 = change_x(&val_with_x, i, '1');
                let val0 = change_x(&val_with_x, i, '0');
                bin_res.push(val1);
                bin_res.push(val0);
                end = false;
                break;
            }
        }
        if end {
            break;
        } else {
            bin_res.remove(0);
        }
    }
    
    for b_v in bin_res {
        let intval = isize::from_str_radix(&b_v, 2).unwrap() as i64;
        res.push(intval);
    }
    res
}

fn change_x(val_with_x: &String, i: usize, n: char) -> String{
    let mut x = val_with_x.get(0..i).unwrap().to_string();
    x.push(n);
    x.push_str(val_with_x.get(i+1..val_with_x.len()).unwrap());
    x
}

#[test]
fn test_docking_data() {
    assert_eq!(docking_data(&[ "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
                               "mem[8] = 11".to_string(),
                               "mem[7] = 101".to_string(),
                               "mem[8] = 0".to_string(),
                               ]), 165);
}

#[test]
fn test_docking_data2() {
    assert_eq!(docking_data2(&["mask = 000000000000000000000000000000X1001X".to_string(),
                               "mem[42] = 100".to_string(),
                               "mask = 00000000000000000000000000000000X0XX".to_string(),
                               "mem[26] = 1".to_string(),
                               ]), 208);
}
