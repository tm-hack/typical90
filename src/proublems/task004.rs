use itertools::Itertools;
use ndarray::*;
use proconio::*;

#[fastout]
pub fn main() {
    input! {
        h: usize,
        w: usize,
        aa: [[i32;h];w],
    }

    let aa: Array2<i32> =
        Array::from_shape_vec((h, w), aa.into_iter().flatten().collect()).unwrap();

    let sum_row = (0..h).map(|i| aa.row(i).sum()).collect_vec();
    let sum_col = (0..w).map(|j| aa.column(j).sum()).collect_vec();

    for i in 0..h {
        let b = (0..w)
            .map(|j| sum_row[i] + sum_col[j] - aa[[i, j]])
            .join(" ");

        println!("{}", b);
    }
}
