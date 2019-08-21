// Copyright © 2019 Bader Alshaya
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

// Return the sum of all divisors for a given integer
pub fn sum_divisors(input: i64) -> i64 {
    let mut total = 0;
    for i in 1..input + 1 {
        if (input % i) == 0 {
            total += i;
        }
    }
    total
}

// Check whether a given integer is a perfect number
pub fn is_perfect_num(input: i64) -> bool {
    let total = sum_divisors(input) - input; // Exclude the number from divisors
    input != 0 && input == total
}

// Return a list of all the perfect numbers within a given range (0:upper_bound)
pub fn get_perfect_nums(upper_bound: i64) -> Vec<i64> {
    let mut result = Vec::<i64>::new();

    for i in 0..upper_bound + 1 {
        if is_perfect_num(i) {
            result.push(i);
        }
    }

    result
}

// Check whether a given integer is a superperfect number
pub fn is_superperfect_num(input: i64) -> bool {
    let mut total = sum_divisors(input);
    total = sum_divisors(total);
    total != 0 && total == (input * 2)
}

// Return a list of all the superperfect numbers within a given range (0:upper_bound)
pub fn get_superperfect_nums(upper_bound: i64) -> Vec<i64> {
    let mut result = Vec::<i64>::new();

    for i in 0..upper_bound + 1 {
        if is_superperfect_num(i) {
            result.push(i);
        }
    }

    result
}

// Return a list of prime numbers within a given range (0:upper_bound)
pub fn sieve_of_eratosthenes(upper_bound: i64) -> Vec<i64> {
    let mut primes = Vec::<i64>::new();

    for num in 2..upper_bound + 1 {
        let mut num_is_not_multiple = true;

        for p_num in &primes {
            if num % p_num == 0 {
                num_is_not_multiple = false;
                break;
            }
        }

        if num_is_not_multiple {
            primes.push(num);
        }
    }

    primes
}
