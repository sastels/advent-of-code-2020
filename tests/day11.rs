#[cfg(test)]
use advent_2020::day11::{self, SeatStatus, Seating};
use advent_2020::utils::read_lines;

#[test]
fn test_day11_seating_new() {
    let data = read_lines("./data/day11_test.txt");
    let seating = Seating::new(&data);
    assert_eq!(seating.num_rows, 10);
    assert_eq!(seating.plan[0], SeatStatus::Empty);
    assert_eq!(seating.plan[4], SeatStatus::Floor);
}

#[test]
fn test_day11_seating_num_occupied() {
    let data = read_lines("./data/day11_test.txt");
    let mut seating = Seating::new(&data);
    seating.plan[3] = SeatStatus::Occupied;
    assert_eq!(seating.num_occupied(), 1);
    seating.plan[12] = SeatStatus::Occupied;
    seating.plan[31] = SeatStatus::Occupied;
    assert_eq!(seating.num_occupied(), 3);
}

#[test]
fn test_day11_seating_num_occupied_neighbours() {
    let data = read_lines("./data/day11_test.txt");
    let mut seating = Seating::new(&data);

    seating.plan[0] = SeatStatus::Occupied;
    seating.plan[2] = SeatStatus::Occupied;
    seating.plan[10] = SeatStatus::Occupied;
    assert_eq!(seating.num_occupied_neighbours(0, 1), 3);
    assert_eq!(seating.num_occupied_neighbours(0, 3), 1);
    assert_eq!(seating.num_occupied_neighbours(1, 1), 3);
    assert_eq!(seating.num_occupied_neighbours(2, 0), 1);
}

#[test]
fn test_day11_seating_step() {
    let data = read_lines("./data/day11_test.txt");
    let mut seating = Seating::new(&data);

    assert!(seating.step());
    assert!(seating.step());
    assert!(seating.step());
    assert!(seating.step());
    assert!(seating.step());
    assert!(!seating.step()); // finally nothing changes
    assert_eq!(seating.num_occupied(), 37);
}

#[test]
#[ignore]
fn test_day11_seating_num_occupied_visible_neighbours() {
    let data = read_lines("./data/day11_test_2.txt");
    let seating = Seating::new(&data);
    assert_eq!(seating.num_occupied_visible_neighbours(3, 3), 0)
}
#[test]
fn test_day11_solve_a() {
    let data = read_lines("./data/day11_test.txt");
    assert_eq!(day11::solve_a(&data), 37);
}

#[test]
#[ignore]
fn test_day11_solve_b() {
    let data = read_lines("./data/day11_test.txt");
    assert_eq!(day11::solve_b(&data), 666);
}
