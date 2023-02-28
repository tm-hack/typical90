use num::integer;
use proconio::*;

pub fn main() {
    input! {
        (a, b, c): (i128,i128,i128),
    };

    let gcd_abc = integer::gcd(integer::gcd(a, b), c);
    let res = (a + b + c) / gcd_abc - 3;
    println!("{}", res);
}
