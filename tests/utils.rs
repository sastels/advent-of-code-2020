use advent_2020::utils::join_lines;
use advent_2020::utils::read_lines;

#[test]
fn test_utils_join_lines() {
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
