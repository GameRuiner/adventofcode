use std::collections::HashSet;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn ticket_translation(notes : &[String]) -> i32 {
    let mut i = 0;
    let mut num_rules = vec![];
    while i < notes.len() {
        if notes[i] == "" {
            break;
        }
        num_rules.push(&notes[i]);
        i+=1;
    }

    let my_ticket =  notes[i+2].split(",").map(|x| x.parse::<i32>().unwrap())
                                                    .collect::<Vec<i32>>();
    i+=5;
    let mut tickets = vec![];
    while i < notes.len() {
        tickets.push(&notes[i]);
        i+=1;
    }
    
    let valid_num = get_valid(&num_rules);

    let mut sum = 0; 
    let mut valid_tickets = vec![];
    for ticket in tickets {
        let ticket_split = ticket.split(",").map(|x| x.parse::<i32>().unwrap())
                                                    .collect::<Vec<i32>>();
        let mut valid = true;
        for n in &ticket_split {
            if !valid_num.contains(n) {
                valid = false;
                sum += *n;
                break;
            }
        }
        if valid { 
            valid_tickets.push(ticket_split);
        }
    }

    //let num_slice = num_rules[0..6].into_iter().map(|x| *x).collect::<Vec<&String>>();
    let valid_num = get_valid2(&num_rules);

    //println!("{} {} ", valid_num.len(), valid_tickets.len());
    //println!("{:?}", valid_num);
    let mut candidats = HashMap::<usize,Vec<i32>>::new();

    for i in 0..my_ticket.len() {
        let mut num_num_set = 0;
        for num_set in &valid_num {
            let mut valid = true;
            for ticket in &valid_tickets {
                if !num_set.contains(&ticket[i]) {
                    valid = false;
                    break;
                } 
            }
            if valid {
                //println!("{}", my_ticket[i]);
                //mul *= my_ticket[i] as u64;
                if candidats.contains_key(&i) {
                    let mut candidat_vec = candidats.get(&i).unwrap().clone();
                    candidat_vec.push(num_num_set);
                    candidats.insert(i, candidat_vec);
                } else {
                    candidats.insert(i, vec![num_num_set]);
                }
            }
            num_num_set += 1;
        }
    }
    println!("{:?}", candidats);

    let mut mul: i64 = 1;
    let mut nums = 0;
    loop {
       let mut new_candidats =  HashMap::<usize,Vec<i32>>::new();
       for (k,v) in &candidats {
          if v.len() == 1 {
               new_candidats = del_num(&candidats, v[0]);
               if v[0] < 6 {
                   mul *= my_ticket[*k as usize] as i64;
                   //println!("{:?} {} {}", my_ticket[v[0] as usize], *k, v[0]);
                   nums+=1;
               }
               break;
           }
       }
       candidats = new_candidats;
       println!("{:?}", candidats);
       if nums == 6 {
           break;
       }
    }
    println!("{}", mul);

    sum
}

fn del_num(old_candidats: &HashMap<usize,Vec<i32>>, num_del: i32) ->  HashMap<usize,Vec<i32>> {
    let mut candidats = HashMap::<usize,Vec<i32>>::new();
    for (k,v) in old_candidats {
        if v.contains(&num_del) {
            let mut new_v = v.clone();
            let i_del = v.into_iter().position(|x| *x == num_del).unwrap();
            new_v.remove(i_del);
            candidats.insert(*k, new_v);
        } else {
            candidats.insert(*k, v.clone());
        }
    }
    candidats
}

fn get_valid2(rules: &Vec<&String>) -> Vec<HashSet<i32>>{
    let mut nums = Vec::<HashSet<i32>>::new();
    for line in rules {
        let mut new_set = HashSet::<i32>::new();
        let line_split = line.split(":").collect::<Vec<&str>>()[1].trim();
        let line_split2 = line_split.split_ascii_whitespace().collect::<Vec<&str>>();
        let p1 = line_split2[0];
        let p2 = line_split2[2];
        let (s,e) = get_nums(p1);
        for i in s..e+1 {
            new_set.insert(i);
        }
        let (s,e) = get_nums(p2);
        for i in s..e+1 {
            new_set.insert(i);
        }
        //println!("{}", line);
        nums.push(new_set);

    }
    nums

}

fn get_valid(rules: &Vec<&String>) -> HashSet<i32>{
    let mut nums = HashSet::<i32>::new();
    for line in rules {
        let line_split = line.split(":").collect::<Vec<&str>>()[1].trim();
        let line_split2 = line_split.split_ascii_whitespace().collect::<Vec<&str>>();
        let p1 = line_split2[0];
        let p2 = line_split2[2];
        let (s,e) = get_nums(p1);
        for i in s..e+1 {
            nums.insert(i);
        }
        let (s,e) = get_nums(p2);
        for i in s..e+1 {
            nums.insert(i);
        }
    }
    nums

}


fn get_nums(str_pair: &str) -> (i32,i32) {
    let pair_split = str_pair.split("-").collect::<Vec<&str>>();
    (pair_split[0].parse().unwrap(),pair_split[1].parse().unwrap())
}

#[test]
fn test_ticket_translation() {
    assert_eq!(ticket_translation(&["class: 1-3 or 5-7".to_string(),
                               "row: 6-11 or 33-44".to_string(),
                               "seat3: 13-40 or 45-50".to_string(),
                               "clas5s: 1-3 or 5-7".to_string(),
                               "row2: 6-11 or 33-44".to_string(),
                               "sea3t: 13-40 or 45-50".to_string(),
                               "cla4ss: 1-3 or 5-7".to_string(),
                               "row2: 6-11 or 33-44".to_string(),
                               "se5at: 13-40 or 45-50".to_string(),
                               "".to_string(),
                               "your ticket:".to_string(),
                               "7,1,14".to_string(),
                               "".to_string(),
                               "nearby tickets:".to_string(),
                               "7,3,47".to_string(),
                               "40,4,50".to_string(),
                               "55,2,20".to_string(),
                               "38,6,12".to_string(),
                               ]), 71);
}
