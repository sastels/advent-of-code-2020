use advent_2020::day2::Password;

#[test]
fn test_password_new() {
    let actual = Password::new("6-10 p: ctpppjmdpppppp");
    assert_eq!(actual.letter, 'p');
    assert_eq!(actual.min, 6);
    assert_eq!(actual.max, 10);
    assert_eq!(actual.password, String::from("ctpppjmdpppppp"));
}

#[test]
fn test_password_passes_a_valid() {
    assert!(Password::new("6-10 p: ctpppjmdpppppp").passes_a());
}

#[test]
fn test_password_passes_a_invalid_1() {
    assert!(!Password::new("6-10 z: ctpppjmdpppppp").passes_a());
}

#[test]
fn test_password_passes_a_invalid_2() {
    assert!(!Password::new("1-2 p: ctpppjmdpppppp").passes_a());
}

#[test]
fn test_password_passes_a_invalid_3() {
    assert!(!Password::new("60-100 p: ctpppjmdpppppp").passes_a());
}

#[test]
fn test_password_passes_b_valid() {
    assert!(Password::new("1-3 a: abcde").passes_b());
}

#[test]
fn test_password_passes_b_invalid_1() {
    assert!(!Password::new("1-3 b: cdefg").passes_b());
}

#[test]
fn test_password_passes_b_invalid_2() {
    assert!(!Password::new("2-9 c: ccccccccc").passes_b());
}
