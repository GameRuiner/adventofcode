#[allow(dead_code)]
pub fn toboggan_trajectory (road_map: &[String]) -> u64 {
    slope(road_map, 3, 1)
}

#[allow(dead_code)]
pub fn toboggan_trajectory2 (road_map: &[String]) -> u64 {
    slope(road_map, 3, 1) * 
    slope(road_map, 1, 1) * 
    slope(road_map, 5, 1) * 
    slope(road_map, 7, 1) *
    slope(road_map, 1, 2) 
}

fn slope(road_map: &[String], step: usize, step_down: i32) -> u64{
    let mut x_s = 0;
    let mut x = 0;
    let mut i = -1;
    for line in road_map {
        i +=1;
        if i % step_down != 0 {
            continue;
        }
        if line.chars().nth(x).unwrap() == '#' {
            x_s += 1;
        }
        x+=step;
        if x >= line.len() {
            x -= line.len();
        }
    }
    x_s
}

#[test]
fn test1() {
    assert_eq!(toboggan_trajectory(&["..##.......".to_string(), 
                                     "#...#...#..".to_string(), 
                                     ".#....#..#.".to_string(),
                                     "..#.#...#.#".to_string(),
                                     ".#...##..#.".to_string(),
                                     "..#.##.....".to_string(),
                                     ".#.#.#....#".to_string(),
                                     ".#........#".to_string(),
                                     "#.##...#...".to_string(),
                                     "#...##....#".to_string(),
                                     ".#..#...#.#".to_string(),]), 7);
}

#[test]
fn test2() {
    assert_eq!(toboggan_trajectory2(&["..##.......".to_string(), 
                                     "#...#...#..".to_string(), 
                                     ".#....#..#.".to_string(),
                                     "..#.#...#.#".to_string(),
                                     ".#...##..#.".to_string(),
                                     "..#.##.....".to_string(),
                                     ".#.#.#....#".to_string(),
                                     ".#........#".to_string(),
                                     "#.##...#...".to_string(),
                                     "#...##....#".to_string(),
                                     ".#..#...#.#".to_string(),]), 336);
}