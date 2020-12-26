use std::collections::HashSet;  
use regex::Regex;

#[allow(dead_code)]
pub fn passport_processing (batch_files: &[String]) -> u64 {
    let mut password = "".to_string();
    let mut valid = 0;
    for line in batch_files {
        if *line == "".to_string() {
            valid += password_validate(&password);
            password = "".to_string();
        } else {
            password.push(' ');
            password.push_str(&line);
        }
    }
    valid += password_validate(&password);
    valid
}

fn password_validate(password: &String) -> u64 {
    let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut contain_set = HashSet::<String>::new();
    
    let passwd_split = password.split_whitespace().collect::<Vec<_>>();
    for kv in passwd_split {
        let mut pair = kv.split(":");
        let key = pair.nth(0).unwrap();
        if required.contains(&key) {
            contain_set.insert(key.to_string());
        }
    }
    if contain_set.len() == required.len() {
        1
    } else {
        0
    }
}

#[allow(dead_code)]
pub fn passport_processing2 (batch_files: &[String]) -> u64 { 
    let mut password = "".to_string();
    let mut valid = 0;
    for line in batch_files {
        if *line == "".to_string() {
            valid += password_validate2(&password);
            password = "".to_string();
        } else {
            password.push(' ');
            password.push_str(&line);
        }
    }
    valid += password_validate2(&password);
    valid  
}

fn password_validate2(password: &String) -> u64 {
    let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut contain_set = HashSet::<String>::new();
    
    let passwd_split = password.split_whitespace().collect::<Vec<_>>();
    for kv in passwd_split {
        let pair = kv.split(":").collect::<Vec<_>>();
        let key = pair[0];
        let value = pair[1];
        if required.contains(&key) {
            let is_valid =validate_k_v(key, value);
            if is_valid == false {
                return 0;
            }
            contain_set.insert(key.to_string());
        }
    }
    if contain_set.len() == required.len() {
        1
    } else {
        0
    }
}


fn validate_k_v(key: &str, value: &str) -> bool {
    match key {
        "byr" => num_validate(value, 1920, 2002),
        "iyr" => num_validate(value, 2010, 2020),
        "eyr" => num_validate(value, 2020, 2030),
        "hgt" => hgt_validate(value),
        "hcl" => hcl_validate(value),
        "ecl" => ecl_validate(value),
        "pid" => pid_validate(value),
        _ => false,
    }
}

fn num_validate(value: &str, min: i32, max: i32) -> bool {
    let opt = value.parse::<i32>();
    if opt.is_err() {
        false
    } else {
        let num = opt.unwrap();
        if num>= min && num <= max {
            true
        } else {
            false
        }
        
    }
}

fn hgt_validate(value: &str) -> bool {
    let sliced_opt = value.get(value.len()-2..value.len());
    if sliced_opt.is_none() {
        false
    } else {
        let sliced = sliced_opt.unwrap();
        let num_slice = value.get(0..value.len()-2).unwrap();
        match sliced {
            "in" => num_validate(num_slice, 59, 76),
            "cm" => num_validate(num_slice, 150, 193),
            _ => false
        }
    }
}

fn hcl_validate(value: &str) -> bool {
    let re = Regex::new(r"^#[0-9a-fA-F]{6}$").unwrap();
    re.is_match(value)
}

fn ecl_validate(value: &str) -> bool {
    let re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    re.is_match(value)
}

fn pid_validate(value: &str) -> bool {
    let re = Regex::new(r"^[0-9]{9}$").unwrap();
    re.is_match(value)
}

#[test]
fn test1() {
    assert_eq!(passport_processing(&["ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".to_string(), 
                                     "byr:1937 iyr:2017 cid:147 hgt:183cm".to_string(), 
                                     "".to_string(),
                                     "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884".to_string(),
                                     "hcl:#cfa07d byr:1929".to_string(),
                                     "".to_string(),
                                     "hcl:#ae17e1 iyr:2013".to_string(),
                                     "eyr:2024".to_string(),
                                     "ecl:brn pid:760753108 byr:1931".to_string(),
                                     "hgt:179cm".to_string(),
                                     "".to_string(),
                                     "hcl:#cfa07d eyr:2025 pid:166559648".to_string(),
                                     "iyr:2011 ecl:brn hgt:59in".to_string(),
                                     ]), 2);
}

#[test]
fn test2() {
    assert_eq!(passport_processing2(&["ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".to_string(), 
                                     "byr:1937 iyr:2017 cid:147 hgt:183cm".to_string(), 
                                     "".to_string(),
                                     "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884".to_string(),
                                     "hcl:#cfa07d byr:1929".to_string(),
                                     "".to_string(),
                                     "hcl:#ae17e1 iyr:2013".to_string(),
                                     "eyr:2024".to_string(),
                                     "ecl:brn pid:760753108 byr:1931".to_string(),
                                     "hgt:179cm".to_string(),
                                     "".to_string(),
                                     "hcl:#cfa07d eyr:2025 pid:166559648".to_string(),
                                     "iyr:2011 ecl:brn hgt:59in".to_string(),
                                     ]), 2);
}

#[test]
fn test_hgt() {
    assert_eq!(hgt_validate("60in"), true);
    assert_eq!(hgt_validate("74in"), true);
    assert_eq!(hgt_validate("190cm"), true);
    assert_eq!(hgt_validate("190in"), false);
    assert_eq!(hgt_validate("190"), false);
}

#[test]
fn test_byr() {
    assert_eq!(num_validate("2002",1920,2002), true);
    assert_eq!(num_validate("2003",1920,2002), false);

}

#[test]
fn test_hcl() {
    assert_eq!(hcl_validate("#123abc"), true);
    assert_eq!(hcl_validate("#623a2f"), true);
    assert_eq!(hcl_validate("#123abz"), false);
    assert_eq!(hcl_validate("123abc"), false);
}

#[test]
fn test_ecl() {
    assert_eq!(ecl_validate("brn"), true);
    assert_eq!(ecl_validate("grn"), true);
    assert_eq!(ecl_validate("wat"), false);
}

#[test]
fn test_pid() {
    assert_eq!(pid_validate("000000001"), true);
    assert_eq!(pid_validate("087499704"), true);
    assert_eq!(pid_validate("0123456789"), false);
    assert_eq!(pid_validate("186cm"), false);  
}

#[test]
fn test3() {
    assert_eq!(passport_processing2(&[
                                    "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980".to_string(),
                                    "hcl:#623a2f".to_string(),
                                    "".to_string(),
                                    "eyr:2029 ecl:blu cid:129 byr:1989".to_string(),
                                    "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm".to_string(),
                                    "".to_string(),
                                    "hcl:#888785".to_string(),
                                    "hgt:164cm byr:2001 iyr:2015 cid:88".to_string(),
                                    "pid:545766238 ecl:hzl".to_string(),
                                    "eyr:2022".to_string(),
                                    "".to_string(),
                                    "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719".to_string(),]), 4);
}


#[test]
fn test_invalid() {
    assert_eq!(passport_processing2(&[
                    "eyr:1972 cid:100".to_string(),
                    "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926".to_string(),
                    "".to_string(),
                    "iyr:2019".to_string(),
                    "hcl:#602927 eyr:1967 hgt:170cm".to_string(),
                    "ecl:grn pid:012533040 byr:1946".to_string(),
                    "".to_string(),
                    "hcl:dab227 iyr:2012".to_string(),
                    "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277".to_string(),
                    "".to_string(),
                    "hgt:59cm ecl:zzz".to_string(),
                    "eyr:2038 hcl:74454a iyr:2023".to_string(),
                    "pid:3556412378 byr:2007".to_string(),]), 0);
                }


