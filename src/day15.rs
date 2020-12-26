use std::collections::HashMap;
#[allow(dead_code)]
pub fn rambunctious_recitation(numbers : &[i32], last_turn: i32) -> i32 {
    let mut numbers_map = HashMap::<i32,i32>::new();
    let mut turn = 1;
    let mut last = 0;
    for num in numbers {
        numbers_map.insert(*num, turn);
        turn +=1;
        last = *num;
    }

    let mut new_n = false;
    while turn <= last_turn {
        if numbers_map.contains_key(&last) {
            let last_spoken = *numbers_map.get(&last).unwrap();
            if new_n {
                new_n = false;
                last = 0;
                numbers_map.insert(last, turn-1);
            } else {
                numbers_map.insert(last, turn-1);
                last = (turn-1)-last_spoken;
            }         
        } else {
            numbers_map.insert(last, turn-1);
            last = 0;
        }
        //println!("{:?} turn {} last {} ", numbers_map, turn, last, );
        turn+=1;
    }

    last
}


#[test]
fn test_rambunctious_recitation() {
    assert_eq!(rambunctious_recitation(&[0,3,6], 9), 4);
}

#[test]
fn test2_rambunctious_recitation() {
    assert_eq!(rambunctious_recitation(&[0,3,6], 10), 0);
}

#[test]
fn test3_rambunctious_recitation() {
    assert_eq!(rambunctious_recitation(&[1,3,2], 2020), 1);
}

#[test]
fn test4_rambunctious_recitation() {
    assert_eq!(rambunctious_recitation(&[2,1,3], 2020), 10);
}

#[test]
fn test5_rambunctious_recitation() {
    assert_eq!(rambunctious_recitation(&[0,3,6], 30000000), 175594);
}