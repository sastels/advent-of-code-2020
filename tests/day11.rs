#[cfg(test)]
use advent_2020::day11::{self, SeatStatus, Seating};
use advent_2020::utils::{join_lines, read_lines};

#[test]
fn test_day11_seating_new() {
    let data = read_lines("./data/day11_test.txt");
    let data = &join_lines(&data)[0].replace(" ", "");
    let seating = Seating::new(data, 10);
    assert_eq!(seating.num_rows, 10);
    assert_eq!(seating.plan[0], SeatStatus::Empty);
    assert_eq!(seating.plan[4], SeatStatus::Floor);
}

#[test]
fn test_day11_seating_num_occupied() {
    let data = read_lines("./data/day11_test.txt");
    let data = &join_lines(&data)[0].replace(" ", "");
    let mut seating = Seating::new(data, 10);
    seating.plan[3] = SeatStatus::Occupied;
    assert_eq!(seating.num_occupied(), 1);
    seating.plan[12] = SeatStatus::Occupied;
    seating.plan[31] = SeatStatus::Occupied;
    assert_eq!(seating.num_occupied(), 3);
}

#[test]
fn test_day11_seating_num_occupied_neighbours() {
    let data = read_lines("./data/day11_test.txt");
    let data = &join_lines(&data)[0].replace(" ", "");
    let mut seating = Seating::new(data, 10);

    seating.plan[0] = SeatStatus::Occupied;
    seating.plan[2] = SeatStatus::Occupied;
    seating.plan[10] = SeatStatus::Occupied;
    assert_eq!(seating.num_occupied_neighbours(0, 1), 3);
    assert_eq!(seating.num_occupied_neighbours(0, 3), 1);
    assert_eq!(seating.num_occupied_neighbours(1, 1), 3);
    assert_eq!(seating.num_occupied_neighbours(2, 0), 1);
}

#[test]
#[ignore]
fn test_day11_solve_a() {
    let data = read_lines("./data/day11_test.txt");
    assert_eq!(day11::solve_a(&data), 666);
}

#[test]
#[ignore]
fn test_day11_solve_b() {
    let data = read_lines("./data/day11_test.txt");
    assert_eq!(day11::solve_b(&data), 666);
}
