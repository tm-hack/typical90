use proconio::*;

pub fn main() {
    input! {
        n: usize,
        mut aa: [i64; n],
        mut bb:  [i64; n],
    }

    aa.sort();
    bb.sort();

    let mut kai = 0;
    for i in 0..n {
        kai = kai + (aa[i] - bb[i]).abs()
    }

    println!("{}", kai);
}
