use advent_2020::utils::{join_lines, read_lines, Grid};

#[test]
fn utils_join_lines() {
    let data = read_lines("./data/day4_test_a.txt");
    let joined = join_lines(&data);
    assert_eq!(joined.len(), 4);
    assert_eq!(
        joined[0],
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm"
    );
    assert_eq!(
        joined[3],
        "hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in"
    )
}

#[test]
fn get_set_grid() {
    let mut grid = Grid::<i32>::new(10, -1);
    assert_eq!(grid.count_equals(-1), 100);
    assert_eq!(grid.count_equals(15), 0);
    assert_eq!(*grid.get(2, 1), -1);
    grid.set(2, 1, 15);
    assert_eq!(grid.count_equals(15), 1);

    assert_eq!(*grid.get(2, 1), 15);
}
