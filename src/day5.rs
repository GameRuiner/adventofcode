#[allow(dead_code)]
pub fn binary_boarding(boarding_passes: &[String]) -> i32  {
    let mut max_id = 0;
    let mut passes_id = Vec::<i32>::new();
    for pass in boarding_passes {
        let cur_id = seat_id(pass);
        if cur_id > max_id {
            max_id = cur_id;
        }
        passes_id.push(cur_id);
    }
    passes_id.sort();
    let mut last_id = passes_id[0];
    for pass_id in passes_id {
        if last_id+1 != pass_id {
            println!("{} {} answer {}", last_id, pass_id, last_id+1);
        }
        last_id = pass_id;
    }
    max_id
}

fn seat_id(boarding_pass: &String) -> i32 {
    let mut row_max = 127;
    let mut row_min = 0;
    let mut column_max = 7;
    let mut column_min = 0;    
    for ch_tuple in boarding_pass.chars().enumerate() {
        let i = ch_tuple.0;
        let ch = ch_tuple.1;
        if i < 7 {
            choose_half(ch, &mut row_max, &mut row_min);
        } else {
            choose_half(ch, &mut column_max, &mut column_min);
        }
    }
    row_min*8+column_min
}

fn choose_half(ch: char, max: &mut i32, min: &mut i32) -> (i32,i32) {
    match ch {
        'B'|'R' => *min = ((*max+*min) as f64 / 2.0).ceil() as i32,
        'F'|'L' => *max = ((*max+*min) as f64 / 2.0).floor() as i32,
        _       => ()
    }
    (*min, *max)
}


#[test]
fn test_seat_id() {
    assert_eq!(seat_id(&"BFFFBBFRRR".to_string()), 567);
    assert_eq!(seat_id(&"FFFBBBFRRR".to_string()), 119);
    assert_eq!(seat_id(&"BBFFBBFRLL".to_string()), 820);

}


#[test]
fn test_choose_half() {
    assert_eq!(choose_half('F', &mut 127, &mut 0), (0,63));
    assert_eq!(choose_half('B', &mut 63, &mut 0), (32,63));
}
