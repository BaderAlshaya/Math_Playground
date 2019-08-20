
use math_playground::*;

#[test]
fn test_valid_perfect_nums() {
    assert_eq!(is_perfect_num(6), true);
    assert_eq!(is_perfect_num(28), true);
    assert_eq!(is_perfect_num(496), true);
    assert_eq!(is_perfect_num(8128), true);
}

#[test]
fn test_invalid_perfect_nums() {
    assert_eq!(is_perfect_num(0), false);
    assert_eq!(is_perfect_num(1), false);
    assert_eq!(is_perfect_num(2), false);
    assert_eq!(is_perfect_num(3), false);
    assert_eq!(is_perfect_num(4), false);
}

#[test]
fn test_valid_get_perfect_nums() {
    assert_eq!(get_perfect_nums(0).is_empty(), true);
    assert_eq!(get_perfect_nums(1).is_empty(), true);
    assert_eq!(get_perfect_nums(6).is_empty(), false);
    assert_eq!(get_perfect_nums(6)[0], 6);
    assert_eq!(get_perfect_nums(28)[1], 28);
    assert_eq!(get_perfect_nums(10000)[3], 8128);
}
