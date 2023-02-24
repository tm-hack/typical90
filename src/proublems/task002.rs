use std::convert::TryInto;

use proconio::*;

pub fn main() {
    input! {
        n: usize,
    }

    let mut res: Vec<String> = Vec::new();
    for i in 0..2_i32.pow(n.try_into().unwrap()) {
        let parentheses_bainary = decimal_to_binary(i as u32, n);

        if is_valid_parentheses(&parentheses_bainary) {
            let parentheses = binary_to_parentheses(parentheses_bainary);
            res.push(parentheses);
        }
    }

    for a in res {
        println!("{}", a);
    }
}

fn decimal_to_binary(decimal: u32, digits: usize) -> String {
    format!("{:0>1$b}", decimal, digits)
}

fn is_valid_parentheses(parentheses_binary: &String) -> bool {
    let cnt_0 = parentheses_binary.chars().filter(|&c| c == '0').count();
    let cnt_1 = parentheses_binary.chars().filter(|&c| c == '1').count();

    let mut res = true;
    if cnt_0 == cnt_1 {
        for i in 0..parentheses_binary.len() {
            let slice = parentheses_binary[..i].to_string();
            let cnt_0_i = slice.chars().filter(|&c| c == '0').count();
            let cnt_1_i = slice.chars().filter(|&c| c == '1').count();

            if cnt_0_i < cnt_1_i {
                res = false;
                break;
            }
        }
    } else {
        res = false;
    }

    return res;
}

fn binary_to_parentheses(parentheses_binary: String) -> String {
    return parentheses_binary.replace("0", "(").replace("1", ")");
}
