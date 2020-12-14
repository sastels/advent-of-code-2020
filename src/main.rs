use advent_2020::day1;
// use advent_2020::day10;
// use advent_2020::day11;
// use advent_2020::day12;
use advent_2020::day13;
use advent_2020::day14;
use advent_2020::day2;
use advent_2020::day3;
// use advent_2020::day4;
// use advent_2020::day5;
// use advent_2020::day6;
// use advent_2020::day7;
// use advent_2020::day8;
// use advent_2020::day9;
use advent_2020::utils::read_lines;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    match day.as_str() {
        "1" => {
            let data = read_lines("./data/day1.txt");
            println!("Day 1 A: {}", day1::solve_a(&data));
            println!("Day 1 B: {}", day1::solve_b(&data));
        }
        "2" => {
            let data = read_lines("./data/day1.txt");
            println!("Day 2 A: {}", day2::solve_a(&data));
            println!("Day 2 B: {}", day2::solve_b(&data));
        }
        "3" => {
            let data = read_lines("./data/day1.txt");
            println!("Day 3 A: {}", day3::solve_a(&data));
            println!("Day 3 B: {}", day3::solve_b(&data));
        }
        "14" => {
            let data = read_lines("./data/day14.txt");
            println!("Day 14 A: {}", day14::solve_a(&data));
            println!("Day 14 B: {}", day14::solve_b(&data));
        }

        _ => panic!(),
    }

    // let data = read_lines("./data/day2.txt");
    // println!("Day 2 A: {}", day2::solve_a(&data));
    // println!("Day 2 B: {}", day2::solve_b(&data));

    // let data = read_lines("./data/day3.txt");
    // println!("Day 3 A: {}", day3::solve_a(&data));
    // println!("Day 3 B: {}", day3::solve_b(&data));

    // let data = read_lines("./data/day4.txt");
    // println!("Day 4 A: {}", day4::solve_a(&data));
    // println!("Day 4 B: {}", day4::solve_b(&data));

    // let data = read_lines("./data/day5.txt");
    // println!("Day 5 A: {}", day5::solve_a(&data));
    // println!("Day 5 B: {}", day5::solve_b(&data));

    // let data = read_lines("./data/day6.txt");
    // println!("Day 6 A: {}", day6::solve_a(&data));
    // println!("Day 6 B: {}", day6::solve_b(&data));

    // let data = read_lines("./data/day7.txt");
    // println!("Day 7 A: {}", day7::solve_a(&data));
    // println!("Day 7 B: {}", day7::solve_b(&data));

    // let data = read_lines("./data/day8.txt");
    // println!("Day 8 A: {}", day8::solve_a(&data));
    // println!("Day 8 B: {}", day8::solve_b(&data));

    // let data = read_lines("./data/day9.txt");
    // println!("Day 9 A: {}", day9::solve_a(&data, 25));
    // println!("Day 9 B: {}", day9::solve_b(&data, 466456641));

    // let data = read_lines("./data/day10.txt");
    // println!("Day 10 A: {}", day10::solve_a(&data));
    // println!("Day 10 B: {}", day10::solve_b(&data));

    // let data = read_lines("./data/day11.txt");
    // println!("Day 11 A: {}", day11::solve_a(&data));
    // println!("Day 11 B: {}", day11::solve_b(&data));

    // let data = read_lines("./data/day12.txt");
    // println!("Day 12 A: {}", day12::solve_a(&data));
    // println!("Day 12 B: {}", day12::solve_b(&data));

    let data = read_lines("./data/day13.txt");
    println!("Day 13 A: {}", day13::solve_a(&data));
    println!("Day 13 B: {}", day13::solve_b(&data));

    let data = read_lines("./data/day13.txt");
    println!("Day 14 A: {}", day14::solve_a(&data));
    println!("Day 14 B: {}", day14::solve_b(&data));
}
