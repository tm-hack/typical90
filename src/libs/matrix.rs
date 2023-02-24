use itertools::Itertools;
use ndarray::*;
use proconio::*;

pub fn main() {
    input! {
        h: usize,
        w: usize,
        aa: [[i32;h];w],
    }

    // VecからArrayへの変換
    let aa: Array2<i32> =
        Array::from_shape_vec((h, w), aa.into_iter().flatten().collect()).unwrap();

    // 行へのアクセス
    for i in 0..h {
        let row = aa.row(i);
        println!("{}", row);
    }

    // 列へのアクセス
    for j in 0..w {
        let column = aa.column(j);
        println!("{}", column);
    }

    // 要素へのアクセス
    println!("{}", aa[[0, 0]]);

    // 各行、各列の合計値を算出する
    let sum_row = (0..h).map(|i| aa.row(i).sum()).collect_vec();
    let sum_col = (0..w).map(|j| aa.column(j).sum()).collect_vec();

    // 同じ行または同じ列にあるマスの合計値を求める
    for i in 0..h {
        let b = (0..w)
            .map(|j| sum_row[i] + sum_col[j] - aa[[i, j]])
            .join(" ");

        println!("{}", b);
    }
}
