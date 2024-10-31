// 49620     = (1 * multiplier + increment) % modulus
// Can be rewritten as:
// increment = (49620 - 1 * multiplier) % modulus
// increment = (49620 - 1 *       6329) % 4294967301


/// extended euclidean algorithm - finds GCD of a and b, and finds x and y s.t. ax+by = GCD(a,b)
/// returns (gcd, x, y)
fn euclidean_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, y, x) = euclidean_gcd(b % a, a);
        (g, x - (b / a) * y, y)
    }
}

fn gcd(a: i128, b: i128) -> i128 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a.rem_euclid(b))
    }
}
/// returns mod inverse found using euclidean algorithm
fn mod_inverse(a: i128, m: i128) -> Result<i128, &'static str> {
    let (g, x, _) = euclidean_gcd(a, m);

    if g != 1 {
        Err("mod inverse does not exist for 1")
    } else {
        // this is specifically the modulo - % gives the remainder which is wrong for negative inputs
        Ok(x.rem_euclid(m))
    }
}

fn find_unknown_increment(
    states: &[i128],
    multiplier: i128,
    modulus: i128
) -> i128 {
    let increment =
        (states[1] - states[0] * multiplier).rem_euclid(modulus);
    increment
}

fn find_unknown_multiplier(
    states: &[i128],
    modulus: i128
) -> i128 {
    let inverse_modulus =
        mod_inverse(states[1] - states[0], modulus).unwrap();
    let multiplier =
        (states[2] - states[1]) * inverse_modulus % modulus;
    multiplier
}

pub fn find_unknown_modulus(states: &[i128]) -> i128 {
    let offset_states = &states[1..];
    // Zip together the state lists, adjusted by one position.
    let zipped_states = states.iter().zip(offset_states.iter());
    // Calculate the difference of the states.
    let diffs: Vec<i128> =
        zipped_states.map(|(k0, k1)| k1 - k0).collect();
    // Build the matrix of the differences
    let offset_diffs_1 = diffs.iter().skip(1);
    let offset_diffs_2 = diffs.iter().skip(2);
    let zipped_diffs =
        diffs.iter().zip(offset_diffs_1).zip(offset_diffs_2);
    let zeroes = zipped_diffs.map(|((s0, s1), s2)| s2 * s0 - s1 * s1);
    // Find the greatest common divisor of the differences
    let modulus = zeroes.fold(0, |a, b| gcd(a, b));
    modulus
}
/// returns (multiplier, increment, modulus)
pub fn find_unknown_params(states: &[i128]) -> (i128, i128, i128) {

    let modulus = find_unknown_modulus(states);
    let mult = find_unknown_multiplier(states, modulus);
    let increment = find_unknown_increment(states, mult, modulus);
    (mult, increment, modulus)
}

#[cfg(test)]
mod tests {
    use crate::lcg::LinearCongruentialGenerator;
    use super::*;
    #[test]
    fn test_find_unknown_params() {
        let mult = 6329;
        let increment = 43291;
        let modulus = 4294967295;
        let lgc = LinearCongruentialGenerator::new(mult, increment, modulus);
        let states: Vec<i128> = lgc.take(10).collect();
        assert_eq!(find_unknown_params(&states), (mult, increment, modulus));
    }
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(27, 9), 9);
        assert_eq!(gcd(127, 128), 1);
    }
    #[test]
    fn test_mod_inverse() {
        assert_eq!(mod_inverse(3, 7).unwrap(), 5);
    }
}