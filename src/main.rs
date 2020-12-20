use advent_2020::day1;
use advent_2020::day10;
use advent_2020::day11;
use advent_2020::day12;
use advent_2020::day13;
use advent_2020::day14;
use advent_2020::day15;
use advent_2020::day16;
use advent_2020::day17;
use advent_2020::day18;
use advent_2020::day19;
use advent_2020::day2;
use advent_2020::day20;
use advent_2020::day21;
use advent_2020::day22;
use advent_2020::day23;
use advent_2020::day24;
use advent_2020::day3;
use advent_2020::day4;
use advent_2020::day5;
use advent_2020::day6;
use advent_2020::day7;
use advent_2020::day8;
use advent_2020::day9;
use advent_2020::utils::read_lines;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("usage: cargo run n")
    }
    let day = &args[1];
    match day.as_str() {
        "1" => {
            let data = read_lines("./data/day1.txt");
            println!("Day 1 A: {}", day1::solve_a(&data));
            println!("Day 1 B: {}", day1::solve_b(&data));
        }
        "2" => {
            let data = read_lines("./data/day2.txt");
            println!("Day 2 A: {}", day2::solve_a(&data));
            println!("Day 2 B: {}", day2::solve_b(&data));
        }
        "3" => {
            let data = read_lines("./data/day3.txt");
            println!("Day 3 A: {}", day3::solve_a(&data));
            println!("Day 3 B: {}", day3::solve_b(&data));
        }

        "4" => {
            let data = read_lines("./data/day4.txt");
            println!("Day 4 A: {}", day4::solve_a(&data));
            println!("Day 4 B: {}", day4::solve_b(&data));
        }

        "5" => {
            let data = read_lines("./data/day5.txt");
            println!("Day 5 A: {}", day5::solve_a(&data));
            println!("Day 5 B: {}", day5::solve_b(&data));
        }
        "6" => {
            let data = read_lines("./data/day6.txt");
            println!("Day 6 A: {}", day6::solve_a(&data));
            println!("Day 6 B: {}", day6::solve_b(&data));
        }
        "7" => {
            let data = read_lines("./data/day7.txt");
            println!("Day 7 A: {}", day7::solve_a(&data));
            println!("Day 7 B: {}", day7::solve_b(&data));
        }
        "8" => {
            let data = read_lines("./data/day8.txt");
            println!("Day 8 A: {}", day8::solve_a(&data));
            println!("Day 8 B: {}", day8::solve_b(&data));
        }
        "9" => {
            let data = read_lines("./data/day9.txt");
            println!("Day 9 A: {}", day9::solve_a(&data, 25));
            println!("Day 9 B: {}", day9::solve_b(&data, 466456641));
        }
        "10" => {
            let data = read_lines("./data/day10.txt");
            println!("Day 10 A: {}", day10::solve_a(&data));
            println!("Day 10 B: {}", day10::solve_b(&data));
        }
        "11" => {
            let data = read_lines("./data/day11.txt");
            println!("Day 11 A: {}", day11::solve_a(&data));
            println!("Day 11 B: {}", day11::solve_b(&data));
        }
        "12" => {
            let data = read_lines("./data/day12.txt");
            println!("Day 12 A: {}", day12::solve_a(&data));
            println!("Day 12 B: {}", day12::solve_b(&data));
        }
        "13" => {
            let data = read_lines("./data/day13.txt");
            println!("Day 13 A: {}", day13::solve_a(&data));
            println!("Day 13 B: {}", day13::solve_b(&data));
        }
        "14" => {
            let data = read_lines("./data/day14.txt");
            println!("Day 14 A: {}", day14::solve_a(&data));
            println!("Day 14 B: {}", day14::solve_b(&data));
        }
        "15" => {
            let data = read_lines("./data/day15.txt");
            println!("Day 15 A: {}", day15::solve_a(&data));
            println!("Day 15 B: {}", day15::solve_b(&data));
        }
        "16" => {
            let data = read_lines("./data/day16.txt");
            println!("Day 16 A: {}", day16::solve_a(&data));
            println!("Day 16 B: {}", day16::solve_b(&data));
        }
        "17" => {
            let data = read_lines("./data/day17.txt");
            println!("Day 17 A: {}", day17::solve_a(&data));
            println!("Day 17 B: {}", day17::solve_b(&data));
        }
        "18" => {
            let data = read_lines("./data/day18.txt");
            println!("Day 18 A: {}", day18::solve_a(&data));
            println!("Day 18 B: {}", day18::solve_b(&data));
        }
        "19" => {
            let data = read_lines("./data/day19.txt");
            println!("Day 19 A: {}", day19::solve_a(&data));
            let data = read_lines("./data/day19_b.txt");
            println!("Day 19 B: {}", day19::solve_b(&data));
        }
        "20" => {
            let data = read_lines("./data/day20.txt");
            println!("Day 20 A: {}", day20::solve_a(&data));
            println!("Day 20 B: {}", day20::solve_b(&data));
        }
        "21" => {
            let data = read_lines("./data/day21.txt");
            println!("Day 20 A: {}", day21::solve_a(&data));
            println!("Day 20 B: {}", day21::solve_b(&data));
        }
        "22" => {
            let data = read_lines("./data/day22.txt");
            println!("Day 20 A: {}", day22::solve_a(&data));
            println!("Day 20 B: {}", day22::solve_b(&data));
        }
        "23" => {
            let data = read_lines("./data/day23.txt");
            println!("Day 20 A: {}", day23::solve_a(&data));
            println!("Day 20 B: {}", day23::solve_b(&data));
        }
        "24" => {
            let data = read_lines("./data/day24.txt");
            println!("Day 20 A: {}", day24::solve_a(&data));
            println!("Day 20 B: {}", day24::solve_b(&data));
        }
        _ => panic!("usage: cargo run n for n in 1..=24"),
    }
}
