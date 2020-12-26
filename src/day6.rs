use std::collections::HashSet;  

#[allow(dead_code)]
pub fn custom_customs(groups: &[String]) -> usize {
    let mut group = "".to_string();
    let mut group2 = Vec::<&String>::new();
    let mut count = 0;
    let mut count2 = 0;
    for line in groups {
        if *line == "".to_string() {
            count += answer_count(&group);
            group = "".to_string();

            count2 += answer_count2(group2);
            group2 = Vec::<&String>::new();
        } else {
            group.push_str(line);
            group2.push(line);
        }
    }

    count2 += answer_count2(group2);
    println!("{}",count2);


    count += answer_count(&group);
    count
}

fn answer_count2(group: Vec<&String>) -> usize {
    let mut answers = HashSet::<char>::new();
    for ch in group[0].chars() {
        answers.insert(ch);
    }

    let mut p_answers = HashSet::<char>::new();
    for person in group {
        for ch in person.chars() {
            p_answers.insert(ch);
        }
        answers = answers.intersection(&p_answers).map(|x| *x).collect::<HashSet<_>>();
        p_answers.clear();
    }
    answers.len()
}

fn answer_count(group: &String) -> usize {
    let mut answers = HashSet::<char>::new();
    for ch in group.chars() {
        answers.insert(ch);
    }
    answers.len()
}



#[test]
fn test_custom_customs() {
    assert_eq!(answer_count(&"abc".to_string()), 3);
    assert_eq!(answer_count(&"abac".to_string()), 3);
    assert_eq!(answer_count(&"aaaa".to_string()), 1);
    assert_eq!(answer_count(&"b".to_string()), 1);
}


#[test]
fn test_answer_count() {
    assert_eq!(custom_customs(&["abcx".to_string(),
                                "abcy".to_string(),
                                "abcz".to_string(),
                               ]), 6);
}
