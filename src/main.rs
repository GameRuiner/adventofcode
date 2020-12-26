use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod day1;
mod day2;
mod day3;
//mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;

fn main() {
    let mut input = vec![];
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                //input.push( ip.parse::<i32>().unwrap());
                input.push(ip);
            }
        }
    }
    println!("{}", day16::ticket_translation(&input[..]));
    //println!("{}", day15::rambunctious_recitation(&[0,20,7,16,1,18,15], 30000000));
    //println!("{}", day14::docking_data2(&input[..]));
    //println!("{}", day13::shuttles_search3(&input[..]));
    //println!("{}", day12::rain_risk2(&input[..]));
    //println!("{}", day1::report_repair(&input[..]));
    //println!("{}", day1::report_repair2(&input[..]));
    //println!("{}", day2::password_philosophy(&input[..]));
    //println!("{}", day2::password_philosophy2(&input[..]));
    //println!("{}", day3::toboggan_trajectory(&input[..]));
    //println!("{}", day3::toboggan_trajectory2(&input[..]));
    //println!("{}", day4::passport_processing(&input[..]));
    //println!("{}", day4::passport_processing2(&input[..]));
    //println!("{}", day5::binary_boarding(&input[..]));
    //println!("{}", day6::custom_customs(&input[..]));
    //println!("{}", day7::handy_haversacks2(&input[..]));
    //println!("{}", day8::handheld_halting(&input[..]));
    //println!("{}", day8::handheld_halting2(&input[..]));
    //println!("{}", day9::encoding_error(&input[..], 25));
    //println!("{}", day10::adapter_array3(&input[..]));
    //println!("{}", day11::seating_system(&input[..]));
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}