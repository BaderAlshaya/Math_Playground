

//
pub fn is_perfect_num(input: i64) -> bool {
    let mut total = 0;

    for i in 1..input {
        if (input%i) == 0 {
            total += i;
        }
    }

    input != 0 && total == input
}

//
pub fn get_perfect_nums(upper_bound: i64) -> Vec<i64> {
    let mut result = Vec::<i64>::new();

    for i in 0..upper_bound+1 {
        if is_perfect_num(i) {
            result.push(i);
        }
    }

    result
}
