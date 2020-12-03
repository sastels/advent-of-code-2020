use advent_2020::day1;
use advent_2020::day2;
use advent_2020::day3;
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
}
