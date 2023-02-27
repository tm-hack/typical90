use proconio::*;

pub fn main() {
    input! {
        n: usize,
        mut cp : [(i32,i32); n],
        q: usize,
        lr:  [(usize, usize); q],
    }

    cp.insert(0, (0, 0));

    let mut sum_1: Vec<i32> = vec![0; n + 1];
    let mut sum_2: Vec<i32> = vec![0; n + 1];

    for i in 1..(n + 1) {
        sum_1[i] = sum_1[i - 1];
        sum_2[i] = sum_2[i - 1];

        if cp[i].0 == 1 {
            sum_1[i] = sum_1[i] + cp[i].1;
        } else {
            sum_2[i] = sum_2[i] + cp[i].1;
        }
    }

    for i in 0..q {
        let score_1 = sum_1[lr[i].1] - sum_1[lr[i].0 - 1];
        let score_2 = sum_2[lr[i].1] - sum_2[lr[i].0 - 1];

        println!("{} {}", score_1, score_2);
    }
}
