use std::collections::HashSet;

#[allow(dead_code)]
pub fn encoding_error(numbers : &[u64], preamble: usize) -> u64 {
    let mut i = 0;
    let mut candidats = Vec::<u64>::new();
    let mut sums = Vec::<HashSet<u64>>::new();

    while i < preamble {
        candidats.push(numbers[i]);
        let mut new_sum = HashSet::<u64>::new();
        for j in 0..i {
            new_sum.insert(candidats[j] + numbers[i]);
        }
        sums.push(new_sum);
        i+=1;
    }

    while i < numbers.len() {

        let number = numbers[i];

        if !sum_contain(number, &sums) {
            find_set2(number, &numbers, preamble);
            return number;
        }

        candidats.push(number);
        

        let mut new_sum = HashSet::<u64>::new();
        for j in 0..preamble {
            new_sum.insert(candidats[j] + number);
        }

        candidats.remove(0);

        sums.push(new_sum);
        sums.remove(0);
        i+=1;
    }

    0
}

fn sum_contain(number: u64, sums: &Vec::<HashSet<u64>>) -> bool {
    for set in sums {
        if set.contains(&number) {
            return true;
        }
    }
    false
}

fn find_set2(number: u64, candidats: &[u64], _preamble: usize) {
    for i in 0..candidats.len() {
        let mut sum = candidats[i];
        let mut min_n = candidats[i];
        let mut max_n = candidats[i];
        for j in i+1..candidats.len() {
            sum += candidats[j];
            min_n = min_n.min(candidats[j]);
            max_n = max_n.max(candidats[j]);
            if sum == number {
                println!("{} ", min_n + max_n);
                return;
            }
            if sum > number {
                break;
            }
        }

    }
}

#[test]
fn test_handheld_halting2() {
    assert_eq!(encoding_error(&[30,20,15,25,47,40,62,55,65,95,102,117,150,182,127,219,299,277,309,576], 5), 127);
}