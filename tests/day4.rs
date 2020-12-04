#[cfg(test)]
use advent_2020::day4;
use advent_2020::day4::Passport;
use advent_2020::utils::read_lines;

#[test]
fn test_day4_passport_new() {
    let passport = Passport::new(
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm",
    );
    assert_eq!(passport.fields.get("pid").unwrap(), "860033327")
}

#[test]
fn test_day4_passport_valid() {
    let s1 = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm";
    let s2 = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 hgt:183cm";
    let s3 = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 hgt:183cm sja:hi";
    assert!(Passport::new(s1).is_valid());
    assert!(Passport::new(s2).is_valid());
    assert!(Passport::new(s3).is_valid());
}

#[test]
fn test_day4_passport_not_valid() {
    let s1 = "ecl:gry  eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm";
    let s2 = "ecl:gry pid:860033327 byr:1937 iyr:2017 cid:147 hgt:183cm";
    let s3 = "ecl:gry pid:860033327 eyt:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm";
    assert!(!Passport::new(s1).is_valid());
    assert!(!Passport::new(s2).is_valid());
    assert!(!Passport::new(s3).is_valid());
}

#[test]
fn test_day4_solve_a() {
    let data = read_lines("./data/day4_test.txt");
    assert_eq!(day4::solve_a(&data), 2);
}

#[test]
#[ignore]
fn test_day4_solve_b() {
    let data = read_lines("./data/day4_test.txt");
    assert_eq!(day4::solve_b(&data), 666);
}
