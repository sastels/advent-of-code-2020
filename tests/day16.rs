use advent_2020::day16::solve_a;
use advent_2020::utils::read_lines;

#[test]

fn a() {
    let data = read_lines("./data/day16_test.txt");
    assert_eq!(solve_a(&data), 71);
}

// no b test - didn't have data to test it
