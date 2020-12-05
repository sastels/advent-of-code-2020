#[cfg(test)]
use advent_2020::day5;
use advent_2020::day5::Ticket;
use advent_2020::utils::read_lines;

#[test]
fn test_day5_ticket_new() {
    assert_eq!(Ticket::new("BFFFBBFRRR").id, 567);
    assert_eq!(Ticket::new("FFFBBBFRRR").id, 119);
    assert_eq!(Ticket::new("BBFFBBFRLL").id, 820);
}

#[test]
fn test_day5_solve_a() {
    let data = read_lines("./data/day5.txt");
    assert_eq!(day5::solve_a(&data), 928);
}

#[test]
fn test_day5_solve_b() {
    let data = read_lines("./data/day5.txt");
    assert_eq!(day5::solve_b(&data), 610);
}
