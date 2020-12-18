use advent_2020::day18::{calc, solve_a, solve_b};
use advent_2020::utils::read_lines;

#[test]
fn test_calc_b() {
    // assert_eq!(calc("2 + 3"), 5);
    // assert_eq!(calc("4 * 5"), 20);
    // assert_eq!(calc("2 * 3 + 20"), 46);

    // assert_eq!(calc("2 * 3 + (4 * 5)"), 46);
    assert_eq!(calc("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
    assert_eq!(calc("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
    assert_eq!(
        calc("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
        23340
    );
}

#[test]
#[ignore]
fn a() {
    let data = read_lines("./data/day13_test.txt");
    assert_eq!(solve_a(&data), 0);
}

#[test]
#[ignore]
fn b() {
    let data = read_lines("./data/day13_test.txt");
    assert_eq!(solve_b(&data), 0);
}
