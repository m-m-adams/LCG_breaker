

use lcg_breaker::lcg::LinearCongruentialGenerator;
use lcg_breaker::math::find_unknown_params;

fn main() {

    // these are the specific parameters in use in glibc
    let lgc = LinearCongruentialGenerator::new(1103515245, 12345, 2147483648);
    let states: Vec<i128> = lgc.take(100).collect();
    println!("states: {:?}", states);
    println!("{:?}", find_unknown_params(&states));
}