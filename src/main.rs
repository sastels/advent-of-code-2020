use advent_2020::day1;
use advent_2020::day2;
use advent_2020::day3;
use advent_2020::day4;
use advent_2020::day5;
use advent_2020::utils::read_lines;

fn main() {
    let data = read_lines("./data/day1.txt");
    println!("Day 1 A: {}", day1::solve_a(&data));
    println!("Day 1 B: {}", day1::solve_b(&data));

    let data = read_lines("./data/day2.txt");
    println!("Day 2 A: {}", day2::solve_a(&data));
    println!("Day 2 B: {}", day2::solve_b(&data));

    let data = read_lines("./data/day3.txt");
    println!("Day 3 A: {}", day3::solve_a(&data));
    println!("Day 3 B: {}", day3::solve_b(&data));

    let data = read_lines("./data/day4.txt");
    println!("Day 4 A: {}", day4::solve_a(&data));
    println!("Day 4 B: {}", day4::solve_b(&data));

    let data = read_lines("./data/day5.txt");
    println!("Day 5 A: {}", day5::solve_a(&data));
    println!("Day 5 B: {}", day5::solve_b(&data));
}
