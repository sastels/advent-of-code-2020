use advent_2020::day17::{solve_a, solve_b, Conway};
use advent_2020::utils::read_lines;

#[test]
fn test_num_neighbours_set() {
    let data = read_lines("./data/day17_test.txt");
    let conway = Conway::new(&data);
    println!("{}", conway);
    assert_eq!(conway.num_neighbours_set(0, 0, 0), 5);
    assert_eq!(conway.num_neighbours_set(0, -1, 0), 1);
    assert_eq!(conway.num_neighbours_set(1, 1, 0), 2);
}

#[test]
fn a() {
    let data = read_lines("./data/day17_test.txt");
    assert_eq!(solve_a(&data), 112);
}

#[test]
#[ignore]
fn b() {
    let data = read_lines("./data/day17_test.txt");
    assert_eq!(solve_b(&data), 0);
}
