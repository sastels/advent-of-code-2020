use advent_2020::day23::{solve_a, solve_b};

#[test]
fn a() {
    assert_eq!(solve_a("389125467"), "67384529");
}

#[test]
#[ignore] // takes 4 min
fn b() {
    assert_eq!(solve_b("389125467"), 149245887792);
}
