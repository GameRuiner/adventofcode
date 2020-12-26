#[allow(dead_code)]
pub fn seating_system(seats : &[String]) -> i32 {
    let mut vec_seats = Vec::<Vec<char>>::new();
    for line in seats {
        vec_seats.push(line.chars().collect());
    }
    //let mut new_seats = round(&vec_seats);
    let mut new_seats = round2(&vec_seats);
    while new_seats != vec_seats {
        vec_seats = new_seats;
        new_seats = round2(&vec_seats);       
    }

    let mut occupied = 0;
    for line in new_seats {
        for seat in line {
            if seat == '#' {
                occupied+=1;
            }
        }
    }

    occupied
}

#[allow(dead_code)]
fn round(seats : &Vec::<Vec<char>>) ->  Vec::<Vec<char>> {
    let mut new_seats = Vec::<Vec<char>>::new();
    for line_num in 0..seats.len() {
        let line = seats[line_num].clone();
        let mut new_line = Vec::<char>::new();
        for seat_num in 0..line.len() {
            let seat = line[seat_num];
            match seat {
                'L' => {
                    let occupied = count_seats(line_num, seat_num, &seats, '#');
                    if occupied == 0 {
                        new_line.push('#');
                    } else {
                        new_line.push('L');
                    }
                },
                '#' => {
                    let occupied = count_seats(line_num, seat_num, &seats, '#');
                    if occupied > 3 {
                        new_line.push('L');
                    } else {
                        new_line.push('#');
                    }
                },
                _ => new_line.push(seat),
            }

        }
        new_seats.push(new_line);
    }
    new_seats
}

fn round2(seats : &Vec::<Vec<char>>) ->  Vec::<Vec<char>> {
    let mut new_seats = Vec::<Vec<char>>::new();
    for line_num in 0..seats.len() {
        let line = seats[line_num].clone();
        let mut new_line = Vec::<char>::new();
        for seat_num in 0..line.len() {
            let seat = line[seat_num];
            match seat {
                'L' => {
                    let occupied = count_seats2(line_num, seat_num, &seats);
                    if occupied == 0 {
                        new_line.push('#');
                    } else {
                        new_line.push('L');
                    }
                },
                '#' => {
                    let occupied = count_seats2(line_num, seat_num, &seats);
                    if occupied > 4 {
                        new_line.push('L');
                    } else {
                        new_line.push('#');
                    }
                },
                _ => new_line.push(seat),
            }

        }
        new_seats.push(new_line);
    }
    new_seats
}


fn count_seats(line_num: usize, seat_num: usize, seats: &Vec::<Vec<char>>, element: char) -> i32{
    let mut line = vec![];
    
    if line_num > 0 {
        line.push(line_num-1);
    }

    line.push(line_num);

    if line_num < seats.len()-1 {
        line.push(line_num+1);
    }

    let mut seat = vec![];

    let line_len = seats[0].len();

    if seat_num > 0 {
        seat.push(seat_num-1);
    }

    seat.push(seat_num);

    if seat_num < line_len-1 {
        seat.push(seat_num+1);
    }

    let mut count = 0;
    for l_num in line {
        for s_num in &seat {
            if l_num == line_num && *s_num == seat_num {
                continue;
            }
            if seats[l_num][*s_num] == element {
                count += 1;
            }
        }
    }

    count


}

fn count_seats2(line_num: usize, seat_num: usize, seats: &Vec::<Vec<char>>) -> i32{
    
    let mut count = 0;

    let mut up = line_num as i32 - 1;
    while up >= 0 {
        let status = check_seat(seats[up as usize][seat_num]);
        if status != -1 {
            count += status;
            break;
        }
        up -= 1;
    }
    
    let mut down = line_num+1;
    while down < seats.len() {
        let status = check_seat(seats[down][seat_num]);
        if status != -1 {
            count += status;
            break;
        }
        down += 1;
    }
    
    let mut left = seat_num as i32 -1;
    while left >= 0 {
        let status = check_seat(seats[line_num][left as usize]);
        if status != -1 {
            count += status;
            break;
        }
        left -= 1;
    }
    
    let mut right = seat_num+1;
    while right < seats[0].len() {
        let status = check_seat(seats[line_num][right]);
        if status != -1 {
            count += status;
            break;
        }
        right += 1;
    }


    let mut right = seat_num+1;
    let mut up = line_num as i32 -1;
    while right < seats[0].len() && up >= 0{
        let status = check_seat(seats[up as usize][right]);
        if status != -1 {
            count += status;
            break;
        }
        right += 1;
        up -=1;
    }

    let mut right = seat_num+1;
    let mut down = line_num+1;
    while right < seats[0].len() && down < seats.len() {
        let status = check_seat(seats[down][right]);
        if status != -1 {
            count += status;
            break;
        }
        right += 1;
        down +=1;
    }


    let mut left = seat_num as i32 - 1;
    let mut down = line_num+1;
    while left >= 0 && down < seats.len() {
        let status = check_seat(seats[down][left as usize]);
        if status != -1 {
            count += status;
            break;
        }
        left -= 1;
        down +=1;
    }

    let mut left = seat_num as i32-1;
    let mut up = line_num as i32 -1;
    while left >= 0 && up >= 0 {
        let status = check_seat(seats[up as usize][left as usize]);
        if status != -1 {
            count += status;
            break;
        }
        left -= 1;
        up -=1;
    }

    
    count


}

fn check_seat(ch: char) -> i32 {
    if ch == '#' {
        return 1;       
    }
    if ch == 'L' {
        return 0;
    }
    -1
}



#[test]
fn test_seating_system() {
    assert_eq!(seating_system(&[ "L.LL.LL.LL".to_string(),
                                "LLLLLLL.LL".to_string(),
                                "L.L.L..L..".to_string(),
                                "LLLL.LL.LL".to_string(),
                                "L.LL.LL.LL".to_string(),
                                "L.LLLLL.LL".to_string(),
                                "..L.L.....".to_string(),
                                "LLLLLLLLLL".to_string(),
                                "L.LLLLLL.L".to_string(),
                                "L.LLLLL.LL".to_string(),
                               ]), 26);
}
