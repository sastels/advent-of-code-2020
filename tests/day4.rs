#[cfg(test)]
use advent_2020::day4;
use advent_2020::day4::Passport;
use advent_2020::utils::read_lines;

#[test]
fn test_day4_passport_new() {
    let s = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm";
    assert_eq!(Passport::new(s).fields.get("pid").unwrap(), "860033327")
}

#[test]
fn test_day4_passport_all_present() {
    let s1 = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm";
    let s2 = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 hgt:183cm";
    let s3 = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 hgt:183cm sja:hi";
    assert!(Passport::new(s1).all_present());
    assert!(Passport::new(s2).all_present());
    assert!(Passport::new(s3).all_present());
}

#[test]
fn test_day4_passport_not_all_present() {
    let s1 = "ecl:gry eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm";
    let s2 = "ecl:gry pid:860033327 byr:1937 iyr:2017 cid:147 hgt:183cm";
    let s3 = "ecl:gry pid:860033327 eyt:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm";
    assert!(!Passport::new(s1).all_present());
    assert!(!Passport::new(s2).all_present());
    assert!(!Passport::new(s3).all_present());
}

#[test]
fn test_day4_passport_valid_key() {
    assert!(Passport::new("byr:2002").valid_key("byr"));
    assert!(Passport::new("hgt:190cm").valid_key("hgt"));
    assert!(Passport::new("hgt:60in").valid_key("hgt"));
    assert!(Passport::new("hcl:#123abc").valid_key("hcl"));
    assert!(Passport::new("ecl:brn").valid_key("ecl"));
    assert!(Passport::new("pid:000000001").valid_key("pid"));
}

#[test]
fn test_day4_passport_not_valid_key() {
    assert!(!Passport::new("byr:2003").valid_key("byr"));
    assert!(!Passport::new("hgt:190in").valid_key("hgt"));
    assert!(!Passport::new("hgt:190").valid_key("hgt"));
    assert!(!Passport::new("hcl:#123abz").valid_key("hcl"));
    assert!(!Passport::new("hcl:123abc").valid_key("hcl"));
    assert!(!Passport::new("hcl:#123").valid_key("hcl"));
    assert!(!Passport::new("ecl:wat").valid_key("ecl"));
    assert!(!Passport::new("pid:0123456789").valid_key("pid"));
}

#[test]
fn test_day4_solve_a() {
    let data = read_lines("./data/day4_test_a.txt");
    assert_eq!(day4::solve_a(&data), 2);
}

#[test]
fn test_day4_solve_b() {
    let data = read_lines("./data/day4_test_b.txt");
    assert_eq!(day4::solve_b(&data), 4);
}
