#[allow(dead_code)]
pub fn rain_risk(instructions : &[String]) -> i32 {
    let mut direction = 90;
    let mut x = 0;
    let mut y = 0;
    
    for instruction in instructions {
        let command = instruction.get(0..1).unwrap();
        let value = instruction[1..].to_owned().parse::<i32>().unwrap();
        let mut tup = (0,0);
        match command {
            "F" => tup = forward(direction, value),
            "N" => tup = (-value, 0),
            "S" => tup = (value, 0),
            "E" => tup = (0, value),
            "W" => tup = (0, -value),
            "R" => direction = (direction + value) % 360,
            "L" => direction = (direction + 360 - value) % 360,
            _ => (),
        }
        x += tup.1;
        y += tup.0;

    }
    x.abs() + y.abs()
}

fn forward(dir: i32, val: i32) -> (i32,i32) {
    match dir {
        0 => (-val,0),
        90 => (0,val),
        180 => (val,0),
        270 => (0,-val),
        _ => (0,0),
    }
}

#[allow(dead_code)]
pub fn rain_risk2(instructions : &[String]) -> i32 {
    let mut x_ship = 0;
    let mut y_ship = 0;
    let mut x_w = 10;
    let mut y_w = -1;
    
    for instruction in instructions {
        let command = instruction.get(0..1).unwrap();
        let value = instruction[1..].to_owned().parse::<i32>().unwrap();
        let mut tup_ship = (0,0);
        let mut tup_w = (0,0);
        match command {
            "F" => tup_ship = (y_w*value, x_w*value),
            "N" => tup_w = (-value, 0),
            "S" => tup_w = (value, 0),
            "E" => tup_w = (0, value),
            "W" => tup_w = (0, -value),
            "R" => { let tup_dir = change_dir(value, &(y_w, x_w));
                x_w = tup_dir.1;
                y_w = tup_dir.0;
            },
            "L" => { let tup_dir = change_dir(-value, &(y_w, x_w));
                x_w = tup_dir.1;
                y_w = tup_dir.0;
            },
            _ => (),
        }
        x_ship += tup_ship.1;
        y_ship += tup_ship.0;
        x_w += tup_w.1;
        y_w += tup_w.0;
    }
    x_ship.abs() + y_ship.abs()
}

fn change_dir(value: i32, (y,x): &(i32,i32)) -> (i32,i32) {
    //(direction + value) % 360,
    //(direction + 360 - value) % 360,
    let dir = value % 360;
    match dir {
        0  => (*y,*x),
        -90 => (-*x,*y),
        90 => (*x,-*y),
        180 | -180 => (-*y,-*x),
        270 => (-*x,*y),
        -270 => (*x,-*y),
        _ => (0,0),
    }
    
    //(0,0)
}



#[test]
fn test_rain_risk() {
    assert_eq!(rain_risk(&[ "F10".to_string(),
                            "N3".to_string(),
                            "F7".to_string(),
                            "R90".to_string(),
                            "F11".to_string(),
                               ]), 25);
}

#[test]
fn test_rain_risk2() {
    assert_eq!(rain_risk2(&[ "F10".to_string(),
                            "N3".to_string(),
                            "F7".to_string(),
                            "R90".to_string(),
                            "F11".to_string(),
                               ]), 286);
}
