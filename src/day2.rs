#[allow(dead_code)]
pub fn password_philosophy(passwords: &[String]) -> i32 {
    let mut counter = 0;

    for password in passwords {
        let psswd_split: Vec<&str> = password.split_whitespace().collect();
        let tup = get_min_max( psswd_split[0]);
        let min: i32 = tup.0;
        let max: i32 = tup.1;
        let ch = psswd_split[1].chars().next().unwrap();
        let psswd = psswd_split[2];
        let hits = psswd.matches(ch).count() as i32;
        if hits >= min && hits <= max {
            counter+=1;
        }
    }

    counter
}

#[allow(dead_code)]
pub fn password_philosophy2(passwords: &[String]) -> i32 {
    let mut counter = 0;

    for password in passwords {
        let psswd_split: Vec<&str> = password.split_whitespace().collect();
        let tup = get_min_max( psswd_split[0]);
        let min: i32 = tup.0 - 1;
        let max: i32 = tup.1 - 1;
        let ch = psswd_split[1].chars().next().unwrap();
        let psswd = psswd_split[2].chars().collect::<Vec<char>>();
        if (psswd[min as usize] == ch) ^ (psswd[max as usize] == ch) {
            counter+=1;
        }
    }
    counter
}

fn get_min_max(min_max: &str) -> (i32,i32){
    let tup = min_max.split('-')
                       .map(|x | x.parse::<i32>().unwrap())
                       .collect::<Vec<i32>>();
    (tup[0], tup[1])
}

#[test]
fn test1() {
    assert_eq!(password_philosophy(&["1-3 a: abcde".to_string(), 
                                     "1-3 b: cdefg".to_string(), 
                                     "2-9 c: ccccccccc".to_string()]), 2);
}


#[test]
fn test2() {
    assert_eq!(password_philosophy2(&["1-3 a: abcde".to_string(), 
                                     "1-3 b: cdefg".to_string(), 
                                     "2-9 c: ccccccccc".to_string()]), 1);
}