use proconio::*;

pub fn main() {
    input! {
        n: usize,
        mut an : [i32; n],
        q: usize,
        bq: [i32; q],
    }

    an.sort();
    for b in bq {
        let closest_value = search_closest_value(b, &an);
        println!("{}", (closest_value - b).abs());
    }
}

fn search_closest_value(b: i32, aa: &Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = aa.len() - 1;

    loop {
        if left == right {
            return aa[right];
        }

        if (right - left) == 1 {
            if b - aa[left] < aa[right] - b {
                return aa[left];
            } else {
                return aa[right];
            }
        }

        if b >= aa[(left + right) / 2] {
            left = (left + right) / 2;
        } else {
            right = (left + right) / 2;
        }
    }
}
