#[allow(dead_code)]
pub fn shuttles_search(notes : &[String]) -> i32 {
    let timestamp = notes[0].parse::<i32>().unwrap(); 
    let buses = get_ids(&notes[1]);
    //println!("{} {:?}", timestamp, buses);
    let mut time_to_wait = timestamp;
    let mut bus_id = 0;
    for id in buses {
        let rem = timestamp % id;
        if rem == 0 {
            return 0;
        } else {
            let wait = (timestamp/id + 1)*id - timestamp;
            if wait < time_to_wait {
                time_to_wait = wait;
                bus_id = id;
            } 
        }
    }
    bus_id * time_to_wait
}

fn get_ids(ids: &String) -> Vec<i32> {
     ids.split(',').filter(|x| *x != "x").map(|x| x.parse::<i32>().unwrap()).collect()
}

fn get_ids2(ids: &String) -> Vec<(usize, usize)> {
    ids.split(',').enumerate().filter(|(_i,x)| *x != "x").map(|(i,x)| (i,x.parse::<usize>().unwrap())).map(|(i,x)| (x,(x-(i % x)) % x)).collect()
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}
 
fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}
 
fn chinese_remainder(residues: &Vec<usize>, modulii: &Vec<usize>) -> Option<usize> {
    let prod = modulii.iter().product::<usize>();
 
    let mut sum = 0;
 
    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p as i64, modulus as i64)? as usize * p
    }
 
    Some(sum % prod)
}
 
#[allow(dead_code)]
pub fn shuttles_search3(notes : &[String]) -> usize {
    //let modulii = [3,5,7];
    //let residues = [2,3,2];
    let buses = get_ids2(&notes[1]);
    let mut modulii = vec![];
    let mut residues = vec![];

    for ( n, r) in buses {
        modulii.push(n);
        residues.push(r);
    } 
    match chinese_remainder(&residues, &modulii) {
        Some(sol) => sol,
        None      => 0
    }
 
}


#[test]
fn test_rain_risk() {
    assert_eq!(shuttles_search(&[ "939".to_string(),
                            "7,13,x,x,59,x,31,19".to_string(),
                               ]), 295);
}

#[test]
fn test_rain_risk2() {
    assert_eq!(shuttles_search3(&[ "939".to_string(),
                            "7,13,x,x,59,x,31,19".to_string(),
                               ]), 1068781);
}

#[test]
fn test2_rain_risk2() {
    assert_eq!(shuttles_search3(&[ "939".to_string(),
                            "17,x,13,19".to_string(),
                               ]), 3417);
}

#[test]
fn test3_rain_risk2() {
    assert_eq!(shuttles_search3(&[ "939".to_string(),
                            "67,7,59,61".to_string(),
                               ]), 754018);
}

#[test]
fn test4_rain_risk2() {
    assert_eq!(shuttles_search3(&[ "939".to_string(),
                            "67,x,7,59,61".to_string(),
                               ]), 779210);
}

#[test]
fn test5_rain_risk2() {
    assert_eq!(shuttles_search3(&[ "939".to_string(),
                            "1789,37,47,1889".to_string(),
                               ]), 1202161486);
}

#[test]
fn test6_rain_risk2() {
    assert_eq!(shuttles_search3(&[ "939".to_string(),
                            "67,7,x,59,61".to_string(),
                               ]), 1261476);
}
