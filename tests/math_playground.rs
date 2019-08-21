// Copyright © 2019 Bader Alshaya
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use math_playground::*;

#[test]
fn test_valid_perfect_num_6() {
    assert_eq!(is_perfect_num(6), true);
}

#[test]
fn test_valid_perfect_num_28() {
    assert_eq!(is_perfect_num(28), true);
}

#[test]
fn test_valid_perfect_num_496() {
    assert_eq!(is_perfect_num(496), true);
}

#[test]
fn test_valid_perfect_num_8128() {
    assert_eq!(is_perfect_num(8128), true);
}

#[test]
fn test_invalid_perfect_num_0() {
    assert_eq!(is_perfect_num(0), false);
}

#[test]
fn test_invalid_perfect_num_1() {
    assert_eq!(is_perfect_num(1), false);
}

#[test]
fn test_invalid_perfect_num_5() {
    assert_eq!(is_perfect_num(5), false);
}

#[test]
fn test_invalid_perfect_num_7() {
    assert_eq!(is_perfect_num(7), false);
}

#[test]
fn test_get_perfect_nums_in_range_0() {
    assert_eq!(get_perfect_nums(0).is_empty(), true);
}

#[test]
fn test_get_perfect_nums_in_range_1() {
    assert_eq!(get_perfect_nums(1).is_empty(), true);
}

#[test]
fn test_get_perfect_nums_in_range_5() {
    assert_eq!(get_perfect_nums(5).is_empty(), true);
}

#[test]
fn test_get_perfect_nums_in_range_6() {
    assert_eq!(get_perfect_nums(6).len(), 1);
    assert_eq!(get_perfect_nums(6)[0], 6);
}

#[test]
fn test_get_perfect_nums_in_range_28() {
    assert_eq!(get_perfect_nums(28).len(), 2);
    assert_eq!(get_perfect_nums(28)[1], 28);
}

#[test]
fn test_get_perfect_nums_in_range_10000() {
    assert_eq!(get_perfect_nums(10000).len(), 4);
    assert_eq!(get_perfect_nums(10000)[3], 8128);
}

#[test]
fn test_valid_superperfect_num_2() {
    assert_eq!(is_superperfect_num(2), true);
}

#[test]
fn test_valid_superperfect_num_64() {
    assert_eq!(is_superperfect_num(64), true);
}

#[test]
fn test_valid_superperfect_num_4096() {
    assert_eq!(is_superperfect_num(4096), true);
}

#[test]
fn test_invalid_superperfect_num_0() {
    assert_eq!(is_superperfect_num(0), false);
}

#[test]
fn test_invalid_superperfect_num_1() {
    assert_eq!(is_superperfect_num(1), false);
}

#[test]
fn test_invalid_superperfect_num_10000() {
    assert_eq!(is_superperfect_num(10000), false);
}

#[test]
fn test_get_superperfect_nums_in_range_0() {
    assert_eq!(get_superperfect_nums(0).is_empty(), true);
}

#[test]
fn test_get_superperfect_nums_in_range_1() {
    assert_eq!(get_superperfect_nums(1).is_empty(), true);
}

#[test]
fn test_get_superperfect_nums_in_range_2() {
    assert_eq!(get_superperfect_nums(2)[0], 2);
    assert_eq!(get_superperfect_nums(2).len(), 1);
}

#[test]
fn test_get_superperfect_nums_in_range_4() {
    assert_eq!(get_superperfect_nums(4)[0], 2);
    assert_eq!(get_superperfect_nums(4)[1], 4);
    assert_eq!(get_superperfect_nums(4).len(), 2);
}

#[test]
fn test_get_superperfect_nums_in_range_64() {
    assert_eq!(get_superperfect_nums(64)[0], 2);
    assert_eq!(get_superperfect_nums(64)[1], 4);
    assert_eq!(get_superperfect_nums(64)[2], 16);
    assert_eq!(get_superperfect_nums(64)[3], 64);
    assert_eq!(get_superperfect_nums(64).len(), 4);
}
