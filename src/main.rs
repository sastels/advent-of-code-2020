use advent_2020::day2;
use advent_2020::utils::read_lines;

fn main() {
    let data = read_lines("./data/day2.txt");
    day2::solve_a(&data);
    day2::solve_b(&data);
}
