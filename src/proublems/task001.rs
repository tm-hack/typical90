use proconio::*;

pub fn main() {
    input! {
        n: i32,
        l: i32,
        k: i32,
        aa: [i32;n],
    }

    // 答えで2分木探索を行う
    let res;
    let mut left = 0;
    let mut right = l;

    loop {
        if is_split_score((right + left) / 2, k, &aa, l) {
            left = (right + left) / 2;
        } else {
            right = (right + left) / 2 - 1;
        }

        if right - left <= 1 {
            if is_split_score(right, k, &aa, l) {
                res = right;
            } else {
                res = left;
            }
            break;
        }
    }

    println!("{}", res);
}

fn is_split_score(score: i32, k: i32, aa: &Vec<i32>, length: i32) -> bool {
    let mut a_0 = 0;
    let mut a_1 = 0;

    // カット数がk回で長さがm以上のケーキにカットできるかを試行する
    let mut cut_cnt = 0;

    for a_i in aa {
        a_1 = a_i.clone();
        if a_1 - a_0 >= score {
            cut_cnt = cut_cnt + 1;
            a_0 = a_1;
        }

        if cut_cnt == k {
            break;
        }
    }

    // カット数がcut_cntに満たない場合はfalse
    if cut_cnt < k {
        return false;
    // カット数がcut_cntに達したとしても末尾の切れ端がscoreよりも小さければfalse
    } else if length - a_1 < score {
        return false;
    } else {
        return true;
    }
}
