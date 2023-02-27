use proconio::*;
const MAX_I: i64 = 9999;

pub fn main() {
    input! {
        n: i64,
        (a,b,c): (i64,i64,i64)
    }

    let mut min_coins = MAX_I;
    for i in 0..MAX_I + 1 {
        for j in 0..(MAX_I + 1 - i) {
            let balance = n - a * i - b * j;
            let n_flag = if balance >= 0 && balance % c == 0 {
                true
            } else {
                false
            };

            if n_flag {
                let coins = i + j + balance / c;
                if coins < min_coins {
                    min_coins = coins;
                }
            }
        }
    }

    println!("{}", min_coins);
}
