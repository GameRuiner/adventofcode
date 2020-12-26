#[allow(dead_code)]
pub fn report_repair(report: &[i32]) -> i32 {
    let mut res = 0;
    for x in report {
        for y in report {
            if *x+*y == 2020 {
                res = (*x)*(*y);
            } else {
                continue;
            }
        }
    };
    res
    
}

#[allow(dead_code)]
pub fn report_repair2(report: &[i32]) -> i32 {
    let mut res = 0;
    for x in report {
        for y in report {
            for z in report {
            if *x+*y+*z == 2020 {
                res = (*x)*(*y)*(z);
            } else {
                continue;
            }
            }
        }
    };
    res
    
}


#[test]
fn test1() {
    assert_eq!(report_repair(&[1721, 979, 366, 299, 675, 1456]), 514579);
}

#[test]
fn test2() {
    assert_eq!(report_repair2(&[1721, 979, 366, 299, 675, 1456]), 241861950);
}