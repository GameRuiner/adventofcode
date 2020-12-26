use std::collections::HashSet;
use std::collections::HashMap;  

#[allow(dead_code)]
pub fn handy_haversacks(rules: &[String]) -> usize {
    let mut colors = HashSet::<String>::new();
    let mut find_colors = Vec::<String>::new();

    find_colors.push("shiny gold".to_string());

    let mut temp_candidats = find_color_fn(&find_colors, rules, &mut colors);
    while temp_candidats.len() != 0 {
        temp_candidats = find_color_fn(&temp_candidats, rules, &mut colors);
    }

    colors.len()
}

#[allow(dead_code)]
pub fn handy_haversacks2(rules: &[String]) -> usize {
    let rules_map = string_to_map(rules);
    count_rule(&"shiny gold".to_string(), &rules_map) - 1
}

fn count_rule(main_color: &String, rules: &HashMap::<String, Vec<(usize, String)>>) -> usize {
    let mut res = 1;
    let rule_opt = rules.get(main_color);
    if rule_opt.is_none() {
        return 1;
    }
    let rule = rule_opt.unwrap();
    for (amount, color) in rule {
        res += *amount * count_rule(color, rules);
    } 
    res
}

fn string_to_map(rules: &[String]) ->  HashMap::<String, Vec<(usize, String)>>{
    let mut rules_map = HashMap::<String, Vec<(usize, String)>>::new();
    for rule in rules {
        let rule_split = rule.as_str().split_whitespace().collect::<Vec<&str>>();
        let bag_color = extract_color(&rule_split,0);

        let mut tuple_vec = Vec::<(usize, String)>::new();

        let rule_contain = rule_split[4..].into_iter().map(|x| *x).collect::<Vec<&str>>();
        if rule_contain[0] == "no" {
            tuple_vec.push((0,"no".to_string()));
            continue;
        }

        let contain_join = rule_contain.join(" ");
        let contain_split = contain_join.as_str().split(",").map(|x| x.trim()).collect::<Vec<&str>>();

        for contains in contain_split {
            let curr_contain_split = contains.split_whitespace().collect::<Vec<&str>>();
            let color = extract_color(&curr_contain_split,1);
            let amount = curr_contain_split[0].parse::<usize>().unwrap();
            tuple_vec.push((amount, color));
        }

        rules_map.insert(bag_color, tuple_vec);
    }
    rules_map
}


fn find_color_fn(find_colors:  &Vec<String>, rules: &[String], colors:  &mut HashSet<String>) -> Vec<String> {
    let mut temp_candidats = Vec::<String>::new();
    
    for candidat in find_colors {
    
        for rule in rules {
        let rule_split = rule.as_str().split_whitespace().collect::<Vec<&str>>();

        let bag_color = extract_color(&rule_split,0);
        
        let rule_contain = rule_split[4..].into_iter().map(|x| *x).collect::<Vec<&str>>();
        if rule_contain[0] == "no" {
            continue;
        }

        let contain_join = rule_contain.join(" ");
        let contain_split = contain_join.as_str().split(",").map(|x| x.trim()).collect::<Vec<&str>>();

        for contains in contain_split {
            let curr_contain_split = contains.split_whitespace().collect::<Vec<&str>>();
            let color = extract_color(&curr_contain_split,1);
            if color == *candidat {
                if !colors.contains(&bag_color) {
                    colors.insert(bag_color.clone());
                    temp_candidats.push(bag_color.clone());
                }
            }

        }
        
        }
    }
    
    temp_candidats
}

fn extract_color(vec_str: &Vec<&str>, start: usize) -> String {
    let mut bag_color = vec_str[start].to_string();
    bag_color.push(' ');
    bag_color.push_str(vec_str[start+1]);
    bag_color
}


#[test]
fn test_handy_haversacks() {
    assert_eq!(handy_haversacks(&["light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
                                "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
                                "bright white bags contain 1 shiny gold bag.".to_string(),
                                "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
                                "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
                                "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
                                "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
                                "faded blue bags contain no other bags.".to_string(),
                                "dotted black bags contain no other bags.".to_string(),
                               ]), 4);
}

#[test]
fn test_handy_haversacks2() {
    assert_eq!(handy_haversacks2(&["shiny gold bags contain 2 dark red bags.".to_string(),
                                "dark red bags contain 2 dark orange bags.".to_string(),
                                "dark orange bags contain 2 dark yellow bags.".to_string(),
                                "dark yellow bags contain 2 dark green bags.".to_string(),
                                "dark green bags contain 2 dark blue bags.".to_string(),
                                "dark blue bags contain 2 dark violet bags.".to_string(),
                                "dark violet bags contain no other bags.".to_string(),
                               ]), 126);
}

#[test]
fn test2_handy_haversacks2() {
    assert_eq!(handy_haversacks2(&["shiny gold bags contain 2 dark red bags.".to_string(),
                                "dark red bags contain 2 dark orange bags.".to_string(),
                                "dark orange bags contain no other bags.".to_string(),
                               ]), 6);
}