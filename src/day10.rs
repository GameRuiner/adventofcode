use std::collections::HashSet;

#[allow(dead_code)]
pub fn adapter_array(numbers : &[i32]) -> i32 {
    let mut sort_numbers = numbers.into_iter().map(|x| *x).collect::<Vec<_>>();
    let mut all_arrs = <HashSet<Vec<i32>>>::new();
    all_arrs.insert(vec![0]);
    sort_numbers.insert(0, 0);
    sort_numbers.sort();

    for n in 0..sort_numbers.len() {

        if n+1 == sort_numbers.len() {

            let mut new_arrs = <HashSet<Vec<i32>>>::new();
            for all_arr in &all_arrs {
                

                let mut new_arrang = all_arr.clone();
                
                if *all_arr.last().unwrap() != sort_numbers[n] {
                    new_arrang.push(sort_numbers[n]);
                    new_arrs.insert(new_arrang);
                } else {
                    new_arrs.insert(new_arrang);
                }
                
            }
            all_arrs = new_arrs;
            break;
        }


        if sort_numbers[n]+2 >= sort_numbers[n+1] {
        
            if n+2 != sort_numbers.len() {
                
                if sort_numbers[n]+3 >= sort_numbers[n+2] {
                    
                    let mut new_arrs = <HashSet<Vec<i32>>>::new();
                    for all_arr in &all_arrs {
                         let last_num = all_arr.last().unwrap();
                         if sort_numbers[n+2] - last_num <= 3 {
                            let mut new_arrang = all_arr.clone();

                            if *all_arr.last().unwrap() == sort_numbers[n+2] {
                                new_arrs.insert(new_arrang.clone());
                                continue;
                            }

                            if *all_arr.last().unwrap() < sort_numbers[n+2] {
                                new_arrang.push(sort_numbers[n+2]);
                                new_arrs.insert(new_arrang.clone());
                            }

                            
                            if *all_arr.last().unwrap() < sort_numbers[n+1] {
                                new_arrang.pop();
                                new_arrang.push(sort_numbers[n+1]);
                                new_arrs.insert(new_arrang.clone());
                            }
                            
                            if n+3 != sort_numbers.len() && sort_numbers[n]+3 == sort_numbers[n+3] {
                                new_arrang.pop();
                                new_arrang.push(sort_numbers[n+3]);
                                new_arrs.insert(new_arrang.clone());
                            }

                        } else {
                            let mut new_arrang = all_arr.clone();
                            if *all_arr.last().unwrap() < sort_numbers[n+1] {
                                new_arrang.push(sort_numbers[n+1]);
                                new_arrs.insert(new_arrang);
                            } else {
                                new_arrs.insert(new_arrang);
                            }
                            
                        }
                    } 
                    all_arrs = new_arrs;                      
                }   
            }
        } 
        let mut new_arrs = <HashSet<Vec<i32>>>::new();
        for all_arr in &all_arrs {
            let mut new_arrang = all_arr.clone();
            if *all_arr.last().unwrap() < sort_numbers[n+1] {
                new_arrang.push(sort_numbers[n+1]);
                new_arrs.insert(new_arrang);
            } else {
                new_arrs.insert(new_arrang);
            }
            
        }
            all_arrs = new_arrs;
    }
    
    all_arrs.len() as i32

}

use std::collections::HashMap;  


#[allow(dead_code)]
pub fn adapter_array3(numbers : &[i32]) -> u64 {
    let mut sort_numbers = numbers.into_iter().map(|x| *x).collect::<Vec<_>>();
    let mut all_arrs = <HashMap<i32,u64>>::new();
    all_arrs.insert(0, 1);
    sort_numbers.sort();
    sort_numbers.push(sort_numbers[sort_numbers.len()-1]+3);
    

    for  n in 0..sort_numbers.len() {
        let mut arrs_map = <HashMap<i32,u64>>::new();
        for arr in &all_arrs {

            if *arr.0 >= sort_numbers[n] {
                continue;
            }

            if arr.0 + 2 >= sort_numbers[n] && !arrs_map.contains_key(&arr.0){
                arrs_map.insert(*arr.0, *arr.1);
            }

            if arr.0 + 3 >= sort_numbers[n] {
                if arrs_map.contains_key(&sort_numbers[n]) {
                    let val = arrs_map[&sort_numbers[n]];
                    arrs_map.remove(&sort_numbers[n]);
                    arrs_map.insert(sort_numbers[n], val + arr.1);
                } else {
                    arrs_map.insert(sort_numbers[n], *arr.1);
                }
            }
        }
        //println!("{} {:?}", sort_numbers[n], arrs_map);    
        all_arrs = arrs_map; 
    }
    
    let mut sum = 0;
    for (_last_num, count) in &all_arrs {
        sum+=*count;
    }
    sum 
}



#[test]
fn test_adapter_array1() {
    assert_eq!(adapter_array3(&[16,10,15,5,1,11,7,19,6,12,4]), 8);
}

#[test]
fn test_adapter_array2() {
    assert_eq!(adapter_array3(&[28,33,18,42,31,14,46,20,48,47,24,23,49,45,19,38,39,11,1,32,25,35,8,17,7,9,4,2,34,10,3]), 19208);
}

#[test]
fn test_adapter_array3() {
    assert_eq!(adapter_array3(&[1]), 1);
}

#[test]
fn test_adapter_array4() {
    assert_eq!(adapter_array3(&[1,2]), 2);
}

#[test]
fn test_adapter_array5() {
    assert_eq!(adapter_array3(&[1,2,3]), 4);
}

#[test]
fn test_adapter_array6() {
    assert_eq!(adapter_array3(&[1,2,3,6]), 4);
}